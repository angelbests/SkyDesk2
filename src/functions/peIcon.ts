import { appDataDir, basename, resolve } from "@tauri-apps/api/path"
import { invoke } from "@tauri-apps/api/core"
import { ShortCut } from "../types/storeType"
import { getlnks } from "./lnks"
import { uuid } from "."

export const setIcon2 = async function () {
  let arr: ShortCut[] = []
  let lnks = await getlnks()
  let path = await resolve(await appDataDir(), "lnk")
  for (let i = 0; i < lnks.length; i++) {
    let name = await basename(lnks[i].lnkPath)
    let savepath = `${path}\\${name.split(".")[0]}-${i}.png`

    await invoke("get_lnk_png", {
      path: lnks[i].lnkPath,
      savepath: savepath,
      width: 64,
      height: 64,
    })

    let displayName: string = await invoke("get_localized_display_name", { path: lnks[i].lnkPath })
    arr.push({
      name: displayName,
      lnkPath: lnks[i].lnkPath,
      icoPath: savepath,
      targetPath: lnks[i].targetPath,
    })
  }
  return arr
}

export const get_pe_ico = async function (pepath: string, type: "lnk" | "ico" = "lnk") {
  let path = ""
  let name = ""
  if (type == "lnk") {
    path = await resolve(await appDataDir(), "lnk")
    name = await basename(pepath)
  } else if (type == "ico") {
    path = await resolve(await appDataDir(), "ico")
    name = uuid()
  }
  let savepath = `${path}\\${name.split(".")[0]}.png`
  await invoke("get_lnk_png", {
    path: pepath,
    savepath: savepath,
    width: 64,
    height: 64,
  })
  return savepath
}
