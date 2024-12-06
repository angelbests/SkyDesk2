
use std::{thread, time};
use sysinfo::{
    Networks,
    System
};
use tauri::{AppHandle, Emitter};
#[tauri::command]
pub fn netspeed(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut r: u64 = 0;
        let mut t: u64 = 0;
        let mut networks = Networks::new_with_refreshed_list();
        loop {
            thread::sleep(time::Duration::from_secs(1));
            networks.refresh(true);
            r = 0;
            t = 0;
            for (name, network) in &networks {
                // 排除 vEthernet Bridge Virtual Filter Qos 网络
                if (name.contains("vEthernet")
                    || name.contains("Bridge")
                    || name.contains("Virtual")
                    || name.contains("Filter")
                    || name.contains("Qos"))
                    == false
                {
                    r = r + network.received();
                    t = t + network.transmitted();
                }
            }
            let str = format!("{{\"speed_r\":{},\"speed_s\":{}}}", r, t);
            let _ = app.emit("netspeed", str);
        }
    });
}


#[tauri::command]
pub fn system(app: AppHandle){
    tauri::async_runtime::spawn(async move {
        let mut system = System::new_all();
        loop{
            // cpu
            system.refresh_all();
            let _ = app.emit("cpu",system.global_cpu_usage().to_string());
            // memory
            let used_memory = system.used_memory();
            let total_memory = system.total_memory();
            let _ = app.emit("memory", (used_memory as f32/total_memory as f32).to_string());
            thread::sleep(time::Duration::from_millis(1000));
        }
    });
}


