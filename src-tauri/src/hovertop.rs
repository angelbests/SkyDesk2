use tauri::{AppHandle, Emitter, Manager, Runtime};
use windows::Win32::{
    Graphics::Dwm::{DwmEnableBlurBehindWindow, DwmExtendFrameIntoClientArea, DWM_BLURBEHIND},
    UI::Controls::MARGINS,
};

#[tauri::command]
pub fn hovertop(app: AppHandle, label: String, show: bool) {
    tauri::async_runtime::spawn(async move {
        let webview = app.get_webview_window(&label).unwrap();
        let p = webview.outer_position().unwrap();
        let s = webview.outer_size().unwrap();
        let h = s.height;
        let mut n = 0;
        if show {
            loop {
                n += 5;
                if n >= h {
                    n = h;
                }
                let _ = webview.set_position(tauri::PhysicalPosition::new(p.x, p.y + (n as i32)));
                std::thread::sleep(std::time::Duration::from_millis(1));
                if n >= h {
                    break;
                }
            }
        } else {
            loop {
                n += 5;
                if n >= h {
                    n = h;
                }
                let _ = webview.set_position(tauri::PhysicalPosition::new(p.x, 0 - n as i32));
                std::thread::sleep(std::time::Duration::from_millis(1));
                if n >= h {
                    break;
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(40));
        let _ = app.emit("hovertop_status", false);
    });
}

#[tauri::command]
pub fn window_transparent<R: Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    let hwnd = app.get_webview_window("hovertop").unwrap().hwnd().unwrap();
    unsafe {
        let blur = DWM_BLURBEHIND {
            dwFlags: 0x1, // DWM_BB_ENABLE
            fEnable: true.into(),
            hRgnBlur: Default::default(),
            fTransitionOnMaximized: false.into(),
        };

        let _ = DwmEnableBlurBehindWindow(hwnd, &blur);
        let margins = MARGINS {
            cxLeftWidth: -1,
            cxRightWidth: -1,
            cyTopHeight: -1,
            cyBottomHeight: -1,
        };
        let _ = DwmExtendFrameIntoClientArea(hwnd, &margins);
    }
    Ok(())
}
