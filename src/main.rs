#![windows_subsystem = "windows"]

use std::ffi::c_void;
use std::mem::size_of;

use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Dwm::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::Controls::*;
use windows::Win32::UI::WindowsAndMessaging::*;

const AERO_WINDOW_CLASS_NAME: PCSTR = s!("AeroWindow");

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(None)?;
        let black_brush = HBRUSH(GetStockObject(BLACK_BRUSH).0); // Don't ask me why

        let class = WNDCLASSA {
            lpfnWndProc: Some(wndproc),
            hInstance: instance.into(),
            lpszClassName: AERO_WINDOW_CLASS_NAME,
            hbrBackground: black_brush,
            ..Default::default()
        };

        if RegisterClassA(&class) == 0 {
            return Err(Error::from_win32());
        }

        let hwnd = CreateWindowExA(
            WS_EX_OVERLAPPEDWINDOW,
            AERO_WINDOW_CLASS_NAME,
            s!("Aero Window"),
            WS_OVERLAPPEDWINDOW,
            0,
            0,
            600,
            400,
            None,
            None,
            instance,
            None,
        )?;

        let margins = MARGINS {
            cxLeftWidth: -1,
            cxRightWidth: -1,
            cyTopHeight: -1,
            cyBottomHeight: -1,
        };

        DwmExtendFrameIntoClientArea(hwnd, &margins)?;

        // Enable dark mode and Mica on Windows 11+
        _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_USE_IMMERSIVE_DARK_MODE,
            (&TRUE) as *const _ as *const c_void,
            size_of::<BOOL>().try_into()?,
        );
        _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_SYSTEMBACKDROP_TYPE,
            (&DWMSBT_MAINWINDOW) as *const _ as *const c_void,
            size_of::<DWM_SYSTEMBACKDROP_TYPE>().try_into()?,
        );

        _ = ShowWindow(hwnd, SW_NORMAL);

        let mut msg = Default::default();

        while GetMessageA(&mut msg, None, 0, 0).into() {
            _ = TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }

        Ok(())
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CLOSE => {
            DestroyWindow(hwnd).unwrap();
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}
