use std::{
    // ffi::OsString,
    // os::windows::ffi::{OsStrExt, OsStringExt},
    thread,
    time,
};
use sysinfo::{Networks, System};
use tauri::{AppHandle, Emitter};
#[tauri::command]
pub fn netspeed(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut _r: u64 = 0;
        let mut _t: u64 = 0;
        let mut networks = Networks::new_with_refreshed_list();
        loop {
            thread::sleep(time::Duration::from_secs(1));
            networks.refresh(true);
            _r = 0;
            _t = 0;
            for (name, network) in &networks {
                // 排除 vEthernet Bridge Virtual Filter Qos 网络
                if (name.contains("vEthernet")
                    || name.contains("Bridge")
                    || name.contains("Virtual")
                    || name.contains("Filter")
                    || name.contains("Qos"))
                    == false
                {
                    _r = _r + network.received();
                    _t = _t + network.transmitted();
                }
            }
            let str = format!("{{\"speed_r\":{},\"speed_s\":{}}}", _r, _t);
            let _ = app.emit("netspeed", str);
        }
    });
}

#[tauri::command]
pub fn system(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut system = System::new_all();
        loop {
            // cpu
            system.refresh_all();
            let _ = app.emit("cpu", system.global_cpu_usage().to_string());
            // memory
            let used_memory = system.used_memory();
            let total_memory = system.total_memory();
            let _ = app.emit(
                "memory",
                (used_memory as f32 / total_memory as f32).to_string(),
            );
            thread::sleep(time::Duration::from_millis(1000));
        }
    });
}

// use windows::{core::*, Win32::Foundation::*, Win32::System::Performance::*};

// // #[tauri::command] app: AppHandle
// pub fn phd() {
//     unsafe {
//         // 打开性能计数器并获取句柄
//         let mut query: isize = 0;
//         let query_handle = &mut query;
//         let res = PdhOpenQueryW(PCWSTR::null(), 0, query_handle);
//         if res != ERROR_SUCCESS.0 {
//             println!("error1:{:?}", res);
//             return;
//         } else {
//             println!("query_handle:{:?}", *query_handle);
//         }
//         let mut buffer_size = 0;

//         PdhEnumObjectsW(
//             PCWSTR::null(),
//             PCWSTR::null(),
//             PWSTR::null(),
//             &mut buffer_size,
//             PERF_DETAIL_NOVICE,
//             TRUE,
//         );
//         println!("buffer_size:{:?}", buffer_size);
//         let mut buffer: Vec<u16> = Vec::with_capacity(buffer_size as usize);
//         let res = PdhEnumObjectsW(
//             PCWSTR::null(),
//             PCWSTR::null(),
//             PWSTR::from_raw(buffer.as_mut_ptr()),
//             &mut buffer_size,
//             PERF_DETAIL_NOVICE,
//             TRUE,
//         );
//         if res != ERROR_SUCCESS.0 {
//             println!("error3:{:?}", res);
//             return;
//         } else {
//             println!("{:?}", buffer);
//             buffer.set_len(buffer_size as usize);
//             let str = String::from_utf16_lossy(&buffer);
//             let objects: Vec<&str> = str.split('\0').collect();
//             for object in objects.iter() {
//                 if object.is_empty() {
//                     continue;
//                 }
//                 if *object != "Processor" {
//                     continue;
//                 }
//                 println!("Counter Object: {}", object);
//                 let os_string = OsString::from(object);
//                 let utf16: Vec<u16> = os_string.encode_wide().collect();
//                 let object_pcwstr = PCWSTR(utf16.as_ptr());
//                 let mut counter_buffer_size = 0;
//                 let mut instance_buffer_size = 0;

//                 PdhEnumObjectItemsW(
//                     PCWSTR::null(),
//                     PCWSTR::null(),
//                     object_pcwstr,
//                     PWSTR::null(),
//                     &mut counter_buffer_size,
//                     PWSTR::null(),
//                     &mut instance_buffer_size,
//                     PERF_DETAIL_NOVICE,
//                     0,
//                 );
//                 println!("{:?}-{:?}", counter_buffer_size, instance_buffer_size);
//                 let mut counter_buffer: Vec<u16> = Vec::with_capacity(counter_buffer_size as usize);
//                 let mut instance_buffer: Vec<u16> =
//                     Vec::with_capacity(instance_buffer_size as usize);

//                 PdhEnumObjectItemsW(
//                     PCWSTR::null(),
//                     PCWSTR::null(),
//                     object_pcwstr,
//                     PWSTR::from_raw(counter_buffer.as_mut_ptr()),
//                     &mut counter_buffer_size,
//                     PWSTR::from_raw(instance_buffer.as_mut_ptr()),
//                     &mut instance_buffer_size,
//                     PERF_DETAIL_NOVICE,
//                     0,
//                 );

//                 counter_buffer.set_len(counter_buffer_size as usize);
//                 instance_buffer.set_len(instance_buffer_size as usize);

//                 let counter_str = String::from_utf16_lossy(&counter_buffer);
//                 let instance_str = String::from_utf16_lossy(&instance_buffer);

//                 for counter in counter_str.split("\0") {
//                     if counter.is_empty() {
//                         continue;
//                     }
//                     println!("counter:{:?}", counter);
//                 }

//                 for instance in instance_str.split("\0") {
//                     if instance.is_empty() {
//                         continue;
//                     }
//                     println!("instance:{:?}", instance)
//                 }
//             }
//         }
//         let mut counter: isize = 0;
//         let counter_handle = &mut counter;
//         let os_string = OsString::from("\\Processor Information(_Total)\\% Processor Time");
//         let utf16: Vec<u16> = os_string.encode_wide().collect();
//         let counter_path = PCWSTR(utf16.as_ptr());
//         let res = PdhAddCounterW(*query_handle, counter_path, 0, counter_handle);
//         if res != ERROR_SUCCESS.0 {
//             println!("error2:{:?}", res);
//             return;
//         }

//         // 收集数据
//         let res = PdhCollectQueryData(*query_handle);
//         if res != ERROR_SUCCESS.0 {
//             println!("error3:{:?}", res);
//             return;
//         }

//         std::thread::sleep(std::time::Duration::from_secs(1));

//         let res = PdhCollectQueryData(*query_handle);
//         if res != ERROR_SUCCESS.0 {
//             println!("error3:{:?}", res);
//             return;
//         }
//         // 获取格式化后的计数器值
//         let mut counter_value: PDH_FMT_COUNTERVALUE = PDH_FMT_COUNTERVALUE::default();
//         let res =
//             PdhGetFormattedCounterValue(*counter_handle, PDH_FMT_DOUBLE, None, &mut counter_value);
//         if res != ERROR_SUCCESS.0 {
//             println!("error4:{:?}", res);
//             return;
//         }

//         // 输出 CPU 使用率
//         if counter_value.CStatus == PDH_CSTATUS_VALID_DATA {
//             println!("CPU Usage: {:.2}%", counter_value.Anonymous.doubleValue);
//         } else {
//             eprintln!("Invalid data received.");
//         }

//         // 释放资源
//         PdhCloseQuery(*query_handle);
//     }
// }
