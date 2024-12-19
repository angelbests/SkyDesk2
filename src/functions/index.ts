import { convertFileSrc } from "@tauri-apps/api/core";
import { appDataDir, resolve } from "@tauri-apps/api/path";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import {
  BaseDirectory,
  writeFile,
  mkdir,
  readDir,
} from "@tauri-apps/plugin-fs";
import html2canvas from "html2canvas";

// UUID
export const uuid = function (): string {
  return (
    "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
      let r = (Math.random() * 16) | 0,
        v = c == "x" ? r : (r & 0x3) | 0x8;
      return v.toString(16);
    }) +
    "-" +
    new Date().getTime()
  );
};

// DOM截屏
export const screenshot = async function () {
  await mkdir("window", { baseDir: BaseDirectory.AppData, recursive: true });
  let canvas = await html2canvas(document.body);
  canvas.toBlob(async (e) => {
    console.log(e);
    if (e) {
      writeFile(
        "window/screenshot-" + getCurrentWebviewWindow().label + ".png",
        new Uint8Array(await e.arrayBuffer()),
        { baseDir: BaseDirectory.AppData, create: true }
      );
    }
  });
  return convertFileSrc(
    await resolve(
      await appDataDir(),
      "window",
      "screenshot-" + getCurrentWebviewWindow().label + ".png"
    )
  );
};

// 扫描文件夹内所有的文件
export const scanFiles = async function (dir: string) {
  let rdirs: string[] = [];
  let dirs = await readDir(dir);
  if (dirs) {
    for (let i = 0; i < dirs.length; i++) {
      let idir = await resolve(dir, dirs[i].name);
      if (dirs[i].isDirectory) {
        rdirs.push(...(await scanFiles(idir)));
      } else {
        rdirs.push(idir);
      }
    }
  }
  return rdirs;
};

export const gettime = function () {
  let date = new Date();
  let y = date.getFullYear();
  let m = date.getMonth() + 1;
  let d = date.getDate();
  let h = date.getHours();
  let min = date.getMinutes();
  let s = date.getSeconds();
  return y + "-" + m + "-" + d + " " + h + ":" + min + ":" + s;
};
