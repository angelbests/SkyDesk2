use rand::Rng;
use std::ffi::OsStr;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tauri::{path::BaseDirectory, Emitter, Manager};
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
    thumb: String,
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
            if let Some(gsmtc) = sender.as_ref() {
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
            // } else if appname == "msedgewebview2.exe" {
        }
    }
}

fn session_control(session: GlobalSystemMediaTransportControlsSession, window: tauri::AppHandle) {
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
        if let Some(gsmtc) = sender.as_ref() {
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
        if let Some(session) = sender.as_ref() {
            let isession = session.TryGetMediaPropertiesAsync();
            match isession {
                Ok(isession) => {
                    let mediares = isession.get();
                    let media = match mediares {
                        Ok(s) => s,
                        Err(e) => {
                            println!("mediares_error:{:?}", e);
                            return Ok(());
                        }
                    };

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

                    let mut path: String = String::from("");
                    let thumb = media.Thumbnail();
                    match thumb {
                        Ok(thumb) => {
                            let read = thumb.OpenReadAsync().unwrap();
                            let stream = read.get().unwrap();
                            let data = get_thumb_data(stream);
                            let pathbuf = window2
                                .path()
                                .resolve(
                                    String::from("wallpapers\\temp\\")
                                        + &random_string(10)
                                        + ".png",
                                    BaseDirectory::AppData,
                                )
                                .unwrap();

                            let _ = std::fs::write(&pathbuf, &data);
                            path = pathbuf.to_string_lossy().to_string();
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
                        thumb: path,
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
        if let Some(session) = sender.as_ref() {
            let playinfo = session.GetPlaybackInfo();
            match playinfo {
                Ok(playinfo) => {
                    let e = playinfo.PlaybackStatus().unwrap();
                    // 0 已关闭 1 已打开 2 正在更改 3 已停止 4 正在播放 5 已暂停]
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

// 控制音乐的播放 暂停 下一曲 上一曲 -1 上一曲 0 播放暂停toogle切换 1 下一曲
#[tauri::command]
pub fn play_control(appname: String, control: i32) {
    tauri::async_runtime::spawn(async move {
        let agsmtc = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap();
        let gsmtc = agsmtc.get().unwrap();
        let smtcs = gsmtc.GetSessions().unwrap();
        println!("{:?}+{:?}", appname, control);
        for smtc in smtcs {
            let name = smtc
                .SourceAppUserModelId()
                .unwrap()
                .to_string_lossy()
                .to_string();
            println!("{:?}", name);
            if name == appname {
                if control == -1 {
                    let _ = smtc.TrySkipPreviousAsync();
                } else if control == 0 {
                    let _ = smtc.TryTogglePlayPauseAsync();
                } else if control == 1 {
                    let _ = smtc.TrySkipNextAsync();
                }
            }
        }
    });
}

// 随机字符串
fn random_string(len: usize) -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::rng();

    (0..len)
        .map(|_| {
            charset
                .chars()
                .nth(rng.random_range(0..charset.len()))
                .unwrap()
        })
        .collect()
}
