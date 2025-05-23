//! Contains app-related things (so just about everything)
pub mod helpers;
mod keystroke;

use crate::app::helpers::{load_color_image_from_path, load_texture_from_color_image};
use crate::app::keystroke::KeystrokeState;
use crate::consts::graphics::*;
use crate::consts::keyboard::*;
use crate::theme::{AppThemeImage, AppThemeTexture, ThemeSet};
use dashmap::DashMap;
use eframe::epaint::{
    Pos2, TextShape,
    text::{LayoutJob, TextFormat},
};
use egui::{
    Align, Color32, Context, FontFamily, FontId, LayerId, Rect, Stroke, TextureId, ViewportCommand,
    text::LayoutSection,
};
use inputbot::KeybdKey;
use std::rc::Rc;
use std::{
    cell::UnsafeCell,
    default::Default,
    ops::Deref,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle, Thread},
};

/// The main application state
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct BongoApp {
    /// Theme (bongocat images) to be used
    #[serde(skip)]
    themes: Arc<UnsafeCell<ThemeSet>>,
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
    shape_holder: Vec<(&'static VisualKeybdKeyHolder, Vec<Pos2>)>,
    /// Keystroke-related state
    keystroke_state: Arc<KeystrokeState>,
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
        let mut this: Self;

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

        cc.egui_ctx
            .send_viewport_cmd(ViewportCommand::MousePassthrough(true)); // Fix clickthrough for Windows :)

        let arc_this = Rc::new(Self {
            themes: this.themes.clone(),
            context_access: this.context_access.clone(),
            handles: this.handles.clone(),
            exit_notify: this.exit_notify.clone(),
            themes_rendered: false,
            shape_holder: this.shape_holder.clone(),
            keystroke_state: this.keystroke_state.clone(),
        });

        let arc_clone = arc_this.themes.clone();
        let theme_set = unsafe { arc_clone.deref().as_mut_unchecked() };

        for theme in &theme_set.themes {
            theme_set.themes_loaded.push(AppThemeImage {
                id: theme.id.clone(),
                paws_both: load_color_image_from_path(&theme.paws_both),
                paws_left: load_color_image_from_path(&theme.paws_left),
                paws_right: load_color_image_from_path(&theme.paws_right),
                paws_up: load_color_image_from_path(&theme.paws_up),
            });
        }

        for theme in &theme_set.themes_loaded {
            theme_set.themes_rendered.push(AppThemeTexture {
                id: theme.id.clone(),
                paws_both: None,
                paws_left: None,
                paws_right: None,
                paws_up: None,
            });
        }

        for key in &KEYS {
            for key in *key {
                let keystroke_state = arc_this.keystroke_state.clone();
                let context_access = arc_this.context_access.clone();
                let key_clone = key.key;
                key.key.bind(move || {
                    keystroke_state.log_keystroke(&key_clone);
                    context_access.wait().request_repaint();
                });
            }
        }

        arc_this.insert_handle_autoincrement(thread::spawn(|| {
            inputbot::handle_input_events(true);
        }));

        let arc_clone = arc_this.clone();
        let exit_notify = arc_clone.exit_notify.clone();
        let context = arc_clone.context_access.wait().clone();
        let _ = this.keystroke_state.input_update_thread.lock().insert(
            arc_this.insert_handle_autoincrement(thread::spawn(move || {
                loop {
                    if exit_notify.load(Ordering::Relaxed) {
                        return;
                    }
                    thread::park();
                    context.request_repaint();
                }
            })),
        );

        let arc_clone = arc_this.clone();
        let state = arc_clone.keystroke_state.clone();
        let exit_notify = arc_clone.exit_notify.clone();
        arc_this.insert_handle_autoincrement(thread::spawn(move || {
            loop {
                if exit_notify.load(Ordering::Relaxed) {
                    return;
                }
                thread::sleep(KEY_PRESSED_CLEANUP_DELAY);
                state.cleanup_outdated(KEY_PRESSED_LIT_DELAY);
            }
        }));

        let mut rects: Vec<(&VisualKeybdKeyHolder, Rect)> = vec![];

        fn rotate_point(center: Pos2, p: Pos2, theta: f32) -> Pos2 {
            let x = p.x - center.x;
            let y = p.y - center.y;
            let cos_theta = theta.cos();
            let sin_theta = theta.sin();
            Pos2::new(
                center.x + (x * cos_theta - y * sin_theta),
                center.y + (x * sin_theta + y * cos_theta),
            )
        }

        fn rotate_rect(rect: Rect, center: Pos2, theta: f32) -> [Pos2; 4] {
            let corners = [
                rect.left_top(),
                rect.right_top(),
                rect.right_bottom(),
                rect.left_bottom(),
            ];
            corners.map(|p| rotate_point(center, p, theta))
        }

        let mut row_off = 0;
        for y in (0..KEYS.len()).rev() {
            let mut col_off = 0;
            for x in (0..KEYS[y].len()).rev() {
                let key = &KEYS[y][x];
                let size_offset = key.size * 10.0;

                rects.push((
                    key,
                    Rect::from_min_max(
                        Pos2::new(
                            (4 + col_off) as f32,
                            ((WINDOW_HEIGHT as u64 - 95) + row_off) as f32,
                        ),
                        Pos2::new(
                            4.0 + size_offset + col_off as f32,
                            ((WINDOW_HEIGHT as u64 - 95 + PADDING_PIXELS) + row_off) as f32,
                        ),
                    ),
                ));

                col_off += (PADDING_PIXELS as f32 * key.size) as u64;
            }
            row_off += PADDING_PIXELS;
        }

        // Calculate the keyboard center (e.g., average of key rect centers)
        let all_rects: Vec<Rect> = rects.iter().map(|(_, r)| *r).collect();
        let sum = all_rects.iter().fold(Pos2::ZERO, |s, r| {
            let c = r.center();
            Pos2::new(s.x + c.x, s.y + c.y)
        });
        let count = (all_rects.len() - 1) as f32;
        let keyboard_center = if count > 0.0 {
            Pos2::new(sum.x / count, sum.y / count)
        } else {
            Pos2::ZERO
        };

        for (key, rect) in rects {
            this.shape_holder
                .push((key, rotate_rect(rect, keyboard_center, CAT_ANGLE).to_vec()));
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
        let themes = unsafe { self.themes.as_mut_unchecked() };
        if !self.themes_rendered {
            for theme in themes.themes_rendered.iter_mut() {
                let app_theme = themes
                    .themes_loaded
                    .iter()
                    .find(|t| Arc::ptr_eq(&t.id, &theme.id))
                    .unwrap();
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
            self.themes_rendered = true;
        }

        let painter = ctx.layer_painter(LayerId::background());

        let mut left_side_down = false;
        let mut right_side_down = false;
        let mut is_o_face = false;

        for (key, rect) in self.shape_holder.iter() {
            let color = if let KeybdKey::OtherKey(code) = key.key {
                if code == u64::MAX - 1 {
                    Color32::TRANSPARENT
                } else {
                    Color32::WHITE
                }
            } else {
                Color32::WHITE
            };

            painter.add(egui::Shape::convex_polygon(
                rect.clone(),
                match key.key {
                    KeybdKey::OtherKey(v) => {
                        if v < u64::MAX - 2
                            && self.keystroke_state.lit_keys_map[key.key].load(Ordering::Relaxed)
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
                        if self.keystroke_state.lit_keys_map[key.key].load(Ordering::Relaxed) {
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
            ));
        }

        let text = format!(
            "{}",
            self.keystroke_state.keystrokes.load(Ordering::Relaxed)
        );

        painter.add(
            TextShape::new(
                Pos2::new(
                    WINDOW_WIDTH - WINDOW_WIDTH / 2.0,
                    WINDOW_HEIGHT - (WINDOW_HEIGHT / 12.5) * 2.25,
                ),
                painter.layout_job(LayoutJob {
                    sections: vec![LayoutSection {
                        leading_space: 0.0,
                        byte_range: 0usize..text.len(),
                        format: TextFormat::simple(
                            FontId::new(WINDOW_HEIGHT / 12.5, FontFamily::Proportional),
                            Color32::WHITE,
                        ),
                    }],
                    text,
                    halign: Align::Max,
                    ..Default::default()
                }),
                Color32::PLACEHOLDER,
            )
            .with_angle(CAT_ANGLE),
        );

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
        self.exit_notify.store(true, Ordering::Relaxed);
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
