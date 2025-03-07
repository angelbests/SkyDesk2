use tauri::Emitter;
use windows::{
    Foundation::TypedEventHandler,
    Media::Control::{
        // CurrentSessionChangedEventArgs,
        GlobalSystemMediaTransportControlsSession,
        GlobalSystemMediaTransportControlsSessionManager,
        MediaPropertiesChangedEventArgs,
        PlaybackInfoChangedEventArgs,
        SessionsChangedEventArgs,
        TimelinePropertiesChangedEventArgs,
    },
    Storage::Streams::{DataReader, IRandomAccessStreamWithContentType},
};

// use crate::audio;
// static mut COUNTER: u32 = 0;
#[derive(Clone, serde::Serialize)]
struct Mediapayload {
    title: String,
    album_title: String,
    artist: String,
    media_type: i32,
    thumb: Vec<u8>,
}

#[derive(Clone, serde::Serialize)]
struct Timeline {
    start: i64,
    position: i64,
    end: i64,
}

static mut CLOUDMUSIC: bool = false;
static mut QQMUSIC: bool = false;
#[tauri::command]
pub fn smtc_listen(window: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let agsmtc = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap();
        let gsmtc = agsmtc.get().unwrap();
        get_sessions(&gsmtc, window.clone());

        let sessionchangehandler = TypedEventHandler::<
            GlobalSystemMediaTransportControlsSessionManager,
            SessionsChangedEventArgs,
        >::new(move |sender, _args| {
            println!("session change");
            if let Some(gsmtc) = sender {
                get_sessions(gsmtc, window.clone());
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
        }
    });
}

fn get_sessions(
    gsmtc: &GlobalSystemMediaTransportControlsSessionManager,
    window: tauri::AppHandle,
) {
    let sessions = gsmtc.GetSessions().unwrap();
    let checksessions = gsmtc.GetSessions().unwrap();
    let mut cloudmusicstatus = false;
    let mut qqmusicstatus = false;
    for session in checksessions {
        let appname = session.SourceAppUserModelId().unwrap().to_os_string();
        if appname == "cloudmusic.exe" {
            cloudmusicstatus = true;
            // audio::process_audio_capture(window.clone(), appname.to_str().unwrap().to_string());
        } else if appname == "QQMusic.exe" {
            qqmusicstatus = true;
            // audio::process_audio_capture(window.clone(), appname.to_str().unwrap().to_string());
        }
    }
    if cloudmusicstatus == false {
        unsafe {
            CLOUDMUSIC = false;
        }
    }
    if qqmusicstatus == false {
        unsafe {
            QQMUSIC = false;
        }
    }
    for session in sessions {
        let appname = session.SourceAppUserModelId().unwrap().to_os_string();
        unsafe {
            if appname == "cloudmusic.exe" && CLOUDMUSIC == false {
                CLOUDMUSIC = true;
                println!("appname:{:?}", appname);
                session_control(session, window.clone());
            } else if appname == "QQMusic.exe" && QQMUSIC == false {
                QQMUSIC = true;
                println!("appname:{:?}", appname);
                session_control(session, window.clone());
            }
        }
    }
}

fn session_control(session: GlobalSystemMediaTransportControlsSession, window: tauri::AppHandle) {
    let timeline = session.GetTimelineProperties();
    match timeline {
        Ok(timeline) => {
            let start = timeline.StartTime().unwrap().Duration;
            let end = timeline.EndTime().unwrap().Duration;
            let position = timeline.Position().unwrap().Duration;
            let payload = Timeline {
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

    let isession = session.TryGetMediaPropertiesAsync();
    match isession {
        Ok(isession) => {
            let media = isession.get().unwrap();
            let album_title = media.AlbumTitle().unwrap();
            let artist = media.Artist().unwrap();
            let title = media.Title().unwrap();
            let media_type = media.PlaybackType().unwrap().Value().unwrap();

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
                title: title.to_string(),
                artist: artist.to_string(),
                album_title: album_title.to_string(),
                media_type: media_type.0,
                thumb: data,
            };
            let _ = window.emit("media", payload);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    let playinfo = session.GetPlaybackInfo();
    match playinfo {
        Ok(playinfo) => {
            let e = playinfo.PlaybackStatus().unwrap();
            // 0 已关闭 1 已打开 2 正在更改 3 已停止 4 正在播放 5 已暂停
            let _ = window.emit("playstatus", e.0);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    let window2 = window.clone();
    let window3 = window.clone();
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
                    let media_type = media.PlaybackType().unwrap().Value().unwrap();

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
                    let _ = window3.emit("playstatus", e.0);
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
