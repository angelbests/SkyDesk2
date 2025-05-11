import { openPath } from "@tauri-apps/plugin-opener"
import { ShortCut } from "../types/storeType"
import { Command } from "@tauri-apps/plugin-shell"
export const exec = async function (item: ShortCut) {
  if (item.type == "openPath") {
    openPath(item.lnkPath)
  } else {
    Command.create("powershell", [item.lnkPath]).execute()
  }
}
