use std::path::Path;
use tauri::{path::BaseDirectory, Error, Manager, Window};
use tauri_plugin_autostart::MacosLauncher;
mod icotopng;
mod server;
mod sysinfo;
mod taskbar;
mod wallpaper;
mod wheel;
use chrono::prelude::*;
mod audio;
mod clipboard;
mod desktop;
mod dropfile;
mod hovertop;
mod lockscreen;
mod monitor;
mod search;
mod shell;
mod smtc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let local: DateTime<Local> = Local::now();
    let t = local.format("%Y-%m-%d");
    tauri::Builder::default()
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
            desktop::focus_desktop(app.handle().clone());
            monitor::init_window_hook(app.handle().clone());
            // let main = app.get_webview_window("main").unwrap();
            // webview2禁用节能模式
            // main.with_webview(|webview| {
            //     let _controller = webview.controller();
            // })?;
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let main = app.get_webview_window("main").unwrap();
            let _ = main.show();
            let _ = main.set_focus();
        }))
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
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
            lockscreen::setlockscreen,
            wheel::wheel_status,
            smtc::play_control,
            hovertop::hovertop,
            search::search_query,
            wheel::screen,
            taskbar::gettaskbarlist,
            taskbar::closetaskbar,
            shell::get_lnk_png,
            shell::get_localized_display_name,
            shell::get_uwp,
            clipboard::copyfile,
            check_path_type,
            open_devtool
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn open_devtool(window: Window, label: String) -> Result<(), Error> {
    let webview = window
        .get_webview_window(&label)
        .ok_or(Error::WebviewNotFound)?;
    webview.open_devtools();
    return Ok(());
}

#[tauri::command]
fn check_path_type(path: String) -> String {
    let path = Path::new(&path);
    match path.metadata() {
        Ok(meta) => {
            if meta.is_file() {
                "file".into()
            } else if meta.is_dir() {
                "directory".into()
            } else {
                "other".into()
            }
        }
        Err(_) => "invalid".into(),
    }
}
