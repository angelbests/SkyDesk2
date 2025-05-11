import { readDir } from "@tauri-apps/plugin-fs"
import { extname, homeDir, resolve } from "@tauri-apps/api/path"
import { info, error } from "@tauri-apps/plugin-log"
import { Command } from "@tauri-apps/plugin-shell"
import { ShortCut } from "../types/storeType"

export const getlnks = async function () {
  let lnks: ShortCut[] = []
  let lnkFiles = await getLnkFile()
  // 拼接shell脚本
  let lnkFilesstr = "$lnkFiles = @("
  for (let i = 0; i < lnkFiles.length; i++) {
    lnkFilesstr = lnkFilesstr + `\"${lnkFiles[i]}\"`
    if (i == lnkFiles.length - 1) {
      lnkFilesstr = lnkFilesstr + ");"
    } else {
      lnkFilesstr = lnkFilesstr + ","
    }
  }
  let forstr =
    lnkFilesstr +
    `
        $shell = New-Object -ComObject WScript.Shell;
        $results = @();
        foreach ($lnkFile in $lnkFiles) {
            $shortcut = $shell.CreateShortcut($lnkFile);
            $targetPath = $shortcut.TargetPath;
            $iconLocation = $shortcut.IconLocation;
            $results += [PSCustomObject]@{
                TargetPath = $targetPath
                IconLocation = $iconLocation
                LnkFile = $lnkFile
            };
        };
        $results | ConvertTo-Json
    `
  let outputtarget = await Command.create("powershell", [`${forstr}`], {
    encoding: "GBK",
  }).execute()
  let res: {
    IconLocation: string
    LnkFile: string
    TargetPath: string
  }[] = JSON.parse(outputtarget.stdout)
  for (let i = 0; i < res.length; i++) {
    lnks.push({
      type: "openPath",
      targetPath: res[i].TargetPath,
      lnkPath: lnkFiles[i],
      icoPath: "",
      name: "",
    })
  }
  info("完成快捷方式信息提取！")
  return lnks
}

// 获取lnk文件列表
export const getLnkFile = async function () {
  let lnkFiles = []
  let files = []
  // 桌面桌面文件夹不递归。
  let desktop = await readDir((await homeDir()) + "\\desktop")
  for (let i = 0; i < desktop.length; i++) {
    if (desktop[i].isFile) {
      files.push(await resolve((await homeDir()) + "\\desktop", desktop[i].name))
    }
  }

  // 递归默认程序快捷方式文件夹
  const lnkPath = [
    "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs",
    (await homeDir()) + "\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs",
  ]
  for (let i = 0; i < lnkPath.length; i++) {
    files.push(...(await scanFiles(lnkPath[i])))
  }
  // 查找lnk和url文件，去除多余文件
  for (let i = 0; i < files.length; i++) {
    try {
      let ext = await extname(files[i])
      if (
        (ext == "lnk" || ext == "url") &&
        (files[i].indexOf("卸载") == -1 || files[i].toLowerCase().indexOf("Uninstall") == -1)
      ) {
        lnkFiles.push(files[i])
      }
    } catch (e: any) {
      error("getLnkFile：" + e)
    }
  }
  // 去掉有关卸载的程序lnk
  lnkFiles = lnkFiles.filter((item) => {
    if (item.indexOf("卸载") < 0) {
      return true
    }
  })
  // 去掉有关uninstalll的lnk
  lnkFiles = lnkFiles.filter((item) => {
    if (item.toLowerCase().indexOf("uninstall") < 0) {
      return true
    }
  })
  info("完成快捷方式文件的扫描！")
  return lnkFiles
}

// 扫描文件夹内所有的文件
const scanFiles = async function (dir: string) {
  let rdirs: string[] = []
  let dirs = await readDir(dir)
  if (dirs) {
    for (let i = 0; i < dirs.length; i++) {
      let idir = await resolve(dir, dirs[i].name)
      if (dirs[i].isDirectory) {
        rdirs.push(...(await scanFiles(idir)))
      } else {
        rdirs.push(idir)
      }
    }
  }
  return rdirs
}
