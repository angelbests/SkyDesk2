use std::ffi::OsStr;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tauri::Emitter;
use windows::{
    Foundation::TypedEventHandler,
    Media::{
        Control::{
            // CurrentSessionChangedEventArgs,
            GlobalSystemMediaTransportControlsSession,
            GlobalSystemMediaTransportControlsSessionManager,
            MediaPropertiesChangedEventArgs,
            PlaybackInfoChangedEventArgs,
            SessionsChangedEventArgs,
            TimelinePropertiesChangedEventArgs,
        },
        MediaPlaybackType,
    },
    Storage::Streams::{DataReader, IRandomAccessStreamWithContentType},
};

// use crate::audio;
// static mut COUNTER: u32 = 0;
#[derive(Clone, serde::Serialize)]
struct Mediapayload {
    app: String,
    title: String,
    album_title: String,
    artist: String,
    media_type: i32,
    thumb: Vec<u8>,
}

#[derive(Clone, serde::Serialize)]
struct Timeline {
    app: String,
    start: i64,
    position: i64,
    end: i64,
}

#[derive(Clone, serde::Serialize)]
struct Playstatus {
    app: String,
    status: i32,
}

#[derive(Clone, serde::Serialize)]
struct Musicapp {
    cloudmusic: bool,
    qqmusic: bool,
    spotify: bool,
}
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
lazy_static! {
    static ref WHEEL_STATUS: Arc<Mutex<Musicapp>> = Arc::new(Mutex::new(Musicapp {
        cloudmusic: false,
        qqmusic: false,
        spotify: false,
    }));
}

#[tauri::command]
pub fn smtc_listen(window: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let agsmtc = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap();
        let gsmtc = agsmtc.get().unwrap();
        get_sessions(&gsmtc, window.clone());
        let window1 = window.clone();
        let sessionchangehandler = TypedEventHandler::<
            GlobalSystemMediaTransportControlsSessionManager,
            SessionsChangedEventArgs,
        >::new(move |sender, _args| {
            println!("session change");
            if let Some(gsmtc) = sender {
                get_sessions(gsmtc, window1.clone());
            }
            Ok(())
        });

        let token = gsmtc.SessionsChanged(&sessionchangehandler).unwrap();
        println!("{:?}", token);

        // let currentsessionchangehandler = TypedEventHandler::<
        //     GlobalSystemMediaTransportControlsSessionManager,
        //     CurrentSessionChangedEventArgs,
        // >::new(move |sender, _args| {
        //     println!("currentsession change");
        //     if let Some(gsmtc) = sender {
        //         get_sessions(gsmtc, window1.clone());
        //     }
        //     Ok(())
        // });
        // let _ = gsmtc.CurrentSessionChanged(&currentsessionchangehandler);
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            checkappstatus(window.clone());
        }
    });
}

fn get_sessions(
    gsmtc: &GlobalSystemMediaTransportControlsSessionManager,
    window: tauri::AppHandle,
) {
    let sessions = gsmtc.GetSessions().unwrap();
    let mut musicapp = WHEEL_STATUS.lock().unwrap();
    for session in sessions {
        let appname = session.SourceAppUserModelId().unwrap().to_os_string();
        if appname == "cloudmusic.exe" && musicapp.cloudmusic == false {
            println!("appname:{:?}", appname);
            musicapp.cloudmusic = true;
            session_control(session, window.clone());
        } else if appname == "QQMusic.exe" && musicapp.qqmusic == false {
            println!("appname:{:?}", appname);
            musicapp.qqmusic = true;
            session_control(session, window.clone());
        } else if appname == "Spotify.exe" && musicapp.spotify == false {
            println!("appname:{:?}", appname);
            musicapp.spotify = true;
            session_control(session, window.clone());
        }
    }
}

