use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;
mod capture;
mod icotopng;
mod server;
mod wallpaper;
mod wheel;
mod taskbar;
mod netspeed;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
            server::open_server,
            icotopng::ico_to_png,
            wheel::wheelclick,
            capture::get_display_capture,
            capture::get_window_capture,
            capture::start_capture,
            netspeed::netspeed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

