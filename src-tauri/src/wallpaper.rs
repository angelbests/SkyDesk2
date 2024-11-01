use windows::{
    core::s,
    Win32::{
        Foundation::{BOOL, HWND, LPARAM, POINT, WPARAM},
        Graphics::Gdi,
        UI::WindowsAndMessaging::{
            self, GWL_EXSTYLE, GWL_STYLE, HWND_BOTTOM, SMTO_NORMAL, SWP_NOOWNERZORDER,
            WS_EX_ACCEPTFILES, WS_EX_APPWINDOW, WS_EX_LAYERED, WS_EX_TOPMOST, WS_EX_TRANSPARENT,
            WS_EX_WINDOWEDGE, WS_TILEDWINDOW,
        },
    },
};
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn setwallpaper(app: AppHandle, label: String, x: i32, y: i32, w: i32, h: i32) {
    tauri::async_runtime::spawn(async move {
        let webview = app.get_webview_window(&label).unwrap();
        let hwnd = webview.hwnd().unwrap();
        attach(hwnd, x, y, w, h);
    });
}

#[tauri::command]
pub fn cancelwallpaper(app: AppHandle, label: String) {
    tauri::async_runtime::spawn(async move {
        let webview = app.get_webview_window(&label).unwrap();
        let hwnd = webview.hwnd().unwrap();
        detach(hwnd);
    });
}

extern "system" fn enum_window(window: HWND, ref_worker_w: LPARAM) -> BOOL {
    unsafe {
        // 搜索 workerw下的 SHELLDLL_DefView 窗口的位置 确定到之后 下一个窗口就是 我们需要的workerw 窗口
        let shell_dll_def_view =
            WindowsAndMessaging::FindWindowExA(window, None, s!("SHELLDLL_DefView"), None);
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

        let worker_w = WindowsAndMessaging::FindWindowExA(None, window, s!("WorkerW"), None);
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

fn attach(hwnd: HWND, x: i32, y: i32, w: i32, h: i32) {
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
            panic!("Could not find worker_w window");
        }

        // 获取 worker_w的窗口坐标
        let p = &mut POINT { x: 0, y: 0 } as *mut POINT;
        let _ = Gdi::ClientToScreen(worker_w, p);
        let p = *p;
        // let _ = WindowsAndMessaging::MoveWindow(hwnd, 0-p.x+x, 0-p.y+y, w, h, true);
        let _ = WindowsAndMessaging::SetWindowPos(
            hwnd,
            HWND_BOTTOM,
            0 - p.x + x,
            0 - p.y + y,
            w,
            h,
            SWP_NOOWNERZORDER,
        );
        // 修改tauri的窗口样式 防止窗口被强行关闭导致出现异常window框
        WindowsAndMessaging::SetWindowLongPtrA(
            hwnd,
            GWL_STYLE,
            WindowsAndMessaging::GetWindowLongPtrA(hwnd, GWL_STYLE) & (!WS_TILEDWINDOW).0 as isize,
        );
        WindowsAndMessaging::SetWindowLongPtrA(
            hwnd,
            GWL_EXSTYLE,
            WindowsAndMessaging::GetWindowLongPtrA(hwnd, GWL_EXSTYLE)
                & (!WS_EX_APPWINDOW).0 as isize
                & (!WS_EX_WINDOWEDGE).0 as isize
                & (!WS_EX_ACCEPTFILES).0 as isize
                & (!WS_EX_TOPMOST).0 as isize
                | WS_EX_TRANSPARENT.0 as isize
                | WS_EX_LAYERED.0 as isize,
        );
        println!("{},{},{},{}", x, y, w, h);
        let _ = WindowsAndMessaging::SetParent(hwnd, worker_w);
    };
}

fn detach(hwnd: HWND) {
    unsafe {
        let _ = WindowsAndMessaging::SetParent(hwnd, None);
        let _ = WindowsAndMessaging::SystemParametersInfoA(
            WindowsAndMessaging::SPI_SETDESKWALLPAPER,
            0,
            None,
            WindowsAndMessaging::SPIF_UPDATEINIFILE,
        );
    };
}
