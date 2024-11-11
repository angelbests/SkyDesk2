use rdev::{grab, Button, Event, EventType};
use tauri::{AppHandle, Emitter};

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
pub fn wheelclick(window: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let _callback = move |event: Event| -> Option<Event> {
            if let EventType::ButtonPress(Button::Middle) = event.event_type {
                let s = format!("{:?}", event.event_type);
                window
                    .emit("wheel-click", Payload { message: s.into() })
                    .unwrap();
                None
            } else if let EventType::ButtonRelease(Button::Middle) = event.event_type {
                let s = format!("{:?}", event.event_type);
                window
                    .emit("wheel-click", Payload { message: s.into() })
                    .unwrap();
                None
            } else if let EventType::MouseMove {x,y}= event.event_type {
                let s = format!("{{\"x\":{:?},\"y\":{:?}}}", x,y);
                window
                    .emit("mouse-move", Payload { message: s})
                    .unwrap();
                return Some(event);
            } else {
                Some(event)
            }
        };
        // let _a = grab(_callback);
        if let Err(error) = grab(_callback) {
            println!("Error: {:?}", error);
        } else {
            // window.emit("event-name", Payload { message: "Tauri is awesome!".into() }).unwrap();
        }
    });
}