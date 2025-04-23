import { exists } from "@tauri-apps/plugin-fs"
// import { Command } from "@tauri-apps/plugin-shell"
import { openPath } from "@tauri-apps/plugin-opener"
export const exec = async function (item: any) {
  await exists(item.lnkPath)
  if (item.lnkPath) {
    console.log(item)
    // let res = await Command.create("powershell", `& "${item.lnkPath}"`, {
    //   encoding: "GBK",
    // }).execute()
    openPath(item.lnkPath)
  } else {
    // await Command.create("powershell", item.targetPath).execute()
    openPath(item.targetPath)
  }
}
