use std::collections::HashMap;

use crate::error::Error;
pub static KEYS_MAP: std::sync::LazyLock<KeysMap> = std::sync::LazyLock::new(||
{
    KeysMap::new()
});

pub struct KeysMap(HashMap<u32, String>);
impl KeysMap
{
    pub fn new() -> Self
    {
        let mut map = HashMap::new();
        map.insert(0x08, "Backspace".into());
        map.insert(0x09, "Tab".into());
        map.insert(0x0D, "Enter".into());
        map.insert(0x10, "Shift".into());
        map.insert(0x11, "Ctrl".into());
        map.insert(0x12, "Alt".into());
        map.insert(0x13, "Pause/Break".into());
        map.insert(0x14, "Caps Lock".into());
        map.insert(0x1B, "Escape".into());
        map.insert(0x20, "Space".into());
        map.insert(0x21, "Page Up".into());
        map.insert(0x22, "Page Down".into());
        map.insert(0x23, "End".into());
        map.insert(0x24, "Home".into());
        map.insert(0x25, "Left Arrow".into());
        map.insert(0x26, "Up Arrow".into());
        map.insert(0x27, "Right Arrow".into());
        map.insert(0x28, "Down Arrow".into());
        map.insert(0x2D, "Insert".into());
        map.insert(0x2E, "Delete".into());
        map.insert(0x30, "0".into());
        map.insert(0x31, "1".into());
        map.insert(0x32, "2".into());
        map.insert(0x33, "3".into());
        map.insert(0x34, "4".into());
        map.insert(0x35, "5".into());
        map.insert(0x36, "6".into());
        map.insert(0x37, "7".into());
        map.insert(0x38, "8".into());
        map.insert(0x39, "9".into());
        map.insert(0x41, "A".into());
        map.insert(0x42, "B".into());
        map.insert(0x43, "C".into());
        map.insert(0x44, "D".into());
        map.insert(0x45, "E".into());
        map.insert(0x46, "F".into());
        map.insert(0x47, "G".into());
        map.insert(0x48, "H".into());
        map.insert(0x49, "I".into());
        map.insert(0x4A, "J".into());
        map.insert(0x4B, "K".into());
        map.insert(0x4C, "L".into());
        map.insert(0x4D, "M".into());
        map.insert(0x4E, "N".into());
        map.insert(0x4F, "O".into());
        map.insert(0x50, "P".into());
        map.insert(0x51, "Q".into());
        map.insert(0x52, "R".into());
        map.insert(0x53, "S".into());
        map.insert(0x54, "T".into());
        map.insert(0x55, "U".into());
        map.insert(0x56, "V".into());
        map.insert(0x57, "W".into());
        map.insert(0x58, "X".into());
        map.insert(0x59, "Y".into());
        map.insert(0x5A, "Z".into());
        map.insert(0x60, "Numpad 0".into());
        map.insert(0x61, "Numpad 1".into());
        map.insert(0x62, "Numpad 2".into());
        map.insert(0x63, "Numpad 3".into());
        map.insert(0x64, "Numpad 4".into());
        map.insert(0x65, "Numpad 5".into());
        map.insert(0x66, "Numpad 6".into());
        map.insert(0x67, "Numpad 7".into());
        map.insert(0x68, "Numpad 8".into());
        map.insert(0x69, "Numpad 9".into());
        map.insert(0x6A, "Numpad *".into());
        map.insert(0x6B, "Numpad +".into());
        map.insert(0x6C, "Numpad Enter".into());
        map.insert(0x6D, "Numpad -".into());
        map.insert(0x6E, "Numpad .".into());
        map.insert(0x6F, "Numpad /".into());
        map.insert(0x70, "F1".into());
        map.insert(0x71, "F2".into());
        map.insert(0x72, "F3".into());
        map.insert(0x73, "F4".into());
        map.insert(0x74, "F5".into());
        map.insert(0x75, "F6".into());
        map.insert(0x76, "F7".into());
        map.insert(0x77, "F8".into());
        map.insert(0x78, "F9".into());
        map.insert(0x79, "F10".into());
        map.insert(0x7A, "F11".into());
        map.insert(0x7B, "F12".into());
        map.insert(0x90, "Num Lock".into());
        map.insert(0x91, "Scroll Lock".into());
        map.insert(0xA0, "Left Shift".into());
        map.insert(0xA1, "Right Shift".into());
        map.insert(0xA2, "Left Ctrl".into());
        map.insert(0xA3, "Right Ctrl".into());
        map.insert(0xA4, "Left Alt".into());
        map.insert(0xA5, "Right Alt".into());
        Self(map)
    }

    pub fn get_key(&self, code: u32) -> Result<&String, Error>
    {
        if let Some(k) = self.0.get(&code)
        {
            Ok(k)
        }
        else 
        {
            Err(Error::UnknownKey(code))    
        }
    }
}

use std::fmt;

