import { defineStore } from "pinia";
import { WindowOptions } from "@tauri-apps/api/window";
import { WebviewOptions } from "@tauri-apps/api/webview";
import { availableMonitors,Monitor} from '@tauri-apps/api/window';
const monitors =await availableMonitors();
const windowStore = defineStore("window",{
    "state":function(){
        return {
            windows:[] as {
                label:string,
                monitor?:boolean,
                option:Omit<WebviewOptions, "x" | "y" | "width" | "height"> & WindowOptions
            }[],
            monitors:monitors as Monitor[]
        }
    },
    "persist":{
        paths:["windows"]
    }
})

const systemStore = defineStore("system",{
    "state":function(){
        return {
            autostart:false as boolean,
            traystart:false as boolean,
            netspeed:{
                show:false as boolean,
                x:0,
                y:0
            },
        }
    },
    "persist":{
        paths:["autostart","traystart","netspeed"]
    }
})

const noteStore = defineStore("note",{
    "state":function(){
        return {
            note:[] as {
                value:string,
                date:string,
                color:string,
                opacity:number,
                label:string,
                option?:Omit<WebviewOptions, "x" | "y" | "width" | "height"> & WindowOptions
            }[]
        }
    },
    persist:true
})

const wallpaperStore = defineStore("wallpaper",{
    "state":function(){
        return {
            config:[] as {
                label:string,
                monitor:Monitor,
                type:"image"|"html"|"video",
                url:string
            }[],
            status:false,
            wallpaperList :[] as {
                "type": "image" | "video" | "html"
                "title": string
                "preview": string // 预览图
                "filename": string // 文件名称
                "path": string
              }[] 
        }
    },
    persist:true
})

const captureStore = defineStore("capture",{
    "state":function(){
        return {
            captureList: [] as {
                name:string,
                time:string,
                path:string
            }[]
        }
    }
})

const shortcutStore = defineStore("shortcut",{
    "state":function(){
        return {
            shortcutsTemp:[] as {
                targetPath:string
                iconLocationPeFile:string
                iconLocation:string
                lnkPath:string
                icoPath:string
                name:string
            }[],
            shortcuts:[] as {
                targetPath:string
                iconLocationPeFile:string
                iconLocation:string
                lnkPath:string
                icoPath:string
                name:string
            }[],
            wheels:[] as {
                targetPath:string
                iconLocationPeFile:string
                iconLocation:string
                lnkPath:string
                icoPath:string
                name:string
            }[]
        }
    },
    "persist":true,
})

export {windowStore,noteStore,wallpaperStore,captureStore,shortcutStore,systemStore}