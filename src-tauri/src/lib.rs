use tauri::{path::BaseDirectory, Manager};
use tauri_plugin_autostart::MacosLauncher;
mod capture;
mod icotopng;
mod server;
mod sysinfo;
mod taskbar;
mod wallpaper;
mod wheel;
use chrono::prelude::*;
mod audio;
mod desktop;
mod hovertop;
mod lockscreen;
mod smtc;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let local: DateTime<Local> = Local::now();
    let t = local.format("%Y-%m-%d");
    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(move |app| {
            let apphandle = app.handle();

            wheel::wheelclick(apphandle.clone());
            sysinfo::netspeed(apphandle.clone());
            sysinfo::system(apphandle.clone());
            taskbar::listentaskbar(apphandle.clone());
            smtc::smtc_listen(apphandle.clone());
            audio::default_audio_capture(apphandle.clone());
            let path = app
                .path()
                .resolve("wallpapers\\html", BaseDirectory::AppData)
                .unwrap();
            server::open_server(path.to_str().unwrap().to_string(), 12345);
            desktop::desktop_mouse_listen(app.handle().clone());
            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some(t.to_string()),
                    },
                ))
                .build(),
        )
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![""]),
        ))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let main = app.get_webview_window("main").unwrap();
            let _ = main.show();
            let _ = main.set_focus();
        }))
        .invoke_handler(tauri::generate_handler![
            wallpaper::setwallpaper,
            wallpaper::cancelwallpaper,
            icotopng::ico_to_png,
            icotopng::zipimage,
            capture::start_capture,
            lockscreen::setlockscreen,
            wheel::wheel_status,
            smtc::play_control,
            hovertop::hovertop
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
