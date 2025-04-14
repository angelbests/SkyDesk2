import { defineStore } from "pinia"
import { WallpaperConfig, WallpaperList } from "../types/storeType"

export const wallpaperStore = defineStore("wallpaper", {
  state: function () {
    return {
      wallpaperConfig: [] as WallpaperConfig[],
      wallpaperList: [] as WallpaperList[],
    }
  },
  persist: true,
})
