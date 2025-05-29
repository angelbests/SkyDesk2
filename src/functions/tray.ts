import { resourceDir } from "@tauri-apps/api/path"
import { TrayIcon, TrayIconEvent } from "@tauri-apps/api/tray"
import { getCurrentWebviewWindow, WebviewWindow } from "@tauri-apps/api/webviewWindow"
import { systemStore } from "./../stores/system"
import { info } from "@tauri-apps/plugin-log"
export const createtray = async function () {
  let icodir = await resourceDir()
  await TrayIcon.new({
    tooltip: "SkyDesk2",
    icon: icodir + "\\icons\\icon.ico",
    action: async (e: TrayIconEvent) => {
      if (e.type == "Click") {
        if (e.button == "Left") {
          await getCurrentWebviewWindow().show()
          await getCurrentWebviewWindow().setFocus()
        } else if (e.button == "Right") {
          const tray = new WebviewWindow("tray", {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            shadow: false,
            decorations: false,
            transparent: true,
            alwaysOnTop: true,
            url: "/#/pages/desktop/tray",
            title: "tray",
            skipTaskbar: true,
            resizable: false,
            minimizable: false,
            maximizable: false,
            visible: false,
          })
          let size = await tray.outerSize()
          e.position.y = Math.trunc(e.position.y - size.height - 10)
          await tray.setPosition(e.position)
          await tray.show()
          await tray.setFocus()
        }
      }
    },
  })
  info("托盘初始化完成")
}

export const traystart = async function () {
  // 启动到托盘
  const systemstore = systemStore()
  if (!systemstore.traystart && getCurrentWebviewWindow().label == "main") {
    await getCurrentWebviewWindow().show()
  }
}
