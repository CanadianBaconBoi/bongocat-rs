use inputbot::KeybdKey;

pub const KEY_PRESSED_LIT_DELAY: std::time::Duration = std::time::Duration::from_millis(250);

pub const LAST_PRESSED_CLEANUP_CHECK_INTERVAL: std::time::Duration =
    std::time::Duration::from_millis(1000);
pub const LAST_PRESSED_MAX_AGE: std::time::Duration = std::time::Duration::from_secs(60);

pub struct VisualKeybdKeyHolder {
    pub size: f32,
    pub key: KeybdKey,
    pub name: &'static str,
}

pub const KEYS: [&[VisualKeybdKeyHolder]; 6] = [
    &ROW_ONE, &ROW_TWO, &ROW_THREE, &ROW_FOUR, &ROW_FIVE, &ROW_SIX,
];

const ROW_ONE: [VisualKeybdKeyHolder; 21] = [
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::EscapeKey,
        name: "Esc",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::OtherKey(u64::MAX - 1),
        name: " ",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::F1Key,
        name: "F1",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::F2Key,
        name: "F2",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::F3Key,
        name: "F3",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::F4Key,
        name: "F4",
    },
    VisualKeybdKeyHolder {
        size: 0.5,
        key: KeybdKey::OtherKey(u64::MAX - 1),
        name: " ",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::F5Key,
        name: "F5",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::F6Key,
        name: "F6",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::F7Key,
        name: "F7",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::F8Key,
        name: "F8",
    },
    VisualKeybdKeyHolder {
        size: 0.5,
        key: KeybdKey::OtherKey(u64::MAX - 1),
        name: " ",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::F9Key,
        name: "F9",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::F10Key,
        name: "F10",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::F11Key,
        name: "F11",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::F12Key,
        name: "F12",
    },
    VisualKeybdKeyHolder {
        size: 0.5,
        key: KeybdKey::OtherKey(u64::MAX - 1),
        name: " ",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::OtherKey(u64::MAX - 1 - 1),
        name: "Prt.Scn",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::OtherKey(u64::MAX - 1 - 1),
        name: "Pause",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::ScrollLockKey,
        name: "Scrl.Lock",
    },
    VisualKeybdKeyHolder {
        size: 4.5,
        key: KeybdKey::OtherKey(u64::MAX - 1),
        name: " ",
    },
];
const ROW_TWO: [VisualKeybdKeyHolder; 23] = [
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::BackquoteKey,
        name: "`",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numrow1Key,
        name: "1",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numrow2Key,
        name: "2",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numrow3Key,
        name: "3",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numrow4Key,
        name: "4",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numrow5Key,
        name: "5",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numrow6Key,
        name: "6",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numrow7Key,
        name: "7",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numrow8Key,
        name: "8",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numrow9Key,
        name: "9",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numrow0Key,
        name: "0",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::MinusKey,
        name: "-",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::EqualKey,
        name: "=",
    },
    VisualKeybdKeyHolder {
        size: 2.0,
        key: KeybdKey::BackspaceKey,
        name: "<--",
    },
    VisualKeybdKeyHolder {
        size: 0.5,
        key: KeybdKey::OtherKey(u64::MAX - 1),
        name: " ",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::InsertKey,
        name: "Ins",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::HomeKey,
        name: "Home",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::PageUpKey,
        name: "Pg.Up",
    },
    VisualKeybdKeyHolder {
        size: 0.5,
        key: KeybdKey::OtherKey(u64::MAX - 1),
        name: " ",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::NumLockKey,
        name: "NumLk",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::OtherKey(61),
        name: "/",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::OtherKey(63),
        name: "*",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::OtherKey(109),
        name: "-",
    },
];
const ROW_THREE: [VisualKeybdKeyHolder; 23] = [
    VisualKeybdKeyHolder {
        size: 1.5,
        key: KeybdKey::TabKey,
        name: "-->",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::QKey,
        name: "Q",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::WKey,
        name: "W",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::EKey,
        name: "E",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::RKey,
        name: "R",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::TKey,
        name: "T",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::YKey,
        name: "Y",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::UKey,
        name: "U",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::IKey,
        name: "I",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::OKey,
        name: "O",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::PKey,
        name: "P",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::LBracketKey,
        name: "[",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::RBracketKey,
        name: "]",
    },
    VisualKeybdKeyHolder {
        size: 1.5,
        key: KeybdKey::BackslashKey,
        name: "\\",
    },
    VisualKeybdKeyHolder {
        size: 0.5,
        key: KeybdKey::OtherKey(u64::MAX - 1),
        name: " ",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::DeleteKey,
        name: "Del",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::EndKey,
        name: "End",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::PageDownKey,
        name: "Pg.Down",
    },
    VisualKeybdKeyHolder {
        size: 0.5,
        key: KeybdKey::OtherKey(u64::MAX - 1),
        name: " ",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numpad7Key,
        name: "7",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numpad8Key,
        name: "88",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numpad9Key,
        name: "9",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::OtherKey(69),
        name: "+",
    },
];
const ROW_FOUR: [VisualKeybdKeyHolder; 18] = [
    VisualKeybdKeyHolder {
        size: 1.75,
        key: KeybdKey::CapsLockKey,
        name: "Caps",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::AKey,
        name: "A",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::SKey,
        name: "S",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::DKey,
        name: "D",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::FKey,
        name: "F",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::GKey,
        name: "G",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::HKey,
        name: "H",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::JKey,
        name: "J",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::KKey,
        name: "K",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::LKey,
        name: "L",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::SemicolonKey,
        name: ";",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::QuoteKey,
        name: "'",
    },
    VisualKeybdKeyHolder {
        size: 2.25,
        key: KeybdKey::EnterKey,
        name: "Enter",
    },
    VisualKeybdKeyHolder {
        size: 4.0,
        key: KeybdKey::OtherKey(u64::MAX - 1),
        name: " ",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numpad4Key,
        name: "4",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numpad5Key,
        name: "5",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numpad6Key,
        name: "6",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::OtherKey(69),
        name: "+",
    },
];
const ROW_FIVE: [VisualKeybdKeyHolder; 19] = [
    VisualKeybdKeyHolder {
        size: 2.5,
        key: KeybdKey::LShiftKey,
        name: "Shift",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::ZKey,
        name: "Z",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::XKey,
        name: "X",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::CKey,
        name: "C",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::VKey,
        name: "V",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::BKey,
        name: "B",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::NKey,
        name: "N",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::MKey,
        name: "M",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::CommaKey,
        name: ",",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::PeriodKey,
        name: ".",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::SlashKey,
        name: "/",
    },
    VisualKeybdKeyHolder {
        size: 2.5,
        key: KeybdKey::RShiftKey,
        name: "Shift",
    },
    VisualKeybdKeyHolder {
        size: 1.5,
        key: KeybdKey::OtherKey(u64::MAX - 1),
        name: " ",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::UpKey,
        name: "^",
    },
    VisualKeybdKeyHolder {
        size: 1.5,
        key: KeybdKey::OtherKey(u64::MAX - 1),
        name: " ",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numpad1Key,
        name: "1",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numpad2Key,
        name: "2",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::Numpad3Key,
        name: "3",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::OtherKey(96),
        name: "Enter",
    },
];
const ROW_SIX: [VisualKeybdKeyHolder; 15] = [
    VisualKeybdKeyHolder {
        size: 1.5,
        key: KeybdKey::LControlKey,
        name: "Ctrl",
    },
    VisualKeybdKeyHolder {
        size: 1.5,
        key: KeybdKey::LSuper,
        name: "Win",
    },
    VisualKeybdKeyHolder {
        size: 1.5,
        key: KeybdKey::LAltKey,
        name: "Alt",
    },
    VisualKeybdKeyHolder {
        size: 6.0,
        key: KeybdKey::SpaceKey,
        name: "----",
    },
    VisualKeybdKeyHolder {
        size: 1.5,
        key: KeybdKey::RAltKey,
        name: "Alt",
    },
    VisualKeybdKeyHolder {
        size: 1.5,
        key: KeybdKey::RSuper,
        name: "Win",
    },
    VisualKeybdKeyHolder {
        size: 1.5,
        key: KeybdKey::RControlKey,
        name: "Ctrl",
    },
    VisualKeybdKeyHolder {
        size: 0.5,
        key: KeybdKey::OtherKey(u64::MAX - 1),
        name: " ",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::LeftKey,
        name: "<",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::DownKey,
        name: "V",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::RightKey,
        name: ">",
    },
    VisualKeybdKeyHolder {
        size: 0.5,
        key: KeybdKey::OtherKey(u64::MAX - 1),
        name: " ",
    },
    VisualKeybdKeyHolder {
        size: 2.0,
        key: KeybdKey::Numpad0Key,
        name: "0",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::DeleteKey,
        name: ".",
    },
    VisualKeybdKeyHolder {
        size: 1.0,
        key: KeybdKey::OtherKey(96),
        name: "Enter",
    },
];
