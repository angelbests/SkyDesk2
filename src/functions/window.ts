import { WebviewWindow, getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { availableMonitors, WindowOptions } from '@tauri-apps/api/window'
import { WebviewOptions } from '@tauri-apps/api/webview'
import { windowStore } from '../stores/window'
import { Monitor } from '@tauri-apps/api/window'
import { setWindowToMonitor } from '../functions/monitor'
import { UnlistenFn } from '@tauri-apps/api/event'
export const createWindow = async function (
  label: string,
  option: Omit<WebviewOptions, 'x' | 'y' | 'width' | 'height'> & WindowOptions,
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
    if (w.label.indexOf('wallpaper-') < 0) {
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
  for (let i = 0; i < windowstore.windows.length; i++) {
    let { option, label, wallpaper } = windowstore.windows[i]
    let monitor = (await availableMonitors()).filter((item) => item.name == wallpaper.monitor?.name)
    if (wallpaper.status) {
      if (monitor.length == 1) {
        let w = new WebviewWindow(label, {
          ...option,
        })
        await listenClose(w)
        setTimeout(() => {
          setWindowToMonitor(label, monitor[0].position.x, monitor[0].position.y, monitor[0].size.width, monitor[0].size.height, wallpaper.z)
        }, 20)
      }
    } else {
      let w = new WebviewWindow(label, {
        ...option,
      })
      let unlisten1 = await listenMove(w)
      let unlisten2 = await listenSize(w)
      await listenClose(w, unlisten1, unlisten2)
    }
  }
}

const listenMove = async function (w: WebviewWindow) {
  let label = w.label
  const factor = await getCurrentWebviewWindow().scaleFactor()
  let unlisten = await w.listen('tauri://move', function (event: any) {
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
  let unlisten = await w.listen('tauri://close-requested', () => {
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
  let unlisten = await w.listen('tauri://resize', (event: any) => {
    const windowstore = windowStore()
    let index = windowstore.windows.findIndex((item) => {
      return item.label === label
    })
    windowstore.windows[index].option.width = Math.ceil(event.payload.width / factor)
    windowstore.windows[index].option.height = Math.ceil(event.payload.height / factor)
  })
  return unlisten
}