fn session_control(session: GlobalSystemMediaTransportControlsSession, window: tauri::AppHandle) {
    // // 获取时间轴信息
    // let timeline = session.GetTimelineProperties();
    // match timeline {
    //     Ok(timeline) => {
    //         let start = timeline.StartTime().unwrap().Duration;
    //         let end = timeline.EndTime().unwrap().Duration;
    //         let position = timeline.Position().unwrap().Duration;
    //         let payload = Timeline {
    //             start,
    //             position,
    //             end,
    //         };
    //         let _ = window.emit("timeline", payload);
    //     }
    //     Err(e) => {
    //         println!("{:?}", e);
    //     }
    // }

    // // 获取媒体信息
    // let isession = session.TryGetMediaPropertiesAsync();
    // match isession {
    //     Ok(isession) => {
    //         let media = isession.get().unwrap();
    //         let album_title = media.AlbumTitle().unwrap();
    //         let artist = media.Artist().unwrap();
    //         let title = media.Title().unwrap();
    //         let match_media_type = media.PlaybackType();
    //         let media_type = match match_media_type {
    //             Ok(media_type) => media_type.Value().unwrap(),
    //             Err(e) => {
    //                 println!("media_type{:?}", e);
    //                 MediaPlaybackType::Music
    //             }
    //         };

    //         let mut data = vec![];
    //         let thumb = media.Thumbnail();
    //         match thumb {
    //             Ok(thumb) => {
    //                 let read = thumb.OpenReadAsync().unwrap();
    //                 let stream = read.get().unwrap();
    //                 data = get_thumb_data(stream);
    //             }
    //             Err(e) => {
    //                 println!("thumb_error:{:?}", e);
    //             }
    //         }
    //         let payload = Mediapayload {
    //             title: title.to_string(),
    //             artist: artist.to_string(),
    //             album_title: album_title.to_string(),
    //             media_type: media_type.0,
    //             thumb: data,
    //         };
    //         let _ = window.emit("media", payload);
    //     }
    //     Err(e) => {
    //         println!("{:?}", e);
    //     }
    // }

    // // 0 已关闭 1 已打开 2 正在更改 3 已停止 4 正在播放 5 已暂停
    // let playinfo = session.GetPlaybackInfo();
    // match playinfo {
    //     Ok(playinfo) => {
    //         let e = playinfo.PlaybackStatus().unwrap();
    //         let _ = window.emit("playstatus", e.0);
    //     }
    //     Err(e) => {
    //         println!("{:?}", e);
    //     }
    // }
    let window2 = window.clone();
    let window3 = window.clone();
    let appname = session
        .SourceAppUserModelId()
        .unwrap()
        .to_os_string()
        .to_string_lossy()
        .to_string();
    let appname1 = appname.clone();
    let appname2 = appname.clone();
    // 时间轴变化
    let timelinehandler = TypedEventHandler::<
        GlobalSystemMediaTransportControlsSession,
        TimelinePropertiesChangedEventArgs,
    >::new(move |sender, _args| {
        println!("timeline change");
        if let Some(gsmtc) = sender {
            let timeline = gsmtc.GetTimelineProperties();
            match timeline {
                Ok(timeline) => {
                    let start = timeline.StartTime().unwrap().Duration;
                    let end = timeline.EndTime().unwrap().Duration;
                    let position = timeline.Position().unwrap().Duration;
                    let payload = Timeline {
                        app: appname.clone(),
                        start,
                        position,
                        end,
                    };
                    let _ = window.emit("timeline", payload);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Ok(())
    });
    let _timelinetoken = session.TimelinePropertiesChanged(&timelinehandler).unwrap();

    // 媒体信息变化
    let mediahandler = TypedEventHandler::<
        GlobalSystemMediaTransportControlsSession,
        MediaPropertiesChangedEventArgs,
    >::new(move |sender, _args| {
        println!("media change");
        if let Some(session) = sender {
            let isession = session.TryGetMediaPropertiesAsync();
            match isession {
                Ok(isession) => {
                    let media = isession.get().unwrap();
                    let album_title = media.AlbumTitle().unwrap();
                    let artist = media.Artist().unwrap();
                    let title = media.Title().unwrap();
                    let match_media_type = media.PlaybackType();
                    let media_type = match match_media_type {
                        Ok(media_type) => media_type.Value().unwrap(),
                        Err(e) => {
                            println!("media_type{:?}", e);
                            MediaPlaybackType::Music
                        }
                    };

                    let mut data = vec![];
                    let thumb = media.Thumbnail();
                    match thumb {
                        Ok(thumb) => {
                            let read = thumb.OpenReadAsync().unwrap();
                            let stream = read.get().unwrap();
                            data = get_thumb_data(stream);
                        }
                        Err(e) => {
                            println!("thumb_error:{:?}", e);
                        }
                    }
                    let payload = Mediapayload {
                        app: appname1.clone(),
                        title: title.to_string(),
                        artist: artist.to_string(),
                        album_title: album_title.to_string(),
                        media_type: media_type.0,
                        thumb: data,
                    };
                    let _ = window2.emit("media", payload);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Ok(())
    });
    let _mediatoken = session.MediaPropertiesChanged(&mediahandler).unwrap();

    // 播放状态变化
    let playhandler = TypedEventHandler::<
        GlobalSystemMediaTransportControlsSession,
        PlaybackInfoChangedEventArgs,
    >::new(move |sender, _args| {
        println!("play change");
        if let Some(session) = sender {
            let playinfo = session.GetPlaybackInfo();
            match playinfo {
                Ok(playinfo) => {
                    let e = playinfo.PlaybackStatus().unwrap();
                    // 0 已关闭 1 已打开 2 正在更改 3 已停止 4 正在播放 5 已暂停]
                    println!("rust:播放状态{:?}", e.0);
                    let _ = window3.emit(
                        "playstatus",
                        Playstatus {
                            app: appname2.clone(),
                            status: e.0,
                        },
                    );
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Ok(())
    });
    let _playtoken = session.PlaybackInfoChanged(&playhandler);
}

// 将封面数据流取出
fn get_thumb_data(stream: IRandomAccessStreamWithContentType) -> Vec<u8> {
    let stream_len = stream.Size().unwrap() as usize;
    let mut data = vec![0u8; stream_len];
    let reader = DataReader::CreateDataReader(&stream).unwrap();
    reader.LoadAsync(stream_len as u32).unwrap().get().unwrap();
    reader.ReadBytes(&mut data).unwrap();
    reader.Close().ok();
    stream.Close().ok();
    return data;
}

// 检测程序退出
fn checkappstatus(window: tauri::AppHandle) {
    let mut musicapp = WHEEL_STATUS.lock().unwrap();
    let appnames = vec![
        "cloudmusic.exe".to_string(),
        "QQMusic.exe".to_string(),
        "Spotify.exe".to_string(),
    ];

    // "cloudmusic.exe" "QQMusic.exe" "Spotify.exe"
    let refreshes = RefreshKind::nothing().with_processes(ProcessRefreshKind::everything());
    let system = System::new_with_specifics(refreshes);
    for app in appnames {
        let process_ids = system.processes_by_name(OsStr::new(&app));
        if process_ids.count() == 0 {
            if app == "cloudmusic.exe" {
                musicapp.cloudmusic = false;
            } else if app == "QQMusic.exe" {
                musicapp.qqmusic = false;
            } else if app == "Spotify.exe" {
                musicapp.spotify = false;
            }
        }
    }
    let _ = window.emit("appstatus", musicapp.clone());
}