#[derive(Debug)]
enum VirtualKey 
{
    Backspace,
    Tab,
    Enter,
    Shift,
    Ctrl,
    Alt,
    PauseBreak,
    CapsLock,
    Escape,
    Space,
    PageUp,
    PageDown,
    End,
    Home,
    LeftArrow,
    UpArrow,
    RightArrow,
    DownArrow,
    Insert,
    Delete,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadMultiply,
    NumpadAdd,
    NumpadEnter,
    NumpadSubtract,
    NumpadDecimal,
    NumpadDivide,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    NumLock,
    ScrollLock,
    LeftShift,
    RightShift,
    LeftCtrl,
    RightCtrl,
    LeftAlt,
    RightAlt,
    Unknown,
}

impl VirtualKey 
{
    fn from_code(code: u32) -> Self 
    {
        match code 
        {
            0x08 => VirtualKey::Backspace,
            0x09 => VirtualKey::Tab,
            0x0D => VirtualKey::Enter,
            0x10 => VirtualKey::Shift,
            0x11 => VirtualKey::Ctrl,
            0x12 => VirtualKey::Alt,
            0x13 => VirtualKey::PauseBreak,
            0x14 => VirtualKey::CapsLock,
            0x1B => VirtualKey::Escape,
            0x20 => VirtualKey::Space,
            0x21 => VirtualKey::PageUp,
            0x22 => VirtualKey::PageDown,
            0x23 => VirtualKey::End,
            0x24 => VirtualKey::Home,
            0x25 => VirtualKey::LeftArrow,
            0x26 => VirtualKey::UpArrow,
            0x27 => VirtualKey::RightArrow,
            0x28 => VirtualKey::DownArrow,
            0x2D => VirtualKey::Insert,
            0x2E => VirtualKey::Delete,
            0x30 => VirtualKey::Key0,
            0x31 => VirtualKey::Key1,
            0x32 => VirtualKey::Key2,
            0x33 => VirtualKey::Key3,
            0x34 => VirtualKey::Key4,
            0x35 => VirtualKey::Key5,
            0x36 => VirtualKey::Key6,
            0x37 => VirtualKey::Key7,
            0x38 => VirtualKey::Key8,
            0x39 => VirtualKey::Key9,
            0x41 => VirtualKey::KeyA,
            0x42 => VirtualKey::KeyB,
            0x43 => VirtualKey::KeyC,
            0x44 => VirtualKey::KeyD,
            0x45 => VirtualKey::KeyE,
            0x46 => VirtualKey::KeyF,
            0x47 => VirtualKey::KeyG,
            0x48 => VirtualKey::KeyH,
            0x49 => VirtualKey::KeyI,
            0x4A => VirtualKey::KeyJ,
            0x4B => VirtualKey::KeyK,
            0x4C => VirtualKey::KeyL,
            0x4D => VirtualKey::KeyM,
            0x4E => VirtualKey::KeyN,
            0x4F => VirtualKey::KeyO,
            0x50 => VirtualKey::KeyP,
            0x51 => VirtualKey::KeyQ,
            0x52 => VirtualKey::KeyR,
            0x53 => VirtualKey::KeyS,
            0x54 => VirtualKey::KeyT,
            0x55 => VirtualKey::KeyU,
            0x56 => VirtualKey::KeyV,
            0x57 => VirtualKey::KeyW,
            0x58 => VirtualKey::KeyX,
            0x59 => VirtualKey::KeyY,
            0x5A => VirtualKey::KeyZ,
            0x60 => VirtualKey::Numpad0,
            0x61 => VirtualKey::Numpad1,
            0x62 => VirtualKey::Numpad2,
            0x63 => VirtualKey::Numpad3,
            0x64 => VirtualKey::Numpad4,
            0x65 => VirtualKey::Numpad5,
            0x66 => VirtualKey::Numpad6,
            0x67 => VirtualKey::Numpad7,
            0x68 => VirtualKey::Numpad8,
            0x69 => VirtualKey::Numpad9,
            0x6A => VirtualKey::NumpadMultiply,
            0x6B => VirtualKey::NumpadAdd,
            0x6C => VirtualKey::NumpadEnter,
            0x6D => VirtualKey::NumpadSubtract,
            0x6E => VirtualKey::NumpadDecimal,
            0x6F => VirtualKey::NumpadDivide,
            0x70 => VirtualKey::F1,
            0x71 => VirtualKey::F2,
            0x72 => VirtualKey::F3,
            0x73 => VirtualKey::F4,
            0x74 => VirtualKey::F5,
            0x75 => VirtualKey::F6,
            0x76 => VirtualKey::F7,
            0x77 => VirtualKey::F8,
            0x78 => VirtualKey::F9,
            0x79 => VirtualKey::F10,
            0x7A => VirtualKey::F11,
            0x7B => VirtualKey::F12,
            0x90 => VirtualKey::NumLock,
            0x91 => VirtualKey::ScrollLock,
            0xA0 => VirtualKey::LeftShift,
            0xA1 => VirtualKey::RightShift,
            0xA2 => VirtualKey::LeftCtrl,
            0xA3 => VirtualKey::RightCtrl,
            0xA4 => VirtualKey::LeftAlt,
            0xA5 => VirtualKey::RightAlt,
            _ => VirtualKey::Unknown,
        }
    }
}

