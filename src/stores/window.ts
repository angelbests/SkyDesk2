import { defineStore } from "pinia";
import { WindowOptions } from "@tauri-apps/api/window";
import { WebviewOptions } from "@tauri-apps/api/webview";
import { availableMonitors, Monitor } from "@tauri-apps/api/window";
const monitors = await availableMonitors();
const windowStore = defineStore("window", {
  state: function () {
    return {
      windows: [] as {
        label: string;
        monitor?: boolean;
        option: Omit<WebviewOptions, "x" | "y" | "width" | "height"> &
          WindowOptions;
        wallpaper:{
          x:number,
          y:number,
          w:number,
          h:number,
          z:number,
          status:boolean
        }
      }[],
      monitors: monitors as Monitor[],
    };
  },
  persist: {
    paths: ["windows"],
  },
});

const systemStore = defineStore("system", {
  state: function () {
    return {
      autostart: false as boolean,
      traystart: false as boolean,
      netspeed: {
        show: false as boolean,
        x: 0,
        y: 0,
      },
    };
  },
  persist: {
    paths: ["autostart", "traystart", "netspeed"],
  },
});

const noteStore = defineStore("note", {
  state: function () {
    return {
      note: [] as {
        value: string;
        date: string;
        color: string;
        opacity: number;
        label: string;
        wallpaper:boolean;
        option?: Omit<WebviewOptions, "x" | "y" | "width" | "height"> &
          WindowOptions;
      }[],
    };
  },
  persist: true,
});

const wallpaperStore = defineStore("wallpaper", {
  state: function () {
    return {
      config: [] as {
        label: string;
        monitor: Monitor;
        type: "image" | "html" | "video";
        url: string;
      }[],
      status: false,
      wallpaperList: [] as {
        type: "image" | "video" | "html";
        title: string;
        preview: string; // 预览图
        filename: string; // 文件名称
        path: string;
      }[],
    };
  },
  persist: true,
});

const shortcutStore = defineStore("shortcut", {
  state: function () {
    return {
      shortcutsTemp: [] as {
        targetPath: string;
        iconLocationPeFile: string;
        iconLocation: string;
        lnkPath: string;
        icoPath: string;
        name: string;
      }[],
      shortcuts: [{
        title:"开始",
        index:0,
        shortcut:[]
      }] as {
        title:string,
        index:number,
        shortcut:{
          targetPath: string;
          iconLocationPeFile: string;
          iconLocation: string;
          lnkPath: string;
          icoPath: string;
          name: string;
        }[]
      }[],
      wheels: [] as {
        targetPath: string;
        iconLocationPeFile: string;
        iconLocation: string;
        lnkPath: string;
        icoPath: string;
        name: string;
      }[],
    };
  },
  persist: true,
});

const weatherStore = defineStore("weather", {
  state() {
    return {
      apikey: "9cda7ed49a914d5eb6987706d642da65" as string,
      city: "" as string,
      citycode: "" as string,
      query: "" as string,
      pois: [] as any[],
    };
  },
  persist: true,
});

const captureStore = defineStore("capture", {
  state() {
    return {
      video: [] as {
        name: string;
        path: string;
      }[],
    };
  },
  persist: true,
});

export {
  windowStore,
  noteStore,
  wallpaperStore,
  shortcutStore,
  systemStore,
  weatherStore,
  captureStore,
};
