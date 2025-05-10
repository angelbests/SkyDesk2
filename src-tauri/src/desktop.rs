use tauri::{AppHandle, Emitter, Manager};
use windows::core::{s, BOOL};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, EnumWindows, FindWindowExA, SetWindowsHookExA, WindowFromPoint, HC_ACTION,
    HHOOK, MSLLHOOKSTRUCT, WH_MOUSE_LL, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP,
    WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP,
};

#[derive(Clone, serde::Serialize, PartialEq)]
enum MouseAction {
    LeftDown,
    LeftUp,
    RightDown,
    RightUp,
    MiddleDown,
    MiddleUp,
    Move,
    Wheel,
}

#[derive(Clone, serde::Serialize)]
struct MouseInfo {
    x: i32,
    y: i32,
    mouse: MouseAction,
    monitor: Option<Monitor>,
    hwnd: usize,
}
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::{LazyLock, Mutex};
static TX: LazyLock<Mutex<Option<SyncSender<Option<MouseInfo>>>>> =
    LazyLock::new(|| Mutex::new(None));
static mut HOOK: Option<HHOOK> = None;
extern "system" fn enum_window(window: HWND, ref_worker_w: LPARAM) -> BOOL {
    unsafe {
        // 搜索 workerw下的 SHELLDLL_DefView 窗口的位置 确定到之后 下一个窗口就是 我们需要的workerw 窗口
        let shell_dll_def_view = FindWindowExA(Some(window), None, s!("SHELLDLL_DefView"), None);
        match shell_dll_def_view {
            Ok(s) => {
                if s == HWND(std::ptr::null_mut()) {
                    return true.into();
                } else {
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

unsafe extern "system" fn mouse_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if (n_code as u32) == HC_ACTION {
        let mouse_info = *(l_param.0 as *const MSLLHOOKSTRUCT);
        let hwnd_clicked = WindowFromPoint(mouse_info.pt);

        match w_param.0 as u32 {
            WM_LBUTTONDOWN => {
                println!("鼠标左键点击了桌面");
                if let Some(sender) = &*TX.lock().unwrap() {
                    let _ = sender.try_send(Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::LeftDown,
                        monitor: None,
                        hwnd: hwnd_clicked.0.addr(),
                    }));
                }
            }
            WM_RBUTTONDOWN => {
                // println!("鼠标右键点击了桌面");
                if let Some(sender) = &*TX.lock().unwrap() {
                    let _ = sender.try_send(Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::RightDown,
                        monitor: None,
                        hwnd: hwnd_clicked.0.addr(),
                    }));
                }
            }
            WM_MBUTTONDOWN => {
                // println!("鼠标中键点击了桌面");
                if let Some(sender) = &*TX.lock().unwrap() {
                    let _ = sender.try_send(Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::MiddleDown,
                        monitor: None,
                        hwnd: hwnd_clicked.0.addr(),
                    }));
                }
            }
            WM_LBUTTONUP => {
                // println!("鼠标左键释放了桌面");
                if let Some(sender) = &*TX.lock().unwrap() {
                    let _ = sender.try_send(Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::LeftUp,
                        monitor: None,
                        hwnd: hwnd_clicked.0.addr(),
                    }));
                }
            }
            WM_RBUTTONUP => {
                // println!("鼠标右键释放了桌面");
                if let Some(sender) = &*TX.lock().unwrap() {
                    let _ = sender.try_send(Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::RightUp,
                        monitor: None,
                        hwnd: hwnd_clicked.0.addr(),
                    }));
                }
            }
            WM_MBUTTONUP => {
                // println!("鼠标中键释放了桌面");
                if let Some(sender) = &*TX.lock().unwrap() {
                    let _ = sender.try_send(Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::MiddleUp,
                        monitor: None,
                        hwnd: hwnd_clicked.0.addr(),
                    }));
                }
            }
            WM_MOUSEMOVE => {
                // println!("鼠标移动了桌面");
                if let Some(sender) = &*TX.lock().unwrap() {
                    let _ = sender.try_send(Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::Move,
                        monitor: None,
                        hwnd: hwnd_clicked.0.addr(),
                    }));
                }
            }
            WM_MOUSEWHEEL => {
                // println!("鼠标滚轮滚动了桌面");
                if let Some(sender) = &*TX.lock().unwrap() {
                    let _ = sender.try_send(Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::Wheel,
                        monitor: None,
                        hwnd: hwnd_clicked.0.addr(),
                    }));
                }
            }
            _ => {
                if let Some(sender) = &*TX.lock().unwrap() {
                    let _ = sender.try_send(None);
                }
            }
        }
    }

    CallNextHookEx(Some(HOOK.unwrap_or_default()), n_code, w_param, l_param)
}
use tauri::Monitor;
pub fn desktop_mouse_listen(app: AppHandle) {
    unsafe {
        let w = app.get_webview_window("taskbar").unwrap();
        let mut h = w.hwnd().unwrap();
        h = FindWindowExA(Some(h), None, s!("WRY_WEBVIEW"), None).unwrap();
        h = FindWindowExA(Some(h), None, s!("Chrome_WidgetWin_0"), None).unwrap();
        h = FindWindowExA(Some(h), None, s!("Chrome_WidgetWin_1"), None).unwrap();
        h = FindWindowExA(Some(h), None, s!("Chrome_RenderWidgetHostHWND"), None).unwrap();
        let mut shell_dll_def_view: HWND = HWND(std::ptr::null_mut());
        let _ = EnumWindows(
            Some(enum_window),
            LPARAM(&mut shell_dll_def_view as *mut HWND as isize),
        );
        let mut sys_list_view32_hwnd = HWND(std::ptr::null_mut());
        let sys_list_view32 =
            FindWindowExA(Some(shell_dll_def_view), None, s!("SysListView32"), None);
        match sys_list_view32 {
            Ok(s) => {
                sys_list_view32_hwnd = s;
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
        let (tx, rx) = sync_channel::<Option<MouseInfo>>(200);
        let h_instance = GetModuleHandleA(None).unwrap();
        let _hook = SetWindowsHookExA(
            WH_MOUSE_LL,
            Some(mouse_proc),
            Some(HINSTANCE(h_instance.0)),
            0,
        )
        .unwrap();
        HOOK = Some(_hook);
        *TX.lock().unwrap() = Some(tx);

        let h = h.0.addr();
        let sys_list_view32_hwnd = sys_list_view32_hwnd.0.addr();
        let shell_dll_def_view = shell_dll_def_view.0.addr();
        tauri::async_runtime::spawn(async move {
            while let Ok(mouse) = rx.recv() {
                match mouse {
                    Some(mut mouse) => {
                        if mouse.hwnd == sys_list_view32_hwnd
                            || mouse.hwnd == shell_dll_def_view
                            || mouse.hwnd == h
                        {
                            let current = app
                                .monitor_from_point(mouse.x as f64, mouse.y as f64)
                                .unwrap();
                            mouse.monitor = current;
                            let _ = app.emit("desktop", mouse);
                        }
                        //  let current = app
                        //             .monitor_from_point(mouse.x as f64, mouse.y as f64)
                        //             .unwrap();
                        //         mouse.monitor = current;
                        //         let _ = app.emit("desktop", mouse);
                        // }
                        // else {
                        //     let current = app
                        //         .monitor_from_point(mouse.x as f64, mouse.y as f64)
                        //         .unwrap();
                        //     mouse.monitor = current;
                        //     let _ = app.emit("desktop", mouse);
                        // }
                    }
                    None => {}
                }
            }
        });
    }
}
