import { Command } from "@tauri-apps/plugin-shell";
import { getlnks } from "./lnks";
import {
  appDataDir,
  basename,
  extname,
  resolve,
  resourceDir,
} from "@tauri-apps/api/path";
import {
  copyFile,
  exists,
  readDir,
  readFile,
  readTextFile,
  writeFile,
} from "@tauri-apps/plugin-fs";
import { uuid } from ".";
import { invoke } from "@tauri-apps/api/core";
import { info,error } from "@tauri-apps/plugin-log";

export const setIcon = async function () {
  let defaultico = (await resourceDir()) + "/resources/program.png";
  let path = await resolve(await appDataDir(), "ico");
  // 检查环境变量 正则表达式
  let pattern = /%[^%]+%/;
  let lnks = await getlnks();
  // 替换环境变量
  info("替换环境变量");
  for (let i = 0; i < lnks.length; i++) {
    if (pattern.test(lnks[i].iconLocationPeFile)) {
      info("iconLocationPeFile有环境变量");
      let pexec = pattern.exec(lnks[i].iconLocationPeFile);
      if (pexec) {
        let systemPara = pexec[0];
        let systemPath = await getSystemPath(systemPara);
        if (systemPath) {
          lnks[i].iconLocationPeFile = (
            lnks[i].iconLocationPeFile as string
          ).replace(systemPara, systemPath);
        }
      }
    }
    if (pattern.test(lnks[i].targetPath)) {
      info("targetPath有环境变量");
      let pexec = pattern.exec(lnks[i].targetPath);
      if (pexec) {
        let systemPara = pexec[0];
        let systemPath = await getSystemPath(systemPara);
        if (systemPath) {
          lnks[i].targetPath = (lnks[i].targetPath as string).replace(
            systemPara,
            systemPath
          );
        }
      }
    }
  }

  for (let i = 0; i < lnks.length; i++) {
    // 检查路径文件是否存在
    if (!(await exists(lnks[i].lnkPath))) continue;
    // 获取文件名称
    let name = await basename(lnks[i].lnkPath);
    // 获取文件格式
    let ext = await extname(name);
    ext = ext.toLocaleLowerCase();
    // 分别处理 lnk 和 url文件
    info(lnks[i].name)
    if (ext == "lnk") {
      info("无图标文件");
      if (lnks[i].iconLocationPeFile == "") {
        if (lnks[i].targetPath == "") {
          lnks[i].icoPath = defaultico;
        } else {
          lnks[i].icoPath = await targetPathgetico(lnks[i].targetPath);
        }
      } else {
        info("有图标文件");
        lnks[i].icoPath = await iconLocationPeFilegetico(
          lnks[i].iconLocationPeFile,
          lnks[i].iconLocation
        );
      }
    } else if (ext == "url") {
      // url快捷方式
      let urlico = await getUrlInfo(lnks[i].lnkPath);
      if (await exists(urlico)) {
        let icoPath = (await resolve(path, "other")) + "\\" + uuid() + ".ico";
        let res = await readFile(urlico);
        await writeFile(icoPath, res);
        lnks[i].icoPath = icoPath;
      } else {
        info(`url格式`);
        let path = (await resourceDir()) + "/resources/url.png";
        lnks[i].icoPath = path;
      }
    }
  }

  // ico转PNG
  for (let i = 0; i < lnks.length; i++) {
    let ext = await extname(lnks[i].icoPath)
    if(ext != "ico") continue;
    if (await exists(lnks[i].icoPath)) {
      if (containsChinese(lnks[i].icoPath)) {
        let icoPath = await resolve(path, "other",uuid()+'.ico',);
        await copyFile(lnks[i].icoPath, icoPath);
        let res = await invoke("ico_to_png", {
          from: icoPath,
          to: await resolve(icoPath.replace(".ico", ".png")),
        });
        if (res == 1) {
          lnks[i].icoPath = await resolve(icoPath.replace(".ico", ".png"));
        } else {
          error(`${lnks[i].name} ：ico转png失败`);
        }
      } else {
        let res = await invoke("ico_to_png", {
          from: await resolve(lnks[i].icoPath),
          to: await resolve((lnks[i].icoPath as string).replace(".ico", ".png")),
        });
        if (res == 1) {
          lnks[i].icoPath = await resolve((lnks[i].icoPath as string).replace(".ico", ".png"))
        } else {
          error(`${lnks[i].name} ：ico转png失败`);
        }
      }
    }
  }
  return lnks;
};

