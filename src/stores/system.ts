import { convertFileSrc } from "@tauri-apps/api/core"
import { resourceDir } from "@tauri-apps/api/path"
import { defineStore } from "pinia"
let path = ((await resourceDir()) + "\\resources\\bg.png") as string
export const systemStore = defineStore("system", {
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
      programbcakground: `url("${convertFileSrc(path)}")` as string,
      leftbackground: "" as string,
      topbackground: "" as string,
      btnbackground: "" as string,
      shortcutbackground: "rgba(123,123,123,0.2)" as string,
      btnbarbackground: "" as string,
    }
  },
  persist: true,
})
