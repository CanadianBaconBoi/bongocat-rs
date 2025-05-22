use crate::theme::{AppThemeImage, AppThemeTexture, ThemeSet};
use crate::{KEY_PRESSED_LIT_DELAY, KEYS, UV_RECT, WINDOW_HEIGHT, WINDOW_RECT, WINDOW_WIDTH};
use dashmap::DashMap;
use eframe::epaint::{Pos2, RectShape, TextShape, text::LayoutJob};
use egui::{
    Color32, ColorImage, Context, FontFamily, FontId, Rect, Shape, Stroke, StrokeKind,
    TextureHandle, TextureId, TextureOptions,
};
use enum_map::EnumMap;
use image::DynamicImage;
use inputbot::KeybdKey;
use parking_lot::{Mutex, RwLock};
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicUsize, Ordering},
};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{default::Default, ops::Add, thread, time::Instant};

/// The main application state
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct BongoApp {
    /// Theme (bongocat images) to be used
    #[serde(skip)]
    themes: RwLock<ThemeSet>,
    /// Access to the eGui `Context` from other threads
    #[serde(skip)]
    context_access: OnceLock<Context>,
    /// `JoinHandle`s for threads spunup
    #[serde(skip)]
    handles: Arc<Mutex<Vec<JoinHandle<()>>>>,
    /// Notifies threads of exiting
    #[serde(skip)]
    exit_notify: Arc<AtomicBool>,
    /// Are they rendered yet?
    #[serde(skip)]
    themes_rendered: bool,
    /// Keystroke related state
    keystroke_state: Arc<KeystrokeState>,
}

impl Default for BongoApp {
    fn default() -> Self {
        Self {
            themes: RwLock::new(ThemeSet::default()),
            keystroke_state: Arc::default(),
            context_access: OnceLock::default(),
            handles: Arc::default(),
            exit_notify: Arc::default(),
            themes_rendered: false,
        }
    }
}

/// The keystroke related application state
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct KeystrokeState {
    /// Number of total keystrokes ever
    keystrokes: AtomicUsize,
    /// Number of keystrokes per key
    keystroke_map: EnumMap<KeybdKey, AtomicUsize>,
    /// Deadline to repaint the UI due to a keypress
    #[serde(skip)]
    light_thread_deadline: Mutex<Option<Instant>>,
    /// Last time keys were pressed
    #[serde(skip)]
    last_pressed_map: DashMap<KeybdKey, Instant>,
}

impl Default for KeystrokeState {
    fn default() -> Self {
        Self {
            keystrokes: AtomicUsize::new(0),
            keystroke_map: EnumMap::default(),
            light_thread_deadline: Mutex::new(None),
            last_pressed_map: DashMap::new(),
        }
    }
}

impl KeystrokeState {
    pub fn log_keystroke(&self, key: &KeybdKey) {
        let _ = *self
            .light_thread_deadline
            .lock()
            .insert(Instant::now().add(KEY_PRESSED_LIT_DELAY));
        self.keystroke_map[*key].fetch_add(1, Ordering::Relaxed);
        self.keystrokes.fetch_add(1, Ordering::Relaxed);
        self.last_pressed_map.insert(*key, Instant::now());
    }

    pub fn cleanup_old_keystrokes(&self, older_than: Duration) {
        self.last_pressed_map
            .retain(|_, instant| instant.elapsed() < older_than);
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
            themes: RwLock::new(this.themes.read().clone()),
            context_access: this.context_access.clone(),
            handles: this.handles.clone(),
            exit_notify: this.exit_notify.clone(),
            themes_rendered: false,
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

        arc_this.handles.lock().push(thread::spawn(|| {
            inputbot::handle_input_events(true);
        }));

        let arc_clone = arc_this.clone();
        arc_this.handles.lock().push(thread::spawn(move || {
            let context = arc_clone.context_access.wait();
            loop {
                if arc_clone.exit_notify.load(Ordering::Relaxed) {
                    return;
                }

                if let Some(deadline) = *arc_clone.keystroke_state.light_thread_deadline.lock()
                    && let Some(duration) = deadline.checked_duration_since(Instant::now())
                {
                    thread::sleep(duration);
                } else {
                    thread::sleep(KEY_PRESSED_LIT_DELAY);
                }

                context.request_repaint();
            }
        }));

        let mut theme_guard = this.themes.write();
        let themes = theme_guard.themes.clone();

        for theme in themes {
            theme_guard.themes_loaded.push(AppThemeImage {
                app_theme: Some(theme.clone()),
                paws_both: load_color_image_from_path(&theme.paws_both),
                paws_left: load_color_image_from_path(&theme.paws_left),
                paws_right: load_color_image_from_path(&theme.paws_right),
                paws_up: load_color_image_from_path(&theme.paws_up),
            });
        }

        let themes_loaded = theme_guard.themes_loaded.clone();

        for theme in themes_loaded.iter() {
            theme_guard.themes_rendered.push(AppThemeTexture {
                app_theme_image: theme.clone(),
                paws_both: None,
                paws_left: None,
                paws_right: None,
                paws_up: None,
            });
        }
        drop(theme_guard);

        this
    }
}

fn load_texture_from_color_image(
    context: &Context,
    image: &ColorImage,
    name: String,
) -> TextureHandle {
    context.load_texture(name, image.clone(), TextureOptions::LINEAR)
}

