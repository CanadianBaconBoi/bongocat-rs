use crate::theme::{AppTheme, ThemeSet};
use crate::{KEY_PRESSED_LIT_DELAY, KEYS, WINDOW_HEIGHT, WINDOW_WIDTH};
use dashmap::DashMap;
use eframe::epaint::{Pos2, RectShape, TextShape, text::LayoutJob};
use egui::{Color32, Context, FontFamily, FontId, Rect, Shape, Stroke, StrokeKind};
use enum_map::EnumMap;
use inputbot::KeybdKey;
use parking_lot::{Mutex, RwLock};
use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Duration;
use std::{default::Default, ops::Add, thread, time::Instant};

/// The main application state
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct BongoApp {
    /// Theme (bongocat images) to be used
    #[serde(skip)]
    theme: RwLock<ThemeSet>,
    /// Access to the eGui `Context` from other threads
    #[serde(skip)]
    context_access: OnceLock<Context>,
    /// Keystroke related state
    keystroke_state: Arc<KeystrokeState>,
}

impl Default for BongoApp {
    fn default() -> Self {
        Self {
            theme: RwLock::new(ThemeSet {
                themes: Vec::from([AppTheme::default(), AppTheme::new("assets/frames/o")]),
            }),
            keystroke_state: Arc::default(),
            context_access: OnceLock::default(),
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
            theme: RwLock::new(this.theme.read().clone()),
            context_access: this.context_access.clone(),
            keystroke_state: this.keystroke_state.clone(),
        });

        for key in &KEYS {
            for key in *key {
                let arc_this = arc_this.clone();
                key.key.bind(move || {
                    Self::log_key(&arc_this.clone(), &key.key);
                });
            }
        }

        thread::spawn(|| {
            inputbot::handle_input_events(true);
        });

        let arc_state = this.keystroke_state.clone();
        let arc_context = this.context_access.clone();
        thread::spawn(move || {
            loop {
                if let Some(deadline) = *arc_state.clone().light_thread_deadline.lock()
                    && let Some(duration) = deadline.checked_duration_since(Instant::now())
                {
                    thread::sleep(duration);
                    arc_context.clone().wait().request_repaint();
                } else {
                    thread::sleep(KEY_PRESSED_LIT_DELAY);
                    arc_context.clone().wait().request_repaint();
                }
            }
        });

        this
    }
}

const SPACING: u64 = 10;
impl eframe::App for BongoApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(Color32::TRANSPARENT)
                    .stroke(Stroke::new(0.0, Color32::TRANSPARENT)),
            )
            .show(ctx, |ui| {
                let mut left_side_down = false;
                let mut right_side_down = false;

                let mut y_off = 0;

                let mut is_o_face = false;

                let mut shape_holder: Vec<Shape> = Vec::new();

                for y in (0..KEYS.len()).rev() {
                    let mut x_off = 0;
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
                                    (10 + x_off) as f32,
                                    ((WINDOW_HEIGHT as u64 - 70) + y_off) as f32,
                                ),
                                Pos2::new(
                                    10.0 + size_offset + x_off as f32,
                                    ((WINDOW_HEIGHT as u64 - 70 + SPACING) + y_off) as f32,
                                ),
                            ),
                            0.5,
                            self.keystroke_state.last_pressed_map.get(&key.key).map_or(
                                Color32::TRANSPARENT,
                                |last_pressed| {
                                    if Instant::now().duration_since(*last_pressed)
                                        <= KEY_PRESSED_LIT_DELAY
                                    {
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
                        x_off += (SPACING as f32 * key.size) as u64;
                        shape_holder.push(shape.into());
                    }
                    y_off += SPACING;
                }

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

                if is_o_face && self.theme.read().themes.len() > 1 {
                    if left_side_down && right_side_down {
                        ui.add(
                            self.theme.read().themes[1]
                                .paws_both
                                .clone()
                                .max_width(WINDOW_WIDTH),
                        );
                    } else if left_side_down {
                        ui.add(
                            self.theme.read().themes[1]
                                .paws_left
                                .clone()
                                .max_width(WINDOW_WIDTH),
                        );
                    } else if right_side_down {
                        ui.add(
                            self.theme.read().themes[1]
                                .paws_right
                                .clone()
                                .max_width(WINDOW_WIDTH),
                        );
                    } else {
                        ui.add(
                            self.theme.read().themes[1]
                                .paws_up
                                .clone()
                                .max_width(WINDOW_WIDTH),
                        );
                    }
                } else if left_side_down && right_side_down {
                    ui.add(
                        self.theme
                            .read()
                            .first()
                            .paws_both
                            .clone()
                            .max_width(WINDOW_WIDTH),
                    );
                } else if left_side_down {
                    ui.add(
                        self.theme
                            .read()
                            .first()
                            .paws_left
                            .clone()
                            .max_width(WINDOW_WIDTH),
                    );
                } else if right_side_down {
                    ui.add(
                        self.theme
                            .read()
                            .first()
                            .paws_right
                            .clone()
                            .max_width(WINDOW_WIDTH),
                    );
                } else {
                    ui.add(
                        self.theme
                            .read()
                            .first()
                            .paws_up
                            .clone()
                            .max_width(WINDOW_WIDTH),
                    );
                }

                ui.painter().add(Shape::Vec(shape_holder));
            });
    }
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
}
