use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HWINSTA, HDESK, BOOL},
        UI::WindowsAndMessaging::{CreateDesktopW, SwitchDesktop, CloseDesktop, DESKTOP_SWITCHDESKTOP, DESKTOP_CREATEWINDOW},
    },
};
use windows::{
    Win32::{
        Graphics::Gdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDesktopWindow, SelectObject, SRCCOPY},
        Foundation::{HWND, HDC, HBITMAP},
    },
};

use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HWND, HINSTANCE, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::{BitBlt, CreateCompatibleDC, DeleteDC, SelectObject, StretchBlt, SRCCOPY},
        UI::WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW, PostQuitMessage, RegisterClassExW, TranslateMessage, WNDCLASSEXW, WS_OVERLAPPEDWINDOW, WS_VISIBLE, CW_USEDEFAULT, WM_DESTROY, WM_PAINT, CS_HREDRAW, CS_VREDRAW},
    },
};


pub fn create_desktop() {
    let desktop_name = PCWSTR("UAC_Desktop".as_ptr() as *const u16);

    unsafe {
        // Создаем новый рабочий стол
        let hdesk = CreateDesktopW(
            desktop_name,
            None,
            None,
            0,
            DESKTOP_SWITCHDESKTOP | DESKTOP_CREATEWINDOW,
            None,
        );

        if hdesk.is_null() {
            println!("Failed to create desktop");
            return;
        }

        // Переключаемся на новый рабочий стол
        if SwitchDesktop(hdesk).as_bool() {
            println!("Switched to new desktop");

            // Здесь можно создать окно, похожее на UAC

            // Возвращаемся на исходный рабочий стол
            SwitchDesktop(HDESK::default()).unwrap();
        } else {
            println!("Failed to switch desktop");
        }

        // Закрываем рабочий стол
        CloseDesktop(hdesk).unwrap();
    }
}



fn capture_screen() -> Option<HBITMAP> {
    unsafe {
        let hwnd = GetDesktopWindow();
        let hdc_screen = GetDC(hwnd);
        let hdc_mem = CreateCompatibleDC(hdc_screen);

        let width = 1920; // Ширина экрана
        let height = 1080; // Высота экрана

        let hbitmap = CreateCompatibleBitmap(hdc_screen, width, height);
        SelectObject(hdc_mem, hbitmap);

        BitBlt(hdc_mem, 0, 0, width, height, hdc_screen, 0, 0, SRCCOPY);

        DeleteDC(hdc_mem);
        ReleaseDC(hwnd, hdc_screen);

        Some(hbitmap)
    }
}


unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_PAINT => {
            // Отрисовка фонового изображения
            let hdc = GetDC(hwnd);
            let hbitmap = capture_screen().unwrap();
            let hdc_mem = CreateCompatibleDC(hdc);
            SelectObject(hdc_mem, hbitmap);

            let rect = RECT { left: 0, top: 0, right: 1920, bottom: 1080 };
            StretchBlt(hdc, 0, 0, rect.right, rect.bottom, hdc_mem, 0, 0, rect.right, rect.bottom, SRCCOPY);

            DeleteDC(hdc_mem);
            ReleaseDC(hwnd, hdc);
        }
        WM_DESTROY => {
            PostQuitMessage(0);
        }
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
    LRESULT(0)
}

fn create_window() -> HWND {
    unsafe {
        let hinstance = HINSTANCE::default();
        let class_name = PCWSTR("UAC_Window_Class".as_ptr() as *const u16);

        let wnd_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: hinstance,
            lpszClassName: class_name,
            hCursor: LoadCursorW(None, IDC_ARROW),
            ..Default::default()
        };

        RegisterClassExW(&wnd_class);

        CreateWindowExW(
            0,
            class_name,
            PCWSTR("UAC Window".as_ptr() as *const u16),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            None,
            None,
            hinstance,
            None,
        )
    }
}