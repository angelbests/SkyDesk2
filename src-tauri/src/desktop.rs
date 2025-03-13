use tauri::{AppHandle, Emitter};
use windows::core::s;
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, FindWindowA, FindWindowExA, SetWindowsHookExA, WindowFromPoint, HC_ACTION,
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
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
lazy_static! {
    static ref APP_HANDLE: Arc<Mutex<Option<AppHandle>>> = Arc::new(Mutex::new(None));
}

static mut HOOK: Option<HHOOK> = None;
// static mut APP_HANDLE: Option<AppHandle> = None;
unsafe extern "system" fn mouse_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if (n_code as u32) == HC_ACTION {
        let mouse_info = *(l_param.0 as *const MSLLHOOKSTRUCT);
        let hwnd_clicked = WindowFromPoint(mouse_info.pt);
        let progman = FindWindowA(s!("Progman"), None).unwrap();
        let shell_dll_def_view =
            FindWindowExA(progman, None, s!("SHELLDLL_DefView"), None).unwrap();
        let sys_list_view32_hwnd =
            FindWindowExA(shell_dll_def_view, None, s!("SysListView32"), None).unwrap();
        let mouse: Option<MouseInfo>;
        // println!(
        //     "{:?},{:?},{:?}",
        //     mouse_info.pt, hwnd_clicked, sys_list_view32_hwnd
        // );
        if hwnd_clicked == sys_list_view32_hwnd || hwnd_clicked == shell_dll_def_view {
            match w_param.0 as u32 {
                WM_LBUTTONDOWN => {
                    println!("鼠标左键点击了桌面");
                    mouse = Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::LeftDown,
                    });
                }
                WM_RBUTTONDOWN => {
                    // println!("鼠标右键点击了桌面");
                    mouse = Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::RightDown,
                    });
                }
                WM_MBUTTONDOWN => {
                    // println!("鼠标中键点击了桌面");
                    mouse = Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::MiddleDown,
                    });
                }
                WM_LBUTTONUP => {
                    // println!("鼠标左键释放了桌面");
                    mouse = Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::LeftUp,
                    });
                }
                WM_RBUTTONUP => {
                    // println!("鼠标右键释放了桌面");
                    mouse = Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::RightUp,
                    });
                }
                WM_MBUTTONUP => {
                    // println!("鼠标中键释放了桌面");
                    mouse = Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::MiddleUp,
                    });
                }
                WM_MOUSEMOVE => {
                    // println!("鼠标移动了桌面");
                    mouse = Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::Move,
                    });
                }
                WM_MOUSEWHEEL => {
                    // println!("鼠标滚轮滚动了桌面");
                    mouse = Some(MouseInfo {
                        x: mouse_info.pt.x,
                        y: mouse_info.pt.y,
                        mouse: MouseAction::Wheel,
                    });
                }
                _ => mouse = None,
            }

            let _ = APP_HANDLE
                .lock()
                .unwrap()
                .as_ref()
                .unwrap()
                .emit("desktop", mouse);
        }
    }

    CallNextHookEx(HOOK.unwrap_or_default(), n_code, w_param, l_param)
}

pub fn desktop_mouse_listen(app: AppHandle) {
    unsafe {
        *APP_HANDLE.lock().unwrap() = Some(app);
        let h_instance = GetModuleHandleA(None).unwrap();
        let _hook = SetWindowsHookExA(WH_MOUSE_LL, Some(mouse_proc), h_instance, 0).unwrap();
        HOOK = Some(_hook);
    }
}
