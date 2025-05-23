//! Contains app-related things (so just about everything)
pub mod helpers;
mod keystroke;

use crate::app::helpers::{load_color_image_from_path, load_texture_from_color_image};
use crate::app::keystroke::KeystrokeState;
use crate::consts::graphics::*;
use crate::consts::keyboard::*;
use crate::theme::{AppThemeImage, AppThemeTexture, ThemeSet};
use dashmap::DashMap;
use eframe::epaint::{Pos2, RectShape, TextShape, text::LayoutJob};
use egui::{Color32, Context, FontFamily, FontId, LayerId, Rect, Stroke, StrokeKind, TextureId};
use inputbot::KeybdKey;
use parking_lot::RwLock;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, OnceLock, atomic::Ordering};
use std::thread::{JoinHandle, Thread};
use std::{default::Default, thread};

/// The main application state
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct BongoApp {
    /// Theme (bongocat images) to be used
    #[serde(skip)]
    themes: Arc<RwLock<ThemeSet>>,
    /// Access to the eGui `Context` from other threads
    #[serde(skip)]
    context_access: OnceLock<Context>,
    /// `JoinHandle`s for threads spunup
    #[serde(skip)]
    handles: Arc<DashMap<usize, Option<JoinHandle<()>>>>,
    /// Notifies threads of exiting
    #[serde(skip)]
    exit_notify: Arc<AtomicBool>,
    /// Are they rendered yet?
    #[serde(skip)]
    themes_rendered: bool,
    /// Holds shapes we don't want to keep redrawing
    #[serde(skip)]
    shape_holder: Arc<RwLock<Vec<(&'static VisualKeybdKeyHolder, Rect)>>>,
    /// Keystroke-related state
    keystroke_state: Arc<KeystrokeState>,
}

impl Default for BongoApp {
    fn default() -> Self {
        Self {
            themes: Arc::default(),
            keystroke_state: Arc::default(),
            context_access: OnceLock::default(),
            handles: Arc::default(),
            exit_notify: Arc::default(),
            themes_rendered: false,
            shape_holder: Arc::default(),
        }
    }
}

impl BongoApp {
    pub fn log_key(&self, key: &KeybdKey) {
        self.keystroke_state.log_keystroke(key);
        self.context_access.wait().request_repaint();
    }

    /// Creates a new `BongoApp`
    /// # Panics
    /// When context instance is somehow deserialized
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let this: Self;

        egui_extras::install_image_loaders(&cc.egui_ctx);

        if let Some(storage) = cc.storage {
            this = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        } else {
            this = Self::default();
        }

        assert!(
            this.context_access.set(cc.egui_ctx.clone()).is_ok(),
            "Context instance was already set?"
        );

        let arc_this = Arc::new(Self {
            themes: this.themes.clone(),
            context_access: this.context_access.clone(),
            handles: this.handles.clone(),
            exit_notify: this.exit_notify.clone(),
            themes_rendered: false,
            shape_holder: this.shape_holder.clone(),
            keystroke_state: this.keystroke_state.clone(),
        });

        for key in &KEYS {
            for key in *key {
                let arc_clone = arc_this.clone();
                key.key.bind(move || {
                    Self::log_key(&arc_clone, &key.key);
                });
            }
        }

        arc_this.insert_handle_autoincrement(thread::spawn(|| {
            inputbot::handle_input_events(true);
        }));

        let arc_clone = arc_this.clone();
        let _ = this.keystroke_state.input_update_thread.lock().insert(
            arc_this.insert_handle_autoincrement(thread::spawn(move || {
                let context = arc_clone.context_access.wait();
                loop {
                    if arc_clone.exit_notify.load(Ordering::SeqCst) {
                        return;
                    }
                    thread::park_timeout(KEY_PRESSED_LIT_DELAY);
                    context.request_repaint();
                }
            })),
        );

        let arc_clone = arc_this.clone();
        arc_this.insert_handle_autoincrement(thread::spawn(move || {
            let state = arc_clone.keystroke_state.clone();
            loop {
                if arc_clone.exit_notify.load(Ordering::SeqCst) {
                    return;
                }
                thread::sleep(KEY_PRESSED_CLEANUP_DELAY);
                state.cleanup_outdated(KEY_PRESSED_LIT_DELAY);
            }
        }));

        let theme_clone = arc_this.themes.clone();
        let theme_ref = theme_clone.data_ptr() as *const ThemeSet;

        unsafe {
            for theme in &(*theme_ref).themes {
                let mut id: [u8; 16] = [0; 16];
                rand::fill(&mut id);
                arc_this.themes.write().themes_loaded.push(AppThemeImage {
                    app_theme: Some(&theme),
                    id: hex::encode(id),
                    paws_both: load_color_image_from_path(&theme.paws_both),
                    paws_left: load_color_image_from_path(&theme.paws_left),
                    paws_right: load_color_image_from_path(&theme.paws_right),
                    paws_up: load_color_image_from_path(&theme.paws_up),
                });
            }

            for theme in &(*theme_ref).themes_loaded {
                arc_this
                    .themes
                    .write()
                    .themes_rendered
                    .push(AppThemeTexture {
                        app_theme_image: &theme,
                        paws_both: None,
                        paws_left: None,
                        paws_right: None,
                        paws_up: None,
                    });
            }
        }

        let mut row_off = 0;
        for y in (0..KEYS.len()).rev() {
            let mut col_off = 0;
            for x in (0..KEYS[y].len()).rev() {
                let key = &KEYS[y][x];
                let size_offset = key.size * 10.0;

                arc_this.shape_holder.write().push((
                    key,
                    Rect::from_min_max(
                        Pos2::new(
                            (10 + col_off) as f32,
                            ((WINDOW_HEIGHT as u64 - 70) + row_off) as f32,
                        ),
                        Pos2::new(
                            10.0 + size_offset + col_off as f32,
                            ((WINDOW_HEIGHT as u64 - 70 + PADDING_PIXELS) + row_off) as f32,
                        ),
                    ),
                ));

                col_off += (PADDING_PIXELS as f32 * key.size) as u64;
            }
            row_off += PADDING_PIXELS;
        }

        this
    }

    pub fn insert_handle_autoincrement(&self, handle: JoinHandle<()>) -> Thread {
        let id = self.handles.len();
        let thread = handle.thread().clone();
        self.handles.insert(id, Some(handle));

        thread
    }
    pub fn insert_handle(&self, id: usize, handle: JoinHandle<()>) -> Thread {
        let thread = handle.thread().clone();
        self.handles.insert(id, Some(handle));

        thread
    }

    pub fn cleanup_handles(&self) {
        self.handles.retain(|_id, handle| {
            if let Some(handle_) = handle
                && !handle_.is_finished()
            {
                true
            } else {
                false
            }
        });
    }
}

