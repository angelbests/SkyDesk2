use std::collections::VecDeque;
use std::error::{self};
// use std::ffi::OsStr;
use std::sync::mpsc;
use std::thread;
// use std::time::Duration;
// use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tauri::Emitter;
use wasapi::*;
// static mut APP_STATUS: bool = false;
type Res<T> = Result<T, Box<dyn error::Error>>;

// fn capture_loop(
//     tx_capt: std::sync::mpsc::SyncSender<Vec<u8>>,
//     chunksize: usize,
//     process_id: u32,
// ) -> Res<()> {
//     // Chunks是音频流中的连续小块，可以用于分析、处理或播放音频。Samples是音频信号的离散时间点上的振幅值，其数量取决于采样率。Frames是由一系列采样点组成的连续时间片段，其大小取决于采样率和块大小
//     // 初始化 COM 以供多线程单元 （MTA） 的调用线程使用。
//     initialize_mta().ok().unwrap();
//     //WAVEFORMAT 结构描述波形音频数据的格式。 此结构中仅包含所有波形音频数据格式通用的格式信息。
//     let desired_format = WaveFormat::new(16, 16, &SampleType::Int, 32000, 2, None);
//     // BlockAlign： 2字节小端序。大小等于NumChannels * BitsPerSample / 8， 表示每帧的多通道总字节数。
//     let blockalign = desired_format.get_blockalign();
//     // PROCESS_LOOPBACK_MODE 如果为 true，则环回捕获客户端将从目标进程及其所有子进程捕获音频。 如果为 false，则仅捕获来自目标进程的音频。
//     let include_tree = true;

//     // 特定进程创建环回捕获 AudioClient。
//     let mut audio_client = AudioClient::new_application_loopback_client(process_id, include_tree)?;
//     // let (_def_time, min_time) = audio_client.get_periods()?;
//     // 为给定的方向、共享模式和格式初始化 [IAudioClient]。 设置为 true 将启用自动采样率和格式转换，这意味着几乎接受任何格式
//     audio_client.initialize_client(
//         &desired_format, // 波形音频数据的格式
//         0,
//         &Direction::Capture, // Direction::Capture 录制 Direction::Render 播放
//         &ShareMode::Shared,  // ShareMode::Shared 分享  ShareMode::Exclusive 独占
//         true,                // 转换
//     )?;

//     //  [IAudioClient] 创建并返回事件句柄
//     let h_event = audio_client.set_get_eventhandle().unwrap();

//     // 获取捕获客户端
//     let capture_client = audio_client.get_audiocaptureclient().unwrap();

//     // just eat the reallocation because querying the buffer size gives massive values.
//     // 双端队列数据结构
//     let mut sample_queue: VecDeque<u8> = VecDeque::new();

//     // [IAudioClient] 上启动流
//     audio_client.start_stream().unwrap();

//     loop {
//         while sample_queue.len() > (blockalign as usize * chunksize) {
//             let mut chunk = vec![0u8; blockalign as usize * chunksize];
//             for element in chunk.iter_mut() {
//                 *element = sample_queue.pop_front().unwrap();
//             }
//             tx_capt.send(chunk).unwrap();
//         }
//         // 在共享模式下获取下一个数据包中的帧数
//         let new_frames = capture_client.get_next_nbr_frames()?.unwrap_or(0);
//         // saturating_sub 大减小 100-50 = 50  小减大  20-30=0
//         // capacity 些容器预先分配的内存可以存储多少个元素
//         // reserve 提前增加集合类型（如 Vec<T> 或 VecDeque<T>）容量的方法
//         let additional = (new_frames as usize * blockalign as usize)
//             .saturating_sub(sample_queue.capacity() - sample_queue.len());
//         sample_queue.reserve(additional);
//         if new_frames > 0 {
//             capture_client
//                 .read_from_device_to_deque(&mut sample_queue)
//                 .unwrap();
//         }
//         if h_event.wait_for_event(5000).is_err() {
//             audio_client.stop_stream().unwrap();
//             break;
//         }
//         unsafe {
//             // print!("{:?}", APP_STATUS);
//             if APP_STATUS {
//                 APP_STATUS = false;
//                 break;
//             }
//         }
//     }
//     Ok(())
// }

