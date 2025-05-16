// use tauri::{AppHandle, Manager};
// use windows::Win32::{
//     Foundation::{HWND, LPARAM, LRESULT, WPARAM},
//     UI::{
//         Shell::DragFinish,
//         WindowsAndMessaging::{
//             CallWindowProcW, DefWindowProcW, SetWindowLongPtrW, GWLP_WNDPROC, WM_DROPFILES, WNDPROC,
//         },
//     },
// };
// pub fn dropfile(app: AppHandle) {
//     let main = app.get_window("main").unwrap();
//     let w = tauri::WindowBuilder::new(&app, "drop")
//         // .parent_raw(main.hwnd().unwrap())
//         .transparent(false)
//         .decorations(true)
//         .resizable(false)
//         .skip_taskbar(true)
//         .visible(true)
//         .drag_and_drop(false)
//         .shadow(false)
//         .build()
//         .unwrap();

//     // let _ = w.set_ignore_cursor_events(true);
//     let h = w.hwnd().unwrap();
//     unsafe { subclass_window(h) };
//     main.on_window_event(move |event| {
//         if let tauri::WindowEvent::Moved(pos) = event {
//             let _ = w.set_position(*pos);
//         }

//         if let tauri::WindowEvent::Resized(size) = event {
//             let _ = w.set_size(*size);
//         }
//     });
// }

// static mut ORIGINAL_WNDPROC: Option<WNDPROC> = None;

// extern "system" fn custom_wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
//     match msg {
//         WM_DROPFILES => unsafe {
//             use windows::Win32::UI::Shell::{DragQueryFileW, HDROP};
//             let hdrop = HDROP(wparam.0 as _);
//             let file_count = DragQueryFileW(hdrop, 0xFFFFFFFF, None);

//             for i in 0..file_count {
//                 let mut buf = [0u16; 260];
//                 let len = DragQueryFileW(hdrop, i, Some(&mut buf));
//                 if len > 0 {
//                     let filename = String::from_utf16_lossy(&buf[..len as usize]);
//                     println!("拖入文件 [{}]: {}", i, filename);
//                 }
//             }
//             DragFinish(hdrop);
//             LRESULT(0)
//         },
//         _ => unsafe {
//             // 调用原始窗口过程
//             if let Some(orig) = ORIGINAL_WNDPROC {
//                 CallWindowProcW(orig, hwnd, msg, wparam, lparam)
//             } else {
//                 DefWindowProcW(hwnd, msg, wparam, lparam)
//             }
//         },
//     }
// }

// unsafe fn subclass_window(hwnd: HWND) {
//     let prev_wndproc = SetWindowLongPtrW(hwnd, GWLP_WNDPROC, custom_wndproc as _);
//     ORIGINAL_WNDPROC = Some(std::mem::transmute(prev_wndproc));
// }