impl eframe::App for BongoApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if !self.themes_rendered {
            let mut theme_guard = self.themes.write();
            for theme in theme_guard.themes_rendered.iter_mut() {
                let app_theme = &theme.app_theme_image;
                theme.paws_both.get_or_insert(load_texture_from_color_image(
                    ctx,
                    &app_theme.paws_both,
                    format!("paws_both_{}", &app_theme.id),
                ));
                theme.paws_left.get_or_insert(load_texture_from_color_image(
                    ctx,
                    &app_theme.paws_left,
                    format!("paws_left_{}", &app_theme.id),
                ));
                theme
                    .paws_right
                    .get_or_insert(load_texture_from_color_image(
                        ctx,
                        &app_theme.paws_right,
                        format!("paws_right_{}", &app_theme.id),
                    ));
                theme.paws_up.get_or_insert(load_texture_from_color_image(
                    ctx,
                    &app_theme.paws_up,
                    format!("paws_up_{}", &app_theme.id),
                ));
            }
            drop(theme_guard);
            self.themes_rendered = true;
        }

        let painter = ctx.layer_painter(LayerId::background());

        let mut left_side_down = false;
        let mut right_side_down = false;
        let mut is_o_face = false;

        for (key, rect) in self.shape_holder.read().iter() {
            let rect = rect.clone();

            let color = if let KeybdKey::OtherKey(code) = key.key {
                if code == u64::MAX - 1 {
                    Color32::TRANSPARENT
                } else {
                    Color32::WHITE
                }
            } else {
                Color32::WHITE
            };

            painter.add(RectShape::new(
                rect,
                0.5,
                match key.key {
                    KeybdKey::OtherKey(v) => {
                        if v < u64::MAX - 2
                            && self.keystroke_state.lit_keys_map[key.key].load(Ordering::SeqCst)
                        {
                            if key.column < 10 {
                                left_side_down = true;
                            } else {
                                right_side_down = true;
                            }
                            Color32::LIGHT_BLUE
                        } else {
                            Color32::TRANSPARENT
                        }
                    }
                    _ => {
                        if self.keystroke_state.lit_keys_map[key.key].load(Ordering::SeqCst) {
                            if key.column < 8 {
                                left_side_down = true;
                            } else {
                                right_side_down = true;
                            }
                            if !is_o_face {
                                is_o_face = matches!(
                                    key.key,
                                    KeybdKey::OKey | KeybdKey::Numrow0Key | KeybdKey::Numpad0Key
                                );
                            }
                            Color32::LIGHT_BLUE
                        } else {
                            Color32::TRANSPARENT
                        }
                    }
                },
                Stroke::new(1.0, color),
                StrokeKind::Middle,
            ));
        }

        painter.add(
            TextShape::new(
                Pos2::new(WINDOW_WIDTH / 20.0, WINDOW_HEIGHT / 2.0),
                painter.layout_job(LayoutJob::simple(
                    format!("{}", self.keystroke_state.keystrokes.load(Ordering::SeqCst)),
                    FontId::new(WINDOW_HEIGHT / 12.5, FontFamily::Proportional),
                    Color32::WHITE,
                    f32::MAX,
                )),
                Color32::PLACEHOLDER,
            )
            .with_angle(0.231_605_19),
        );

        let themes = self.themes.read();
        let theme = if is_o_face {
            themes
                .themes_rendered
                .get(1)
                .or_else(|| themes.themes_rendered.first())
        } else {
            themes.themes_rendered.first()
        };

        if let Some(theme) = theme {
            let id = if left_side_down
                && right_side_down
                && let Some(tex) = &theme.paws_both
            {
                tex.id()
            } else if left_side_down && let Some(tex) = &theme.paws_left {
                tex.id()
            } else if right_side_down && let Some(tex) = &theme.paws_right {
                tex.id()
            } else if let Some(tex) = &theme.paws_up {
                tex.id()
            } else {
                TextureId::default()
            };

            painter.image(id, WINDOW_RECT, UV_RECT, Color32::WHITE);
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.exit_notify.store(true, Ordering::SeqCst);
        inputbot::stop_handling_input_events();
        self.handles.alter_all(|_, h| {
            if let Some(handle) = h {
                handle.thread().unpark();
                handle.join().unwrap();
            }
            None
        });
        self.cleanup_handles();
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
}
