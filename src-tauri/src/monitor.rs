use std::sync::OnceLock;
use std::sync::{LazyLock, Mutex};
use tauri::{AppHandle, Emitter, Manager};
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, SetWindowLongPtrA, GWLP_WNDPROC, WM_DEVICECHANGE, WM_DISPLAYCHANGE,
};
static OLD_WNDPROC: OnceLock<isize> = OnceLock::new();
static APP_HANDLE: LazyLock<Mutex<Option<AppHandle>>> = LazyLock::new(|| Mutex::new(None));
// use windows::Win32::Graphics::Dwm::{
//     DwmSetWindowAttribute, DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_DONOTROUND,
// };
// pub fn disable_round_corner(hwnd: HWND) {
//     unsafe {
//         let preference: u32 = DWMWCP_DONOTROUND.0 as u32;
//         let _ = DwmSetWindowAttribute(
//             hwnd,
//             DWMWA_WINDOW_CORNER_PREFERENCE,
//             &preference as *const u32 as *const std::ffi::c_void,
//             std::mem::size_of_val(&preference) as u32,
//         );
//     }
// }

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DISPLAYCHANGE => {
            let bits_per_pixel = wparam.0 as u32;
            let width = (lparam.0 & 0xFFFF) as u32;
            let height = ((lparam.0 >> 16) & 0xFFFF) as u32;
            println!(
                "显示器分辨率/刷新率变化: {}x{} @ {} bpp",
                width, height, bits_per_pixel
            );
            let app = APP_HANDLE.lock().unwrap();
            if let Some(ref app_handle) = *app {
                app_handle.emit("DISPLAY_CHANGE", 1).ok();
            }
            LRESULT(0)
        }
        WM_DEVICECHANGE => {
            println!("检测到设备变化 (可能是显示器插拔)");
            LRESULT(0)
        }
        _ => unsafe {
            let old_proc = *OLD_WNDPROC.get().unwrap();
            CallWindowProcW(
                Some(std::mem::transmute(old_proc)),
                hwnd,
                msg,
                wparam,
                lparam,
            )
        },
    }
}

// 初始化显示器变化钩子，并发送消息给前端。
#[tauri::command]
pub fn init_window_hook(app: AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    let hwnd = window.hwnd().unwrap();
    unsafe {
        let old_proc = SetWindowLongPtrA(hwnd, GWLP_WNDPROC, wnd_proc as isize);
        OLD_WNDPROC.set(old_proc).ok();
    }
    *APP_HANDLE.lock().unwrap() = Some(app);
}