impl fmt::Display for VirtualKey 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        let name = match self 
        {
            VirtualKey::Backspace => "Backspace",
            VirtualKey::Tab => "Tab",
            VirtualKey::Enter => "Enter",
            VirtualKey::Shift => "Shift",
            VirtualKey::Ctrl => "Ctrl",
            VirtualKey::Alt => "Alt",
            VirtualKey::PauseBreak => "Pause/Break",
            VirtualKey::CapsLock => "Caps Lock",
            VirtualKey::Escape => "Escape",
            VirtualKey::Space => "Space",
            VirtualKey::PageUp => "Page Up",
            VirtualKey::PageDown => "Page Down",
            VirtualKey::End => "End",
            VirtualKey::Home => "Home",
            VirtualKey::LeftArrow => "Left Arrow",
            VirtualKey::UpArrow => "Up Arrow",
            VirtualKey::RightArrow => "Right Arrow",
            VirtualKey::DownArrow => "Down Arrow",
            VirtualKey::Insert => "Insert",
            VirtualKey::Delete => "Delete",
            VirtualKey::Key0 => "0",
            VirtualKey::Key1 => "1",
            VirtualKey::Key2 => "2",
            VirtualKey::Key3 => "3",
            VirtualKey::Key4 => "4",
            VirtualKey::Key5 => "5",
            VirtualKey::Key6 => "6",
            VirtualKey::Key7 => "7",
            VirtualKey::Key8 => "8",
            VirtualKey::Key9 => "9",
            VirtualKey::KeyA => "A",
            VirtualKey::KeyB => "B",
            VirtualKey::KeyC => "C",
            VirtualKey::KeyD => "D",
            VirtualKey::KeyE => "E",
            VirtualKey::KeyF => "F",
            VirtualKey::KeyG => "G",
            VirtualKey::KeyH => "H",
            VirtualKey::KeyI => "I",
            VirtualKey::KeyJ => "J",
            VirtualKey::KeyK => "K",
            VirtualKey::KeyL => "L",
            VirtualKey::KeyM => "M",
            VirtualKey::KeyN => "N",
            VirtualKey::KeyO => "O",
            VirtualKey::KeyP => "P",
            VirtualKey::KeyQ => "Q",
            VirtualKey::KeyR => "R",
            VirtualKey::KeyS => "S",
            VirtualKey::KeyT => "T",
            VirtualKey::KeyU => "U",
            VirtualKey::KeyV => "V",
            VirtualKey::KeyW => "W",
            VirtualKey::KeyX => "X",
            VirtualKey::KeyY => "Y",
            VirtualKey::KeyZ => "Z",
            VirtualKey::Numpad0 => "Numpad 0",
            VirtualKey::Numpad1 => "Numpad 1",
            VirtualKey::Numpad2 => "Numpad 2",
            VirtualKey::Numpad3 => "Numpad 3",
            VirtualKey::Numpad4 => "Numpad 4",
            VirtualKey::Numpad5 => "Numpad 5",
            VirtualKey::Numpad6 => "Numpad 6",
            VirtualKey::Numpad7 => "Numpad 7",
            VirtualKey::Numpad8 => "Numpad 8",
            VirtualKey::Numpad9 => "Numpad 9",
            VirtualKey::NumpadMultiply => "Numpad *",
            VirtualKey::NumpadAdd => "Numpad +",
            VirtualKey::NumpadEnter => "Numpad Enter",
            VirtualKey::NumpadSubtract => "Numpad -",
            VirtualKey::NumpadDecimal => "Numpad .",
            VirtualKey::NumpadDivide => "Numpad /",
            VirtualKey::F1 => "F1",
            VirtualKey::F2 => "F2",
            VirtualKey::F3 => "F3",
            VirtualKey::F4 => "F4",
            VirtualKey::F5 => "F5",
            VirtualKey::F6 => "F6",
            VirtualKey::F7 => "F7",
            VirtualKey::F8 => "F8",
            VirtualKey::F9 => "F9",
            VirtualKey::F10 => "F10",
            VirtualKey::F11 => "F11",
            VirtualKey::F12 => "F12",
            VirtualKey::NumLock => "Num Lock",
            VirtualKey::ScrollLock => "Scroll Lock",
            VirtualKey::LeftShift => "Left Shift",
            VirtualKey::RightShift => "Right Shift",
            VirtualKey::LeftCtrl => "Left Ctrl",
            VirtualKey::RightCtrl => "Right Ctrl",
            VirtualKey::LeftAlt => "Left Alt",
            VirtualKey::RightAlt => "Right Alt",
            VirtualKey::Unknown => "Unknown",
        };
        write!(f, "{}", name)
    }
}