fn load_color_image_from_path(image: &PathBuf) -> ColorImage {
    let image = image::ImageReader::open(image).unwrap().decode().unwrap();
    color_image_from_dynamic(image).unwrap_or_else(|err| {
        panic!("Failed to load image: {err:?}")
    })
}

pub fn color_image_from_dynamic(image: DynamicImage) -> Result<ColorImage, anyhow::Error> {
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
}

const SPACING: u64 = 10;
impl eframe::App for BongoApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if !self.themes_rendered {
            let mut theme_guard = self.themes.write();
            for i in 0..theme_guard.themes_rendered.len() {
                let theme = theme_guard.themes_rendered.get_mut(i).unwrap();
                let app_theme = &theme.app_theme_image;
                theme.paws_both.get_or_insert(load_texture_from_color_image(
                    ctx,
                    &app_theme.paws_both,
                    format!("paws_both_{i}"),
                ));
                theme.paws_left.get_or_insert(load_texture_from_color_image(
                    ctx,
                    &app_theme.paws_left,
                    format!("paws_left_{i}"),
                ));
                theme.paws_right.get_or_insert(load_texture_from_color_image(
                    ctx,
                    &app_theme.paws_right,
                    format!("paws_right_{i}"),
                ));
                theme.paws_up.get_or_insert(load_texture_from_color_image(
                    ctx,
                    &app_theme.paws_up,
                    format!("paws_up_{i}"),
                ));
            }
            drop(theme_guard);
            self.themes_rendered = true;
        }

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(Color32::TRANSPARENT)
                    .stroke(Stroke::new(0.0, Color32::TRANSPARENT)),
            )
            .show(ctx, |ui| {
                let mut left_side_down = false;
                let mut right_side_down = false;

                let mut row_off = 0;

                let mut is_o_face = false;

                let mut shape_holder: Vec<Shape> = Vec::new();

                for y in (0..KEYS.len()).rev() {
                    let mut col_off = 0;
                    for x in (0..KEYS[y].len()).rev() {
                        let key = &KEYS[y][x];
                        let size_offset = key.size * 10.0;
                        let color = if let KeybdKey::OtherKey(code) = key.key {
                            if code == u64::MAX - 1 {
                                Color32::TRANSPARENT
                            } else {
                                Color32::WHITE
                            }
                        } else {
                            Color32::WHITE
                        };
                        let shape = RectShape::new(
                            Rect::from_min_max(
                                Pos2::new(
                                    (10 + col_off) as f32,
                                    ((WINDOW_HEIGHT as u64 - 70) + row_off) as f32,
                                ),
                                Pos2::new(
                                    10.0 + size_offset + col_off as f32,
                                    ((WINDOW_HEIGHT as u64 - 70 + SPACING) + row_off) as f32,
                                ),
                            ),
                            0.5,
                            self.keystroke_state.last_pressed_map.get(&key.key).map_or(
                                Color32::TRANSPARENT,
                                |last_pressed| {
                                    if Instant::now().duration_since(*last_pressed)
                                        <= KEY_PRESSED_LIT_DELAY
                                    {
                                        // Position the center of the keyboard in the middle of the QWERTY layout
                                        // We assume keyboard is Full-Length
                                        if x > KEYS[y].len() / 3 {
                                            right_side_down = true;
                                        } else {
                                            left_side_down = true;
                                        }
                                        if key.key == KeybdKey::OKey
                                            || key.key == KeybdKey::Numrow0Key
                                            || key.key == KeybdKey::Numpad0Key
                                        {
                                            is_o_face = true;
                                        }
                                        Color32::LIGHT_BLUE
                                    } else {
                                        Color32::TRANSPARENT
                                    }
                                },
                            ),
                            Stroke::new(1.0, color),
                            StrokeKind::Middle,
                        );
                        col_off += (SPACING as f32 * key.size) as u64;
                        shape_holder.push(shape.into());
                    }
                    row_off += SPACING;
                }

                ui.painter().add(Shape::Vec(shape_holder));

                ui.painter().add(
                    TextShape::new(
                        Pos2::new(WINDOW_WIDTH / 20.0, WINDOW_HEIGHT / 2.0),
                        ui.painter().layout_job(LayoutJob::simple(
                            format!(
                                "{}",
                                self.keystroke_state.keystrokes.load(Ordering::Relaxed)
                            ),
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
                    themes.themes_rendered.get(1).or_else(|| {themes.themes_rendered.first()})
                } else {
                    themes.themes_rendered.first()
                };

                if let Some(theme) = theme {
                    let id = if left_side_down && right_side_down {
                        if let Some(paws_both) = &theme.paws_both {
                            paws_both.id()
                        } else {
                            TextureId::Managed(0)
                        }
                    } else if left_side_down {
                        if let Some(paws_left) = &theme.paws_left {
                            paws_left.id()
                        } else {
                            TextureId::Managed(0)
                        }
                    } else if right_side_down {
                        if let Some(paws_right) = &theme.paws_right {
                            paws_right.id()
                        } else {
                            TextureId::Managed(0)
                        }
                    } else if let Some(paws_up) = &theme.paws_up {
                        paws_up.id()
                    } else {
                        TextureId::Managed(0)
                    };

                    ui.painter().image(id, WINDOW_RECT, UV_RECT, Color32::WHITE);
                }
            });
    }
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.exit_notify.store(true, Ordering::Relaxed);
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
}
