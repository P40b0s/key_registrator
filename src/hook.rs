use std::ptr::null_mut;
use windows_sys::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        CallNextHookEx, DispatchMessageA, GetMessageA, SetWindowsHookExA, TranslateMessage, UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN
    }
};

///хэндл хука
static mut HOOK: HHOOK = null_mut();
///коллбэк для обработки хука
unsafe extern "system" fn hook_callback(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT
{
    if n_code >= 0
    {
        let kb_struct = *(l_param as *const KBDLLHOOKSTRUCT);
        match w_param as u32
        {
            WM_KEYDOWN | WM_SYSKEYDOWN => 
            {
                println!("нажата кнопка {:#02x}", kb_struct.vkCode);
            },
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
            eprintln!("ошибка регистрации хука");
            return;
        }
        let mut msg = std::mem::zeroed();
        //для того чтобы программа была активной
        while GetMessageA(&mut msg, null_mut(), 0, 0) > 0
        {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
        UnhookWindowsHookEx(HOOK);
    }
}
