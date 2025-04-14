import { defineStore } from "pinia"
import { Note } from "../types/storeType"

export const noteStore = defineStore("note", {
  state: function () {
    return {
      note: [] as Note[],
    }
  },
  persist: true,
})
