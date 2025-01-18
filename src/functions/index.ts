import { resolve } from "@tauri-apps/api/path";
import { readDir } from "@tauri-apps/plugin-fs";

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

export const fileToBase64 = function (file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.readAsDataURL(file);
    reader.onload = function () {
      if (reader.result) {
        resolve(reader.result as string);
      } else {
        reject("Failed to load file");
      }
    };
    reader.onerror = function () {
      reject("Failed to load file");
    };
  });
};
