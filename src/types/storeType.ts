import { WindowOptions } from "@tauri-apps/api/window"
import { WebviewOptions } from "@tauri-apps/api/webview"
import { Monitor } from "@tauri-apps/api/window"
import { Festival } from "./../functions/calendar"
type Windows = {
  label: string
  monitor?: boolean
  option: Omit<WebviewOptions, "x" | "y" | "width" | "height"> & WindowOptions
  wallpaper: {
    x: number
    y: number
    w: number
    h: number
    z: number
    status: boolean
    monitor: Monitor | undefined
  }
}

type Note = {
  value: string
  date: string
  color: string
  opacity: number
  label: string
  wallpaper: boolean
  option?: Omit<WebviewOptions, "x" | "y" | "width" | "height"> & WindowOptions
}

type WallpaperConfig = {
  monitor: string
  label: string
  config: {
    // 动效
    action: boolean
    sakura: boolean
    // 声音
    audio: number
    // 日期
    date: boolean
    datex: number
    datey: number
    datecolor: string
    dateshadow: boolean
    // 天气
    weather: boolean
    weatherx: number
    weathery: number
    weatherd7: number
    weathershadow: boolean
    // 音乐
    music: boolean
    musicx: number
    musicy: number
    musictype: number
    musicapp: string
    musicshadow: boolean
    // 计时
    festivalcountdown: boolean
    festivalcountdownx: number
    festivalcountdowny: number
    festivals: Festival[] // Define or import the Festival type
    // calendar
    calendar: boolean
    calendarx: number
    calendary: number
    calendarshadow: boolean
    calendarcolor: string
  }
}

type WallpaperList = {
  type: "image" | "video" | "html"
  title: string
  preview: string // 预览图
  filename: string // 文件名称
  path: string
}

type ShortCut = {
  type: "cmd" | "openPath"
  lnkPath: string
  icoPath: string
  name: string
  targetPath: string
}

export type { Windows, Note, WallpaperConfig, WallpaperList, ShortCut, Monitor }
