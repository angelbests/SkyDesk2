import { openPath } from "@tauri-apps/plugin-opener"
import { ShortCut } from "../types/storeType"
import { invoke } from "@tauri-apps/api/core"
export const exec = async function (item: ShortCut) {
  if (item.type == "openPath") {
    openPath(item.lnkPath)
  } else {
    await invoke("launch_shell_item", { path: item.lnkPath })
  }
}
