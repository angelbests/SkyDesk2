import { resourceDir } from "@tauri-apps/api/path"
import { TrayIcon, TrayIconEvent } from "@tauri-apps/api/tray"
import { getAllWebviewWindows, getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow"
import { systemStore } from "./../stores/system"
import { info } from "@tauri-apps/plugin-log"
import { invoke } from "@tauri-apps/api/core"
export const createtray = async function () {
  let icodir = await resourceDir()
  await TrayIcon.new({
    tooltip: "SkyDesk2",
    icon: icodir + "\\icons\\icon.ico",
    action: async (e: TrayIconEvent) => {
      if (e.type == "Click") {
        if (e.button == "Left") {
          getCurrentWebviewWindow().show()
          getCurrentWebviewWindow().setFocus()
        } else if (e.button == "Right") {
          let all = await getAllWebviewWindows()
          all.filter(async (item) => {
            if (item.label == "tray") {
              let size = await item.outerSize()
              e.position.y = Math.trunc(e.position.y - size.height - 10)
              item.setPosition(e.position)
              item.show()
              item.setFocus()
              item.listen("tauri://blur", (_e) => {
                item.hide()
              })
            }
          })
        }
      }
    },
  })
  info("托盘初始化完成")
}

export const traystart = async function () {
  // 启动到托盘
  const systemstore = systemStore()
  const appWindow = getCurrentWebviewWindow()
  if (!systemstore.traystart && appWindow.label == "main") {
    await appWindow.show()
  }
}

export const wheelstart = async function () {
  const systemstore = systemStore()
  invoke("wheel_status", { bool: systemstore.wheel })
}