// #[tauri::command]
// pub fn process_audio_capture(window: tauri::AppHandle, appname: String) {
//     checkapp(appname.clone());
//     tauri::async_runtime::spawn(async move {
//         let refreshes = RefreshKind::nothing().with_processes(ProcessRefreshKind::everything());
//         let system = System::new_with_specifics(refreshes);
//         // QQMusic.exe cloudmusic.exe
//         let binding = appname.clone();
//         let process_ids = system.processes_by_name(OsStr::new(&binding));
//         let mut process_id = 0;
//         for process in process_ids {
//             process_id = process.parent().unwrap_or(process.pid()).as_u32();
//         }

//         let (tx_capt, rx_capt): (
//             std::sync::mpsc::SyncSender<Vec<u8>>,
//             std::sync::mpsc::Receiver<Vec<u8>>,
//         ) = mpsc::sync_channel(2);
//         let chunksize = 256;

//         // Capture
//         let _handle = thread::Builder::new()
//             .name("Capture".to_string())
//             .spawn(move || {
//                 let result = capture_loop(tx_capt, chunksize, process_id);
//                 if let Err(err) = result {
//                     println!("{:?}1", err);
//                 }
//             });
//         loop {
//             match rx_capt.recv() {
//                 Ok(chunk) => {
//                     let _ = window.emit("audio_chunk", chunk);
//                 }
//                 Err(err) => {
//                     println!("{:?}2", err);
//                     break;
//                 }
//             }
//         }
//     });
// }

// pub fn checkapp(appname: String) {
//     tauri::async_runtime::spawn(async move {
//         loop {
//             let refreshes = RefreshKind::nothing().with_processes(ProcessRefreshKind::everything());
//             let system = System::new_with_specifics(refreshes);
//             // QQMusic.exe cloudmusic.exe
//             let binding = appname.clone();
//             let process_ids = system.processes_by_name(OsStr::new(&binding));
//             let count = process_ids.count();
//             unsafe {
//                 println!("{:?}", count);
//                 if count > 0 {
//                     APP_STATUS = false;
//                 } else {
//                     APP_STATUS = true;
//                     break;
//                 }
//             }
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
// }

pub fn default_audio_capture(window: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        initialize_mta().ok().unwrap();

        let (tx_capt, rx_capt): (
            std::sync::mpsc::SyncSender<Vec<u8>>,
            std::sync::mpsc::Receiver<Vec<u8>>,
        ) = mpsc::sync_channel(2);
        let chunksize = 1024;

        let _handle = thread::Builder::new()
            .name("Capture".to_string())
            .spawn(move || {
                let result = capture_loop2(tx_capt, chunksize);
                if let Err(err) = result {
                    println!("{:?}", err);
                }
            });

        loop {
            match rx_capt.recv() {
                Ok(chunk) => {
                    let _ = window.emit("audio_chunk", chunk);
                }
                Err(err) => {
                    println!("{:?}", err);
                    break;
                }
            }
        }
    });
}

fn capture_loop2(tx_capt: std::sync::mpsc::SyncSender<Vec<u8>>, chunksize: usize) -> Res<()> {
    let device = get_default_device(&Direction::Render)?;
    let mut audio_client = device.get_iaudioclient()?;
    println!("{:?}", device.get_friendlyname());
    let desired_format = WaveFormat::new(16, 16, &SampleType::Int, 32000, 2, None);
    let blockalign = desired_format.get_blockalign();
    let (_def_time, min_time) = audio_client.get_periods()?;
    audio_client.initialize_client(
        &desired_format,
        min_time,
        &Direction::Capture,
        &ShareMode::Shared,
        true,
    )?;

    let h_event = audio_client.set_get_eventhandle()?;

    let buffer_frame_count = audio_client.get_bufferframecount()?;

    let render_client = audio_client.get_audiocaptureclient()?;
    let mut sample_queue: VecDeque<u8> = VecDeque::with_capacity(
        100 * blockalign as usize * (1024 + 2 * buffer_frame_count as usize),
    );
    let _session_control = audio_client.get_audiosessioncontrol()?;
    audio_client.start_stream()?;

    loop {
        while sample_queue.len() > (blockalign as usize * chunksize) {
            let mut chunk = vec![0u8; blockalign as usize * chunksize];
            for element in chunk.iter_mut() {
                *element = sample_queue.pop_front().unwrap();
            }
            tx_capt.send(chunk)?;
        }

        render_client.read_from_device_to_deque(&mut sample_queue)?;
        if h_event.wait_for_event(3000).is_err() {
            audio_client.stop_stream()?;
            break;
        }
    }
    Ok(())
}