const iconLocationPeFilegetico = async function (
  pe: string,
  iconLocation: string
) {
  let path = await resolve(await appDataDir(), "ico");
  let defaultico = (await resourceDir()) + "/resources/program.png";
  let ext = "";
  try {
    ext = await extname(await basename(pe));
    ext = ext.toLocaleLowerCase();
  } catch (e) {
    error("格式解析报错："+e);
    if (pe.indexOf("C:\\Windows\\Installer\\") >= 0) {
      if (await exists(pe)) {
        let icoPath = (await resolve(path, "other")) + "\\" + uuid() + ".ico";
        let res = await readFile(pe);
        await writeFile(icoPath, res);
        info("安装路径图标文件夹：C:\\Windows\\Installer\\");
        return icoPath;
      } else {
        info("图标指向的路径不存在！");
        return defaultico;
      }
    } else {
      info("有图标路径，无格式文件");
      return defaultico;
    }
  }
  if (ext == "ico" || ext == "png") {
    if (await exists(pe)) {
      let icoPath = (await resolve(path, "other")) + "\\" + uuid() + ".ico";
      let res = await readFile(pe);
      await writeFile(icoPath, res);
      info("ico/png图标复制成功！");
      return icoPath;
    } else {
      info("ico/png图标指向的路径不存在！");
      return defaultico;
    }
  } else if (ext == "exe" || ext == "dll" || ext == "ocx" || ext == "cpl") {
    let icodir = await getIcon(await resolve(pe), path);
    if (icodir) {
      if (Number(iconLocation) >= 0) {
        let res = await readDir(icodir);
        res = res.filter((item) => {
          return item.name.indexOf("ico") > 0;
        });
        res = icoSort(res);
        if (Number(iconLocation) < res.length) {
          info("解析图标成功");
          return icodir + res[Number(iconLocation)].name;
        } else {
          info("解析的图标不存在");
          return defaultico;
        }
      } else if (Number(iconLocation) < 0) {
        let name = await basename(pe);
        try {
          name = name.replace("." + (await extname(name)), "");
        } catch (e) {
          error("格式解析报错："+e);
          return defaultico;
        }
        let path =
          icodir + name + "_" + Math.abs(Number(iconLocation)) + ".ico";
        if (await exists(path)) {
          info("解析图标成功");
          return path;
        } else {
          info("解析的图标不存在");
          return defaultico;
        }
      } else {
        return defaultico;
      }
    } else {
      try {
        return await targetPathgetico(pe);
      } catch (e) {
        error("PE文件解析失败："+e);
        return defaultico;
      }
    }
  } else {
    info("其它格式");
    return defaultico;
  }
};

// 解析PEFILE
const targetPathgetico = async function (pe: string) {
  let path = await resolve(await appDataDir(), "ico");
  let defaultico = (await resourceDir()) + "/resources/program.png";
  let ext = "";
  // 检查文件是否存在
  if (await exists(pe)) {
    try {
      ext = await extname(pe);
      ext = ext.toLocaleLowerCase();
    } catch (e:any) {
      error("格式解析失败，为文件夹：" + e);
      return (await resourceDir()) + "/resources/dir.png";
    }
  } else {
    return defaultico;
  }
  if (ext == "exe" || ext == "dll" || ext == "ocx" || ext == "cpl") {
    let icodir = await getIcon(await resolve(pe), path);
    if (icodir) {
      let res = await readDir(icodir + "\\");
      res = res.filter((item) => {
        return item.name.indexOf("ico") > 0;
      });
      res = icoSort(res);
      info("PE图标提取成功");
      return icodir + res[0].name;
    } else {
      info("PE提取失败");
      return defaultico;
      // 未找到图标文件 或 执行失败 返回false
    }
  } else if (ext == "chm") {
    info(`chm格式`);
    return (await resourceDir()) + "/resources/chm.png";
  } else if (ext == "url") {
    info(`url格式`);
    return (await resourceDir()) + "/resources/url.png";
  } else if (ext == "html" || ext == "htm") {
    info(`html格式`);
    return (await resourceDir()) + "/resources/html.png";
  } else if (
    ext == "mp4" ||
    ext == "flv" ||
    ext == "mkv" ||
    ext == "mov" ||
    ext == "wmv" ||
    ext == "webm"
  ) {
    info(`视频格式`);
    return (await resourceDir()) + "/resources/video.png";
  } else if (
    ext == "png" ||
    ext == "jpg" ||
    ext == "jpeg" ||
    ext == "svg" ||
    ext == "git" ||
    ext == "webp" ||
    ext == "psd" ||
    ext == "bmp"
  ) {
    info(`图片格式`);
    return (await resourceDir()) + "/resources/image.png";
  } else if (ext == "txt") {
    info(`文本格式`);
    return (await resourceDir()) + "/resources/txt.png";
  } else {
    info(`其它文件格式`);
    return (await resourceDir()) + "/resources/file.png";
  }
};

// 检查是否含中文
const containsChinese = function (str: string) {
  const chineseRegex = /[\u4e00-\u9fff]/;
  return chineseRegex.test(str);
};

// 解析url文件
const getUrlInfo = async function (path: string) {
  let icoPath = "";
  let str = "IconFile=";
  let res = await readTextFile(path);
  let arr = res.split("\r\n");
  arr.filter((item) => {
    if (item.indexOf(str) >= 0) {
      icoPath = item.replace(str, "");
    }
  });
  return icoPath;
};

// 获取系统环境变量对应的路径
const getSystemPath = async function (systemPara: string) {
  let res = await Command.create("systemPath", [
    "/c",
    "echo",
    `${systemPara}`,
  ]).execute();
  if (res.code == 0) {
    info(res.stdout);
    return res.stdout.split("\r\n")[0];
  } else {
    return "";
  }
};

// 使用ResourcesExtract.exe 提取程序图标
const getIcon = async function (pePath: string, icoPath: string) {
  let res = await Command.sidecar("bin/ResourcesExtract", [
    "/Source",
    pePath,
    "/DestFolder",
    icoPath,
    "/ExtractIcons",
    "1",
    "/ExtractCursors",
    "0",
    "/OpenDestFolder",
    "0",
    "/MultiFilesMode",
    "2",
  ]).execute();
  if (res.code == 0) {
    let icodir = icoPath + "\\" + (await basename(pePath)) + "\\";
    if (await exists(icodir)) {
      return icodir;
    } else {
      return false;
    }
    return;
  } else {
    return false;
  }
};

// 图标排序
const icoSort = function (ico: any[]) {
  for (let i = 0; i < ico.length; i++) {
    for (let j = 0; j < ico.length; j++) {
      if (
        Number.parseInt(ico[i].name.split("_")[1]) <
        Number.parseInt(ico[j].name.split("_")[1])
      ) {
        let k = ico[i];
        ico[i] = ico[j];
        ico[j] = k;
      }
    }
  }
  return ico;
};
