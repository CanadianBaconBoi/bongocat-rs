use dashmap::DashMap;
use enum_map::EnumMap;
use inputbot::KeybdKey;
use parking_lot::Mutex;
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread::Thread;
use std::time::{Duration, Instant};

/// The keystroke-related application state
#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub struct KeystrokeState {
    /// Number of total keystrokes ever
    pub(crate) keystrokes: AtomicUsize,
    /// Number of keystrokes per key
    pub(crate) keystroke_map: EnumMap<KeybdKey, AtomicUsize>,
    #[serde(skip)]
    /// If the keys are lit or not
    pub(crate) lit_keys_map: EnumMap<KeybdKey, AtomicBool>,
    /// Last time keys were pressed
    #[serde(skip)]
    pub(crate) last_pressed_map: DashMap<KeybdKey, Instant>,
    /// Thread for ui input updates
    #[serde(skip)]
    pub(crate) input_update_thread: Mutex<Option<Thread>>,
}

impl Default for KeystrokeState {
    fn default() -> Self {
        Self {
            keystrokes: AtomicUsize::new(0),
            keystroke_map: EnumMap::default(),
            lit_keys_map: Default::default(),
            last_pressed_map: DashMap::new(),
            input_update_thread: Mutex::default(),
        }
    }
}

impl KeystrokeState {
    pub fn log_keystroke(&self, key: &KeybdKey) {
        self.keystroke_map[*key].fetch_add(1, Ordering::SeqCst);
        self.keystrokes.fetch_add(1, Ordering::SeqCst);
        self.last_pressed_map.insert(*key, Instant::now());

        self.lit_keys_map[*key].store(true, Ordering::SeqCst);

        if let Some(thread) = self.input_update_thread.lock().deref() {
            thread.unpark();
        }
    }

    #[inline(always)]
    /// `max_age`: How old from time of insertion the values can be
    pub fn cleanup_outdated(&self, max_age: Duration) {
        let threshold = Instant::now() - max_age;
        self.last_pressed_map.retain(|key, instant| {
            let keep = *instant > threshold;
            if !keep {
                self.lit_keys_map[*key].store(false, Ordering::SeqCst);
            }
            keep
        });
    }
}
