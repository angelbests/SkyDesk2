use std::{thread, time::Duration};
use tauri::{AppHandle, Manager};
use windows::{
    core::{s, BOOL},
    Win32::{
        Foundation::{COLORREF, HWND, LPARAM, POINT, WPARAM},
        Graphics::Gdi,
        UI::WindowsAndMessaging::{
            self, SetLayeredWindowAttributes, GWL_EXSTYLE, HWND_BOTTOM, HWND_NOTOPMOST, HWND_TOP,
            LWA_ALPHA, SMTO_NORMAL, SWP_HIDEWINDOW, SWP_SHOWWINDOW, SW_HIDE, SW_SHOWNORMAL,
            WS_EX_LAYERED,
        },
    },
};
#[tauri::command]
pub fn setwallpaper(app: AppHandle, label: String, x: i32, y: i32, w: i32, h: i32, z: i32) {
    tauri::async_runtime::spawn(async move {
        let webview = app.get_webview_window(&label);
        match webview {
            Some(webview) => match webview.hwnd() {
                Ok(hwnd) => attach(hwnd, x, y, w, h, z),
                Err(e) => println!("Failed to get hwnd: {:?}", e),
            },
            None => {
                println!("webview not found");
            }
        }
    });
}

#[tauri::command]
pub fn cancelwallpaper(app: AppHandle, label: String, x: i32, y: i32, w: i32, h: i32) {
    tauri::async_runtime::spawn(async move {
        let webview = app.get_webview_window(&label);
        match webview {
            Some(webview) => match webview.hwnd() {
                Ok(hwnd) => detach(hwnd, x, y, w, h),
                Err(e) => println!("Failed to get hwnd: {:?}", e),
            },
            None => {
                println!("webview not found");
            }
        }
    });
}

extern "system" fn enum_window(window: HWND, ref_worker_w: LPARAM) -> BOOL {
    unsafe {
        // 搜索 workerw下的 SHELLDLL_DefView 窗口的位置 确定到之后 下一个窗口就是 我们需要的workerw 窗口
        let shell_dll_def_view =
            WindowsAndMessaging::FindWindowExA(Some(window), None, s!("SHELLDLL_DefView"), None);
        match shell_dll_def_view {
            Ok(s) => {
                if s == HWND(std::ptr::null_mut()) {
                    return true.into();
                }
            }
            Err(_s) => {
                return true.into();
            }
        }

        let worker_w = WindowsAndMessaging::FindWindowExA(None, Some(window), s!("WorkerW"), None);
        match worker_w {
            Ok(s) => {
                if s == HWND(std::ptr::null_mut()) {
                    return true.into();
                } else {
                    // 查询 workerw 赋值给 workerw
                    *(ref_worker_w.0 as *mut HWND) = s;
                }
            }
            Err(_s) => {
                return true.into();
            }
        }
        // 查到 workerw 退出
        return false.into();
    }
}

fn attach(hwnd: HWND, x: i32, y: i32, w: i32, h: i32, z: i32) {
    unsafe {
        let progman = WindowsAndMessaging::FindWindowA(s!("Progman"), None).unwrap();

        // 发送0x052C  消息
        // WindowsAndMessaging::SendMessageA(progman, 0x052C, WPARAM(0x0000000D), LPARAM(0x00000001));
        WindowsAndMessaging::SendMessageTimeoutA(
            progman,
            0x052C,
            WPARAM(0x0000000D),
            LPARAM(0x00000001),
            SMTO_NORMAL,
            1000,
            None,
        );

        let mut worker_w: HWND = HWND(std::ptr::null_mut());
        // 查找workerw窗口
        let _ = WindowsAndMessaging::EnumWindows(
            Some(enum_window),
            LPARAM(&mut worker_w as *mut HWND as isize),
        );
        if worker_w == HWND(std::ptr::null_mut()) {
            worker_w = WindowsAndMessaging::FindWindowExA(Some(progman), None, s!("WorkerW"), None)
                .unwrap();
            if worker_w == HWND(std::ptr::null_mut()) {
                panic!("Could not find worker_w window");
            }
        }

        // 获取 worker_w的窗口坐标
        let p = &mut POINT { x: 0, y: 0 } as *mut POINT;
        let _ = Gdi::ClientToScreen(worker_w, p);
        let p = *p;
        // let _ = WindowsAndMessaging::MoveWindow(hwnd, 0-p.x+x, 0-p.y+y, w, h, true);
        // 修改tauri的窗口样式 防止窗口被强行关闭导致出现异常window框
        let _ = WindowsAndMessaging::SetWindowPos(
            hwnd,
            Some(HWND_BOTTOM),
            99999999,
            99999999,
            w,
            h,
            SWP_HIDEWINDOW,
        );
        // 该窗口是一个分层窗口。 如果窗口的 类样式 为 CS_OWNDC 或 CS_CLASSDC，则不能使用此样式。Windows 8：顶级窗口和子窗口支持WS_EX_LAYERED样式。 以前的 Windows 版本仅支持 顶级窗口WS_EX_LAYERED 。
        WindowsAndMessaging::SetWindowLongPtrA(
            hwnd,
            GWL_EXSTYLE,
            WindowsAndMessaging::GetWindowLongPtrA(hwnd, GWL_EXSTYLE) | WS_EX_LAYERED.0 as isize,
        );
        let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA);
        let _ = WindowsAndMessaging::SetParent(hwnd, Some(worker_w));
        // thread::sleep(Duration::from_millis(100));
        if z == 1 {
            let _ = WindowsAndMessaging::SetWindowPos(
                hwnd,
                Some(HWND_TOP),
                0 - p.x + x,
                0 - p.y + y,
                w,
                h,
                SWP_SHOWWINDOW,
            );
        } else {
            let _ = WindowsAndMessaging::SetWindowPos(
                hwnd,
                Some(HWND_BOTTOM),
                0 - p.x + x,
                0 - p.y + y,
                w,
                h,
                SWP_SHOWWINDOW,
            );
        };
        let shell_dll_def_view =
            WindowsAndMessaging::FindWindowExA(Some(progman), None, s!("SHELLDLL_DefView"), None)
                .unwrap();
        let _ = WindowsAndMessaging::ShowWindow(shell_dll_def_view, SW_HIDE);
        thread::sleep(Duration::from_millis(0));
        let _ = WindowsAndMessaging::ShowWindow(shell_dll_def_view, SW_SHOWNORMAL);
    };
}

fn detach(hwnd: HWND, x: i32, y: i32, w: i32, h: i32) {
    unsafe {
        let _ = WindowsAndMessaging::SetParent(hwnd, None);
        WindowsAndMessaging::SetWindowLongPtrA(
            hwnd,
            GWL_EXSTYLE,
            WindowsAndMessaging::GetWindowLongPtrA(hwnd, GWL_EXSTYLE) & WS_EX_LAYERED.0 as isize,
        );
        let _ = WindowsAndMessaging::SetWindowPos(
            hwnd,
            Some(HWND_NOTOPMOST),
            x,
            y,
            w,
            h,
            SWP_SHOWWINDOW,
        );
    };
}
