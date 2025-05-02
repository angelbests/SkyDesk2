import { MouseAction, MouseEvent } from "../types/desktopType"
import { listen, Event } from "@tauri-apps/api/event"
import { invoke } from "@tauri-apps/api/core"
import { wallpaperStore } from "../stores/wallpaper"
import { currentMonitor, Monitor, monitorFromPoint } from "@tauri-apps/api/window"
import { info } from "@tauri-apps/plugin-log"

//////////////////////////手势检测/////////////////////////////
export const desktopMouseControl = async function (id: string, index: number) {
  let mouseleftdown: {
    x: number
    y: number
    in: boolean
  } | null = null
  const wallpaperstore = wallpaperStore()
  let cancel_listen_desktop = await listen("desktop", async (e: Event<MouseEvent>) => {
    let dom = document.getElementById(id)
    if (!dom) return
    let monitor = (await monitorFromPoint(e.payload.x, e.payload.y)) as Monitor
    let current = await currentMonitor()
    if (monitor?.name != current?.name) {
      mouseleftdown = null
      return
    }

    let x = (e.payload.x - monitor.position.x) / monitor.scaleFactor
    let y = (e.payload.y - monitor.position.y) / monitor.scaleFactor
    let { left, top, right, bottom } = dom.getBoundingClientRect()
    if (left < x && right > x && top < y && bottom > y && e.payload.mouse == MouseAction.LeftDown) {
      mouseleftdown = {
        x: e.payload.x,
        y: e.payload.y,
        in: true,
      }
    }
    if (e.payload.mouse == MouseAction.LeftUp && mouseleftdown && mouseleftdown.in) {
      info(`up ${mouseleftdown.x}-${e.payload.x}-${e.payload.x - mouseleftdown.x}`)
      if (e.payload.x - mouseleftdown.x > 40) {
        invoke("play_control", { appname: wallpaperstore.wallpaperConfig[index].config.musicapp, control: 1 })
      } else if (e.payload.x - mouseleftdown.x < 40 && e.payload.x - mouseleftdown.x > -40) {
        invoke("play_control", { appname: wallpaperstore.wallpaperConfig[index].config.musicapp, control: 0 })
      } else if (e.payload.x - mouseleftdown.x < -40) {
        invoke("play_control", { appname: wallpaperstore.wallpaperConfig[index].config.musicapp, control: -1 })
      }
    }
    if (e.payload.mouse == MouseAction.LeftUp) {
      mouseleftdown = null
    }
  })
  return cancel_listen_desktop
}
