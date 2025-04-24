use rdev::{grab, simulate, Button, Event, EventType, Key, SimulateError};
use tauri::{AppHandle, Emitter};
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
lazy_static! {
    static ref WHEEL_STATUS: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
}

pub fn wheelclick(window: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let callback = move |event: Event| -> Option<Event> {
            match event.event_type {
                EventType::ButtonPress(Button::Middle)
                | EventType::ButtonRelease(Button::Middle) => {
                    let status = WHEEL_STATUS.lock().unwrap();
                    if *status {
                        let s = format!("{:?}", event.event_type);
                        window.emit("wheel-click", Payload { message: s }).unwrap();
                        None
                    } else {
                        Some(event)
                    }
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

#[tauri::command]
pub fn wheel_status(bool: bool) {
    let mut status = WHEEL_STATUS.lock().unwrap();
    *status = bool;
}

#[tauri::command]
pub fn screen() {
    send(&EventType::KeyPress(Key::PrintScreen));
    send(&EventType::KeyRelease(Key::PrintScreen));
}

fn send(event_type: &EventType) {
    use std::{thread, time};
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}
