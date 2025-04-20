use tauri::{Emitter, Manager, Window};

#[tauri::command]
pub fn hovertop(app: Window, label: String, show: bool) {
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
