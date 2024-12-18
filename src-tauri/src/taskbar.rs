use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use windows::{
    core::s,
    Win32::{
        Foundation::RECT,
        UI::WindowsAndMessaging::{self, HWND_TOP, SWP_SHOWWINDOW},
    },
};

#[derive(Serialize, Deserialize)]
struct Payload {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[tauri::command]
pub fn listentaskbar(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let w = app.get_webview_window("taskbar").unwrap();
        let h = w.hwnd().unwrap();
        let mut factor = w.scale_factor().unwrap();
        unsafe {
            let shell_tray_wnd =
                WindowsAndMessaging::FindWindowA(s!("Shell_TrayWnd"), None).unwrap();
            let _ = WindowsAndMessaging::SetParent(h, shell_tray_wnd);
        }
        loop {
            unsafe {
                let c = w.current_monitor().unwrap();
                match c {
                    Some(d) => {
                        factor = d.scale_factor();
                    }
                    None => {}
                }
                let shell_tray_wnd =
                    WindowsAndMessaging::FindWindowA(s!("Shell_TrayWnd"), None).unwrap();
                let start =
                    WindowsAndMessaging::FindWindowExA(shell_tray_wnd, None, s!("Start"), None)
                        .unwrap();
                let tray = WindowsAndMessaging::FindWindowExA(
                    shell_tray_wnd,
                    None,
                    s!("TrayNotifyWnd"),
                    None,
                )
                .unwrap();
                let start_rect = &mut RECT {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom: 0,
                } as *mut RECT;
                let _ = WindowsAndMessaging::GetWindowRect(start, start_rect);
                let start_rect = *start_rect;

                let tray_rect = &mut RECT {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom: 0,
                } as *mut RECT;
                let _ = WindowsAndMessaging::GetWindowRect(tray, tray_rect);
                let tray_rect = *tray_rect;
                let p: Payload;
                if start_rect.left == 0 {
                    p = Payload {
                        x: tray_rect.left - (170 as f64 * factor) as i32,
                        y: 0,
                        height: start_rect.bottom - start_rect.top,
                        width: (170 as f64 * factor) as i32,
                    };
                } else {
                    p = Payload {
                        x: 0,
                        y: 0,
                        height: start_rect.bottom - start_rect.top,
                        width: (170 as f64 * factor) as i32,
                    };
                }
                let json = serde_json::to_string(&p).unwrap();
                let _ = app.emit("taskbar", json);
                let _ = WindowsAndMessaging::SetWindowPos(
                    h,
                    HWND_TOP,
                    p.x,
                    p.y,
                    p.width,
                    p.height,
                    SWP_SHOWWINDOW,
                );
                thread::sleep(Duration::from_millis(300));
            }
        }
    });
}
