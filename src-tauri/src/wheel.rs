use rdev::{grab, simulate, Button, Event, EventType, Key, SimulateError};
use tauri::{AppHandle, Emitter};
#[derive(Clone, serde::Serialize)]
struct Mouse {
    x: i32,
    y: i32,
}
use std::sync::{LazyLock, Mutex};
static WHEEL_STATUS: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(true));

pub fn wheelclick(window: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let callback = move |event: Event| -> Option<Event> {
            match event.event_type {
                EventType::ButtonPress(Button::Middle)
                | EventType::ButtonRelease(Button::Middle) => {
                    let status = WHEEL_STATUS.lock().ok()?;
                    if *status {
                        let s = format!("{:?}", event.event_type);
                        window.emit("wheel-click", s).ok()?;
                        None
                    } else {
                        Some(event)
                    }
                }
                EventType::MouseMove { x, y } => {
                    window
                        .emit(
                            "mouse-move",
                            Mouse {
                                x: x as i32,
                                y: y as i32,
                            },
                        )
                        .ok()?;
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
pub fn wheel_status(bool: bool) -> Result<(), String> {
    let mut status = WHEEL_STATUS.lock().map_err(|e| e.to_string())?;
    *status = bool;
    Ok(())
}

#[tauri::command]
pub fn screen() -> Result<(), String> {
    send(&EventType::KeyPress(Key::PrintScreen)).map_err(|e| e.to_string())?;
    send(&EventType::KeyRelease(Key::PrintScreen)).map_err(|e| e.to_string())?;
    Ok(())
}

fn send(event_type: &EventType) -> Result<(), SimulateError> {
    use std::{thread, time::Duration};
    simulate(event_type)?;
    thread::sleep(Duration::from_millis(20));
    Ok(())
}
