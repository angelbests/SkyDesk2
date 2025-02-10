use std::thread::sleep;
use std::time::Duration;
use tauri::{Emitter, Window};
use windows::{
    core::s,
    Win32::UI::WindowsAndMessaging::{FindWindowA, GetWindowTextLengthW, GetWindowTextW},
};
#[tauri::command]
pub fn get_cliudmusic_name(window: Window) {
    tauri::async_runtime::spawn(async move {
        unsafe {
            loop {
                let reshwnd = FindWindowA(s!("OrpheusBrowserHost"), None);
                match reshwnd {
                    Ok(hwnd) => {
                        if hwnd.is_invalid() {
                            continue;
                        }
                        let length = GetWindowTextLengthW(hwnd);
                        if length == 0 {
                            continue;
                        }
                        let mut data = vec![0u16; (length + 1) as usize];
                        let r = GetWindowTextW(hwnd, &mut data);
                        if r == 0 {
                            continue;
                        }
                        let _ = window.emit(
                            "musicname",
                            String::from_utf16_lossy(&data[..length as usize]),
                        );
                        sleep(Duration::from_secs(1));
                        // println!("{:?}", hwnd);
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
        }
    });
}
