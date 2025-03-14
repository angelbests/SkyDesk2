import { Event, listen, UnlistenFn } from "@tauri-apps/api/event"

type TimeLine = {
  app: string
  start: number
  position: number
  end: number
}

type MusicApps = {
  cloudmusic: boolean
  qqmusic: boolean
  spotify: boolean
}

type Media = {
  app: string
  title: string
  artist: string
  album_title: string
  media_type: number
  thumb: string
}

// 0 已关闭 1 已打开 2 正在更改 3 已停止 4 正在播放 5 已暂停
type PlayStatus = {
  app: string
  status: number
}
class Smtc_Control {
  media_unlisten: UnlistenFn | undefined
  timeline_unlisten: UnlistenFn | undefined
  apps_unlisten: UnlistenFn | undefined
  playstatus_unlisten: UnlistenFn | undefined

  async listen_timeline(fn: (e: Event<TimeLine>) => void) {
    this.timeline_unlisten = await listen("timeline", (e: Event<TimeLine>) => {
      fn(e)
    })
  }

  cancel_listen_timeline() {
    if (this.timeline_unlisten) {
      this.timeline_unlisten()
      this.timeline_unlisten = undefined
    }
  }

  async listen_appstatus(fn: (e: Event<MusicApps>) => void) {
    this.apps_unlisten = await listen("appstatus", (e: Event<MusicApps>) => {
      fn(e)
    })
  }

  cancel_listen_apps() {
    if (this.apps_unlisten) {
      this.apps_unlisten()
      this.apps_unlisten = undefined
    }
  }

  async listen_media(fn: (e: Event<Media>) => void) {
    this.media_unlisten = await listen("media", (e: Event<Media>) => {
      fn(e)
    })
  }

  cancel_listen_media() {
    if (this.media_unlisten) {
      this.media_unlisten()
      this.media_unlisten = undefined
    }
  }

  async listen_playstatus(fn: (e: Event<PlayStatus>) => void) {
    this.playstatus_unlisten = await listen("playstatus", (e: Event<PlayStatus>) => {
      fn(e)
    })
  }

  cancel_listen_playstatus() {
    if (this.playstatus_unlisten) {
      this.playstatus_unlisten()
      this.playstatus_unlisten = undefined
    }
  }
}

export { Smtc_Control }
export type { Event, Media, MusicApps, TimeLine }
