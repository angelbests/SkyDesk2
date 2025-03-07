use rdev::{grab, Button, Event, EventType};
use tauri::{AppHandle, Emitter};

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
pub fn wheelclick(window: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let callback = move |event: Event| -> Option<Event> {
            match event.event_type {
                EventType::ButtonPress(Button::Middle)
                | EventType::ButtonRelease(Button::Middle) => {
                    let s = format!("{:?}", event.event_type);
                    window.emit("wheel-click", Payload { message: s }).unwrap();
                    None
                }
                EventType::MouseMove { x, y } => {
                    let s = format!("{{\"x\":{:?},\"y\":{:?}}}", x, y);
                    window.emit("mouse-move", Payload { message: s }).unwrap();
                    Some(event)
                }
                _ => Some(event),
            }
        };

        if let Err(error) = grab(callback) {
            println!("Error: {:?}", error);
        }
    });
}
