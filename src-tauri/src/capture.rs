use lazy_static::lazy_static;
use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
    time::Instant,
};
use tauri::{AppHandle, Listener, Manager};
use windows_capture::{
    capture::GraphicsCaptureApiHandler,
    encoder::{AudioSettingsBuilder, ContainerSettingsBuilder, VideoEncoder, VideoSettingsBuilder},
    frame::Frame,
    graphics_capture_api::InternalCaptureControl,
    monitor::Monitor,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
};
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

lazy_static! {
    static ref CAPTURE_STATUS: Arc<Mutex<String>> = Arc::new(Mutex::new(String::from("")));
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
pub fn start_capture(
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

use xcap::Monitor as xMonitor;
#[tauri::command]
pub fn get_display_capture(display: String, path: String) {
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
pub fn get_window_capture(app: AppHandle, label: String) {
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
