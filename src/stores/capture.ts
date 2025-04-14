import { defineStore } from "pinia"

export const captureStore = defineStore("capture", {
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
