import { exists } from "@tauri-apps/plugin-fs"
// import { Command } from "@tauri-apps/plugin-shell"
import { openPath } from "@tauri-apps/plugin-opener"
export const exec = async function (item: any) {
  if (await exists(item.lnkPath)) openPath(item.lnkPath)
}
