use crate::consts::keyboard::KEY_PRESSED_LIT_DELAY;
use dashmap::DashMap;
use enum_map::EnumMap;
use inputbot::KeybdKey;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// The keystroke related application state
#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub struct KeystrokeState {
    /// Number of total keystrokes ever
    pub(crate) keystrokes: AtomicUsize,
    /// Number of keystrokes per key
    keystroke_map: EnumMap<KeybdKey, AtomicUsize>,
    /// Deadline to repaint the UI due to a keypress
    #[serde(skip)]
    pub(crate) light_thread_deadline: Mutex<Option<Instant>>,
    /// Last time keys were pressed
    #[serde(skip)]
    pub(crate) last_pressed_map: DashMap<KeybdKey, Instant>,
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
            .insert(Instant::now() + KEY_PRESSED_LIT_DELAY);
        self.keystroke_map[*key].fetch_add(1, Ordering::Relaxed);
        self.keystrokes.fetch_add(1, Ordering::Relaxed);
        self.last_pressed_map.insert(*key, Instant::now());
    }

    pub fn cleanup_outdated(&self, max_age: Duration) {
        let now = Instant::now();
        self.last_pressed_map
            .retain(|_, last_pressed| now.duration_since(*last_pressed) < max_age);
    }
}
