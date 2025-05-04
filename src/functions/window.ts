import { WebviewWindow, getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow"
import { WindowOptions } from "@tauri-apps/api/window"
import { WebviewOptions } from "@tauri-apps/api/webview"
import { windowStore } from "../stores/window"
import { Monitor } from "@tauri-apps/api/window"
import { setWindowToMonitor } from "../functions/monitor"
import { UnlistenFn } from "@tauri-apps/api/event"
export const createWindow = async function (
  label: string,
  option: Omit<WebviewOptions, "x" | "y" | "width" | "height"> & WindowOptions,
  wallpaper = {
    x: 0,
    y: 0,
    w: 0,
    h: 0,
    z: 0,
    status: false,
    monitor: undefined as Monitor | undefined,
  },
) {
  const windowstore = windowStore()
  let indexbool = windowstore.windows.findIndex((item) => {
    return item.label === label
  })
  if (indexbool < 0) {
    windowstore.windows.push({
      label,
      option,
      wallpaper,
    })
    const w = new WebviewWindow(label, option)
    if (w.label.indexOf("wallpaper-") < 0) {
      let unlisten1 = await listenMove(w)
      let unlisten2 = await listenSize(w)
      listenClose(w, unlisten1, unlisten2)
    } else {
      await listenClose(w)
    }

    return w
  } else {
    return null
  }
}

export const initWindow = async function () {
  const windowstore = windowStore()
  windowstore.windows.forEach(async (e) => {
    let w = new WebviewWindow(e.label, {
      ...e.option,
    })
    let unlisten1 = await listenMove(w)
    let unlisten2 = await listenSize(w)
    listenClose(w, unlisten1, unlisten2)
    if (e.wallpaper.status) {
      setTimeout(() => {
        setWindowToMonitor(e.label, e.wallpaper.x, e.wallpaper.y, e.wallpaper.w, e.wallpaper.h, e.wallpaper.z)
      }, 100)
    }
  })
}

const listenMove = async function (w: WebviewWindow) {
  let label = w.label
  const factor = await getCurrentWebviewWindow().scaleFactor()
  let unlisten = await w.listen("tauri://move", function (event: any) {
    const windowstore = windowStore()
    let index = windowstore.windows.findIndex((item) => {
      return item.label === label
    })
    windowstore.windows[index].option.x = Math.ceil(event.payload.x / factor)
    windowstore.windows[index].option.y = Math.ceil(event.payload.y / factor)
  })
  return unlisten
}

const listenClose = async function (w: WebviewWindow, unlisten1?: UnlistenFn, unlisten2?: UnlistenFn) {
  let label = w.label
  let unlisten = await w.listen("tauri://close-requested", () => {
    const windowstore = windowStore()
    let index = windowstore.windows.findIndex((item) => {
      return item.label === label
    })
    windowstore.windows.splice(index, 1)
    if (unlisten1) unlisten1()
    if (unlisten2) unlisten2()
    unlisten()
    w.destroy()
  })
}

const listenSize = async function (w: WebviewWindow) {
  let label = w.label
  const factor = await getCurrentWebviewWindow().scaleFactor()
  let unlisten = await w.listen("tauri://resize", (event: any) => {
    const windowstore = windowStore()
    let index = windowstore.windows.findIndex((item) => {
      return item.label === label
    })
    windowstore.windows[index].option.width = Math.ceil(event.payload.width / factor)
    windowstore.windows[index].option.height = Math.ceil(event.payload.height / factor)
  })
  return unlisten
}
