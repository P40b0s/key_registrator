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
