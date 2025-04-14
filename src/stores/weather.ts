import { defineStore } from "pinia"

export const weatherStore = defineStore("weather", {
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
