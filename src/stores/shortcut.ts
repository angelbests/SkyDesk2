import { defineStore } from "pinia"
import { ShortCut } from "../types/storeType"

export const shortcutStore = defineStore("shortcut", {
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
