use serde::{Deserialize, Serialize};
use std::{
    sync::{LazyLock, Mutex},
    thread,
    time::Duration,
};
use tauri::{async_runtime::JoinHandle, AppHandle, Emitter, Manager};
use windows::{
    core::{s, BOOL},
    Win32::{
        Foundation::{GetLastError, HWND, LPARAM, RECT},
        Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_CLOAKED},
        UI::WindowsAndMessaging::{
            self, EnumWindows, GetClassLongPtrA, GetWindow, GetWindowLongW, GetWindowTextW,
            IsWindowVisible, GCLP_HICON, GWL_EXSTYLE, GW_OWNER, HICON, WS_EX_TOOLWINDOW,
        },
    },
};

#[derive(Serialize, Deserialize, Clone)]
struct Payload {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}
static TASKBAR: LazyLock<Mutex<Option<JoinHandle<()>>>> = LazyLock::new(|| Mutex::new(None));
#[tauri::command]
pub fn listentaskbar(app: AppHandle) {
    let mut taskbar = TASKBAR.lock().unwrap();
    *taskbar = Some(tauri::async_runtime::spawn(async move {
        let w = app.get_webview_window("taskbar").unwrap();
        let h = w.hwnd().unwrap();
        let mut factor = w.scale_factor().unwrap();
        unsafe {
            let shell_tray_wnd =
                WindowsAndMessaging::FindWindowA(s!("Shell_TrayWnd"), None).unwrap();
            let _ = WindowsAndMessaging::SetParent(h, Some(shell_tray_wnd));
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
                let start = WindowsAndMessaging::FindWindowExA(
                    Some(shell_tray_wnd),
                    None,
                    s!("Start"),
                    None,
                )
                .unwrap();
                let tray = WindowsAndMessaging::FindWindowExA(
                    Some(shell_tray_wnd),
                    None,
                    s!("TrayNotifyWnd"),
                    None,
                )
                .unwrap();
                let start_rect: *mut RECT = &mut RECT {
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
                        x: tray_rect.left - (270 as f64 * factor) as i32,
                        y: 0,
                        height: start_rect.bottom - start_rect.top,
                        width: (270 as f64 * factor) as i32,
                    };
                } else {
                    p = Payload {
                        x: 0,
                        y: 0,
                        height: start_rect.bottom - start_rect.top,
                        width: (270 as f64 * factor) as i32,
                    };
                }

                let _ = WindowsAndMessaging::MoveWindow(h, p.x, p.y, p.width, p.height, false);
                let p_rect = &mut RECT {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom: 0,
                } as *mut RECT;
                let _ = WindowsAndMessaging::GetWindowRect(h, p_rect);
                let p_rect = *p_rect;
                let p = Payload {
                    x: p_rect.left,
                    y: p_rect.top,
                    width: p_rect.right - p_rect.left,
                    height: p_rect.bottom - p_rect.top,
                };
                let _ = app.emit("taskbar", p);
                thread::sleep(Duration::from_millis(300));
            }
        }
    }));
}

#[tauri::command]
pub fn closetaskbar() {
    let mut taskbar = TASKBAR.lock().unwrap();
    if let Some(handle) = taskbar.as_mut() {
        handle.abort();
        *taskbar = None;
    }
}

#[tauri::command]
pub fn gettaskbarlist() {
    let mut vec: Vec<HWND> = vec![];
    unsafe {
        let _ = EnumWindows(Some(enum_windows_proc), LPARAM(&mut vec as *mut _ as isize));
        println!("{:?}", vec);

        for item in vec.iter() {
            let hiconptr = GetClassLongPtrA(*item, GCLP_HICON);
            let e = GetLastError();
            //    let vec = lparam.0 as *mut Vec<HWND>;
            let ico = hiconptr as *mut HICON;
            let hicon = *ico;

            println!("{:?},{:?}", hicon, e);
        }
    }
}

fn is_taskbar_window(hwnd: HWND) -> bool {
    unsafe {
        // 检查窗口是否可见
        if !IsWindowVisible(hwnd).as_bool() {
            return false;
        }

        // 检查窗口是否被遮蔽（Cloaked）
        // DWMWA_CLOAK
        // 与 DwmSetWindowAttribute一起使用。 隐藏窗口，使它对用户不可见。 窗口仍由 DWM 组成。
        let mut cloaked: u32 = 0;
        if DwmGetWindowAttribute(
            hwnd,
            DWMWA_CLOAKED,
            &mut cloaked as *mut _ as *mut _,
            std::mem::size_of::<u32>() as u32,
        )
        .is_ok()
            && cloaked != 0
        {
            return false;
        }

        // 检查窗口是否有所有者
        if GetWindow(hwnd, GW_OWNER) == Ok(HWND::default()) {
            return false;
        }

        // 检查窗口扩展样式，排除工具窗口
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        if (ex_style & WS_EX_TOOLWINDOW.0 as i32) != 0 {
            return false;
        }

        // 检查窗口标题是否为空
        let mut title = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut title);
        if len == 0 {
            return false;
        }

        true
    }
}

extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    if is_taskbar_window(hwnd) {
        unsafe {
            let mut title = [0u16; 512];
            let len = GetWindowTextW(hwnd, &mut title);
            let title = String::from_utf16_lossy(&title[..len as usize]);
            let vec = lparam.0 as *mut Vec<HWND>;
            (*vec).push(hwnd);
            println!("任务栏窗口: HWND = {:?}, 标题 = {}", hwnd, title);
        }
    }

    true.into() // 返回 TRUE 继续枚举
}
