use tauri::{AppHandle, Emitter, Manager};
use windows::core::{s, BOOL};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, EnumWindows, FindWindowExA, SetWindowsHookExA, WindowFromPoint, HC_ACTION,
    HHOOK, MSLLHOOKSTRUCT, WH_MOUSE_LL, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP,
    WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP,
};

#[derive(Clone, serde::Serialize)]
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
}
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::{LazyLock, Mutex};
static mut H: HWND = HWND(std::ptr::null_mut());
static TX: LazyLock<Mutex<Option<SyncSender<Option<MouseInfo>>>>> =
    LazyLock::new(|| Mutex::new(None));
static TX2: LazyLock<Mutex<Option<SyncSender<Option<MouseInfo>>>>> =
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
        if hwnd_clicked == sys_list_view32_hwnd
            || hwnd_clicked == shell_dll_def_view
            || hwnd_clicked == H
        {
            match w_param.0 as u32 {
                WM_LBUTTONDOWN => {
                    println!("鼠标左键点击了桌面");
                    if let Some(sender) = &*TX.lock().unwrap() {
                        let _ = sender.send(Some(MouseInfo {
                            x: mouse_info.pt.x,
                            y: mouse_info.pt.y,
                            mouse: MouseAction::LeftDown,
                        }));
                    }
                }
                WM_RBUTTONDOWN => {
                    // println!("鼠标右键点击了桌面");
                    if let Some(sender) = &*TX.lock().unwrap() {
                        let _ = sender.send(Some(MouseInfo {
                            x: mouse_info.pt.x,
                            y: mouse_info.pt.y,
                            mouse: MouseAction::RightDown,
                        }));
                    }
                }
                WM_MBUTTONDOWN => {
                    // println!("鼠标中键点击了桌面");
                    if let Some(sender) = &*TX.lock().unwrap() {
                        let _ = sender.send(Some(MouseInfo {
                            x: mouse_info.pt.x,
                            y: mouse_info.pt.y,
                            mouse: MouseAction::MiddleDown,
                        }));
                    }
                }
                WM_LBUTTONUP => {
                    // println!("鼠标左键释放了桌面");
                    if let Some(sender) = &*TX.lock().unwrap() {
                        let _ = sender.send(Some(MouseInfo {
                            x: mouse_info.pt.x,
                            y: mouse_info.pt.y,
                            mouse: MouseAction::LeftUp,
                        }));
                    }
                }
                WM_RBUTTONUP => {
                    // println!("鼠标右键释放了桌面");
                    if let Some(sender) = &*TX.lock().unwrap() {
                        let _ = sender.send(Some(MouseInfo {
                            x: mouse_info.pt.x,
                            y: mouse_info.pt.y,
                            mouse: MouseAction::RightUp,
                        }));
                    }
                }
                WM_MBUTTONUP => {
                    // println!("鼠标中键释放了桌面");
                    if let Some(sender) = &*TX.lock().unwrap() {
                        let _ = sender.send(Some(MouseInfo {
                            x: mouse_info.pt.x,
                            y: mouse_info.pt.y,
                            mouse: MouseAction::MiddleUp,
                        }));
                    }
                }
                WM_MOUSEMOVE => {
                    // println!("鼠标移动了桌面");
                    if let Some(sender) = &*TX.lock().unwrap() {
                        let _ = sender.send(Some(MouseInfo {
                            x: mouse_info.pt.x,
                            y: mouse_info.pt.y,
                            mouse: MouseAction::Move,
                        }));
                    }
                }
                WM_MOUSEWHEEL => {
                    // println!("鼠标滚轮滚动了桌面");
                    if let Some(sender) = &*TX.lock().unwrap() {
                        let _ = sender.send(Some(MouseInfo {
                            x: mouse_info.pt.x,
                            y: mouse_info.pt.y,
                            mouse: MouseAction::Wheel,
                        }));
                    }
                }
                _ => {
                    if let Some(sender) = &*TX.lock().unwrap() {
                        let _ = sender.send(None);
                    }
                }
            }
        }

        match w_param.0 as u32 {
            WM_LBUTTONDOWN => {
                println!("鼠标左键点击了桌面");
                if let Some(sender) = &*TX2.lock().unwrap() {
                    let _ = sender.send(Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::LeftDown,
                    }));
                }
            }
            WM_RBUTTONDOWN => {
                // println!("鼠标右键点击了桌面");
                if let Some(sender) = &*TX2.lock().unwrap() {
                    let _ = sender.send(Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::RightDown,
                    }));
                }
            }
            WM_MBUTTONDOWN => {
                // println!("鼠标中键点击了桌面");
                if let Some(sender) = &*TX2.lock().unwrap() {
                    let _ = sender.send(Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::MiddleDown,
                    }));
                }
            }
            WM_LBUTTONUP => {
                // println!("鼠标左键释放了桌面");
                if let Some(sender) = &*TX2.lock().unwrap() {
                    let _ = sender.send(Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::LeftUp,
                    }));
                }
            }
            WM_RBUTTONUP => {
                // println!("鼠标右键释放了桌面");
                if let Some(sender) = &*TX2.lock().unwrap() {
                    let _ = sender.send(Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::RightUp,
                    }));
                }
            }
            WM_MBUTTONUP => {
                // println!("鼠标中键释放了桌面");
                if let Some(sender) = &*TX2.lock().unwrap() {
                    let _ = sender.send(Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::MiddleUp,
                    }));
                }
            }
            WM_MOUSEMOVE => {
                // println!("鼠标移动了桌面");
                if let Some(sender) = &*TX2.lock().unwrap() {
                    let _ = sender.send(Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::Move,
                    }));
                }
            }
            WM_MOUSEWHEEL => {
                // println!("鼠标滚轮滚动了桌面");
                if let Some(sender) = &*TX2.lock().unwrap() {
                    let _ = sender.send(Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::Wheel,
                    }));
                }
            }
            _ => {
                if let Some(sender) = &*TX2.lock().unwrap() {
                    let _ = sender.send(None);
                }
            }
        }
    }

    CallNextHookEx(Some(HOOK.unwrap_or_default()), n_code, w_param, l_param)
}
pub fn desktop_mouse_listen(app: AppHandle) {
    unsafe {
        let w = app.get_webview_window("taskbar").unwrap();
        let mut h = w.hwnd().unwrap();
        h = FindWindowExA(Some(h), None, s!("WRY_WEBVIEW"), None).unwrap();
        h = FindWindowExA(Some(h), None, s!("Chrome_WidgetWin_0"), None).unwrap();
        h = FindWindowExA(Some(h), None, s!("Chrome_WidgetWin_1"), None).unwrap();
        h = FindWindowExA(Some(h), None, s!("Chrome_RenderWidgetHostHWND"), None).unwrap();
        H = h;

        let app2 = app.clone();
        let (tx, rx) = sync_channel::<Option<MouseInfo>>(100);
        *TX.lock().unwrap() = Some(tx);
        std::thread::spawn(move || {
            while let Ok(mouse) = rx.recv() {
                let _ = app.emit("desktop", mouse);
            }
        });

        let (tx2, rx2) = sync_channel::<Option<MouseInfo>>(100);
        *TX2.lock().unwrap() = Some(tx2);
        std::thread::spawn(move || {
            while let Ok(mouse) = rx2.recv() {
                let _ = app2.emit("desktop2", mouse);
            }
        });
        // *APP_HANDLE.lock().unwrap() = Some(app);
        let h_instance = GetModuleHandleA(None).unwrap();
        let _hook = SetWindowsHookExA(
            WH_MOUSE_LL,
            Some(mouse_proc),
            Some(HINSTANCE(h_instance.0)),
            0,
        )
        .unwrap();
        HOOK = Some(_hook);
    }
}
