import { WindowOptions } from "@tauri-apps/api/window"
import { WebviewOptions } from "@tauri-apps/api/webview"
import { Monitor } from "@tauri-apps/api/window"
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
    action: true
    sakura: true
    audio: number
    date: boolean
    datex: number
    datey: number
    color: string
    datefontsize: number
    time: boolean
    timex: number
    timey: number
    timefontsize: number
    weather: boolean
    weatherx: number
    weathery: number
    weatherfontsize: number
    netspeed: boolean
    netspeedx: number
    netspeedy: number
    netspeedfontsize: number
    cpu: boolean
    cpux: number
    cpuy: number
    cpufontsize: number
    memory: boolean
    memoryx: number
    memoryy: number
    memoryfontsize: number
    music: boolean
    musictype: number
    musicx: number
    musicy: number
    musicfontsize: number
    musicapp: string
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
  targetPath: string
  iconLocationPeFile: string
  iconLocation: string
  lnkPath: string
  icoPath: string
  name: string
}

export type { Windows, Note, WallpaperConfig, WallpaperList, ShortCut, Monitor }
