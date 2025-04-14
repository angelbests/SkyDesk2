import { availableMonitors, Monitor } from "@tauri-apps/api/window"
import { defineStore } from "pinia"
import { Windows } from "../types/storeType"
const monitors = await availableMonitors()
export const windowStore = defineStore("window", {
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
