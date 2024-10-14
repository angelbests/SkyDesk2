use image::ImageReader;
use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{
    io::{self, Write},
    time::Instant,
};
use tauri::Manager;
use tauri::WebviewWindow;
use tauri::{AppHandle, Emitter, Listener};
use tiny_http::{Response, Server};
mod platforms;
use lazy_static::lazy_static;
use rdev::{grab, Button, Event, EventType};
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::RECT;
use windows::Win32::Foundation::TRUE;
use windows::Win32::Graphics::Gdi;
use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::Graphics::Gdi::HMONITOR;
use windows::Win32::Graphics::Gdi::MONITORINFOEXA;
use windows_capture::{
    capture::GraphicsCaptureApiHandler,
    encoder::{AudioSettingsBuilder, ContainerSettingsBuilder, VideoEncoder, VideoSettingsBuilder},
    frame::Frame,
    graphics_capture_api::InternalCaptureControl,
    monitor::Monitor,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
};
use xcap::Monitor as xMonitor;
lazy_static! {
    static ref CAPTURE_STATUS: Arc<Mutex<String>> = Arc::new(Mutex::new(String::from("")));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init()) //get_display_capture,
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = show_window(app);
        }))
        .invoke_handler(tauri::generate_handler![
            a,
            b,
            get_all_monitors,
            open_server,
            ico_to_png,
            get_display_capture,
            start_capture,
            wheelclick,
            get_window_capture,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn show_window(app: &AppHandle) {
    let main = app.get_webview_window("main").unwrap();
    let _ = main.show();
    let _ = main.set_focus();
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}
#[tauri::command]
fn wheelclick(window: AppHandle) {
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
            } else if let EventType::MouseMove { x: _, y: _ } = event.event_type {
                let s = format!("{:?}", event.event_type);
                window
                    .emit("mouse-move", Payload { message: s.into() })
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

struct Flagset {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    path: String,
}
struct Capture {
    encoder: Option<VideoEncoder>,
    start: Instant,
    flag: Flagset,
}

impl GraphicsCaptureApiHandler for Capture {
    type Flags = Flagset;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    fn new(flagset: Self::Flags) -> Result<Self, Self::Error> {
        let encoder = VideoEncoder::new(
            VideoSettingsBuilder::new(flagset.width, flagset.height),
            AudioSettingsBuilder::default().disabled(true),
            ContainerSettingsBuilder::default(),
            flagset.path.clone(),
        )?;
        Ok(Self {
            encoder: Some(encoder),
            start: Instant::now(),
            flag: flagset,
        })
    }
    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        print!(
            "\rRecording for: {} seconds",
            self.start.elapsed().as_secs()
        );
        io::stdout().flush()?;
        let time = frame.timespan().Duration;
        let mut framecrop = frame
            .buffer_crop(
                self.flag.x,
                self.flag.y,
                self.flag.x + self.flag.width,
                self.flag.y + self.flag.height,
            )
            .unwrap();
        let w = framecrop.width() as usize;
        let h = framecrop.height() as usize;
        let nopad = framecrop.as_raw_nopadding_buffer().unwrap();
        let b = convert_rgba_to_bgra_and_flip(&nopad, w, h);
        self.encoder.as_mut().unwrap().send_frame_buffer(&b, time)?; //纯视频
        let mut status = CAPTURE_STATUS.lock().unwrap();
        println!("{:?}", *status);
        if *status == "\"STOP\"".to_string() {
            *status = "".to_string();
            self.encoder.take().unwrap().finish()?;
            capture_control.stop();
        }
        Ok(())
    }
    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Capture Session Closed");
        Ok(())
    }
}

fn convert_rgba_to_bgra_and_flip(rgba_data: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut bgra_data = vec![0; rgba_data.len()];
    for y in 0..height {
        for x in 0..width {
            let rgba_index = (y * width + x) * 4;
            let bgra_index = ((height - y - 1) * width + x) * 4;
            // 转换为 BGRA
            bgra_data[bgra_index] = rgba_data[rgba_index + 2]; // B
            bgra_data[bgra_index + 1] = rgba_data[rgba_index + 1]; // G
            bgra_data[bgra_index + 2] = rgba_data[rgba_index]; // R
            bgra_data[bgra_index + 3] = rgba_data[rgba_index + 3]; // A
        }
    }
    bgra_data
}

#[tauri::command]
fn start_capture(
    app: AppHandle,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    monitorname: String,
    path: String,
) {
    tauri::async_runtime::spawn(async move {
        let flagset = Flagset {
            x,
            y,
            width,
            height,
            path,
        };
        let monitors = Monitor::enumerate().unwrap();
        let mut monitor: Monitor = Monitor::primary().unwrap();
        for m in monitors {
            if m.device_name().unwrap() == monitorname {
                monitor = m;
            }
        }

        let settings = Settings::new(
            monitor,
            CursorCaptureSettings::Default,
            DrawBorderSettings::WithoutBorder,
            ColorFormat::Rgba8,
            flagset,
        );
        Capture::start_free_threaded(settings).unwrap();
        app.listen("capture", |event| {
            let mut stauts = CAPTURE_STATUS.lock().unwrap();
            *stauts = event.payload().to_string();
        });
        // Capture::start(settings).expect("Screen Capture Failed");
    });
}
/////////////////////////////////////////

#[tauri::command]
fn a(app: AppHandle, label: String, x: i32, y: i32, w: i32, h: i32) {
    let webview = app.get_webview_window(&label).unwrap();
    attach(&webview, x, y, w, h);
}

#[tauri::command]
fn b(app: AppHandle, label: String) {
    let webview = app.get_webview_window(&label).unwrap();
    detach(&webview);
}

pub fn attach(window: &WebviewWindow, x: i32, y: i32, w: i32, h: i32) {
    #[cfg(target_os = "windows")]
    {
        let hwnd = window.hwnd().unwrap();
        platforms::windows::attach(hwnd, x, y, w, h);
    }
}

pub fn detach(window: &WebviewWindow) {
    #[cfg(target_os = "windows")]
    {
        let hwnd = window.hwnd().unwrap();
        platforms::windows::detach(hwnd);
        return;
    }
}

// 获取当前屏幕截屏

#[tauri::command]
fn get_display_capture(display: String, path: String) {
    tauri::async_runtime::spawn(async move {
        let monitors = xMonitor::all().unwrap();
        for monitor in monitors {
            if display == monitor.name() {
                let image = monitor.capture_image().unwrap();
                image.save(path.clone()).unwrap();
            }
        }
    });
}

#[tauri::command]
fn get_window_capture(app: AppHandle, label: String) {
    let webview = app.get_webview_window(&label).unwrap();
    let hwnd = webview.hwnd().unwrap();
    let windows = xcap::Window::all().unwrap();
    for w in windows {
        println!("{:?},{:?},{:?}", w.id(), hwnd.0, w.title());
        if w.id() == hwnd.0 as u32 {
            println!("{label}");
            let image = w.capture_image().unwrap();
            image.save(format!("target/window.png",)).unwrap();
        }
    }
}

// 获取屏幕信息

#[tauri::command]
fn get_all_monitors() {
    let mut monitors = Vec::<MONITORINFOEXA>::new();
    unsafe {
        let _ = Gdi::EnumDisplayMonitors(
            None,
            Some(std::ptr::null()),
            Some(callback),
            LPARAM(&mut monitors as *mut _ as isize),
        );
    }
}

extern "system" fn callback(
    handle: HMONITOR,
    _hdc: HDC,
    _rect: *mut RECT,
    ref_monitors: LPARAM,
) -> BOOL {
    unsafe {
        let monitors = &mut (*(ref_monitors.0 as *mut Vec<MONITORINFOEXA>));
        let mut info = MONITORINFOEXA::default();
        info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXA>() as _;
        let _ = Gdi::GetMonitorInfoA(handle, &mut info.monitorInfo);
        monitors.push(info);
        TRUE
    }
}

#[tauri::command]
fn open_server(str: String) {
    tauri::async_runtime::spawn(async move {
        let server = Server::http("127.0.0.1:12345").unwrap();
        for request in server.incoming_requests() {
            println!(
                "received request! method: {:?}, url: {:?}",
                request.method(),
                request.url(),
            );
            let mut urlstr = request.url();
            if urlstr == "/" {
                urlstr = "/index.html"
            }
            // if urlstr == "/favicon.ico" {
            //     continue;
            // }
            let sstr = str.clone() + urlstr;
            println!("{:?}", sstr);
            let file = File::open(&Path::new(&sstr));
            match file {
                Ok(s) => {
                    let response = Response::from_file(s);
                    let _ = request.respond(response);
                }
                Err(_e) => {
                    let response = Response::empty(404);
                    let _ = request.respond(response);
                }
            }
        }
    });
}

// ico to png

#[tauri::command]
fn ico_to_png(from: String, to: String) -> i32 {
    let mut bool = 0;
    let img = ImageReader::open(from.clone()).unwrap();
    let de = img.decode();
    match de {
        Ok(d) => {
            d.save(to.clone()).unwrap();
            bool = 1;
        }
        _ => {
            println!("{to},{from}");
        }
    }
    bool
}