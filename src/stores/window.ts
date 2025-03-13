import { defineStore } from "pinia"
import { Note, ShortCut, WallpaperConfig, WallpaperList, Windows, Monitor } from "../types/storeType"
import { availableMonitors } from "@tauri-apps/api/window"
const monitors = await availableMonitors()

const windowStore = defineStore("window", {
  state: function () {
    return {
      windows: [] as Windows[],
      monitors: monitors as Monitor[],
    }
  },
  persist: {
    paths: ["windows"],
  },
})

const systemStore = defineStore("system", {
  state: function () {
    return {
      autostart: false as boolean,
      traystart: false as boolean,
      wheel: true as boolean,
      netspeed: {
        show: false as boolean,
        x: 0,
        y: 0,
      },
      szie: {
        width: 0,
        height: 0,
      },
      taskbar: true as boolean,
      fontcolor: "black" as string,
      programbcakground: `white` as string,
      leftbackground: "white" as string,
      topbackground: "wheat" as string,
      btnbackground: "white" as string,
      shortcutbackground: "rgba(123,123,123,0.2)" as string,
      btnbarbackground: "white" as string,
    }
  },
  persist: true,
})

const noteStore = defineStore("note", {
  state: function () {
    return {
      note: [] as Note[],
    }
  },
  persist: true,
})

const wallpaperStore = defineStore("wallpaper", {
  state: function () {
    return {
      wallpaperConfig: [] as WallpaperConfig[],
      wallpaperList: [] as WallpaperList[],
    }
  },
  persist: true,
})

const shortcutStore = defineStore("shortcut", {
  state: function () {
    return {
      shortcutsTemp: [] as ShortCut[],
      shortcuts: [
        {
          title: "开始",
          index: 0,
          shortcut: [],
        },
      ] as {
        title: string
        index: number
        shortcut: ShortCut[]
      }[],
      wheels: [] as ShortCut[],
    }
  },
  persist: true,
})

const weatherStore = defineStore("weather", {
  state() {
    return {
      apikey: "9cda7ed49a914d5eb6987706d642da65" as string,
      city: "" as string,
      citycode: "" as string,
      query: "" as string,
      pois: [] as any[],
    }
  },
  persist: true,
})

const captureStore = defineStore("capture", {
  state() {
    return {
      video: [] as {
        name: string
        path: string
      }[],
    }
  },
  persist: true,
})

export { windowStore, noteStore, wallpaperStore, shortcutStore, systemStore, weatherStore, captureStore }
