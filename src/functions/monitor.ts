// import { Monitor } from "@tauri-apps/api/window";
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
// 设置tauri的window窗口置于 桌面图标层下
export const setWindowToMonitor = async function(label:string,x:number,y:number,width:number,height:number){
    setTimeout(() => {
        invoke("setwallpaper",{label:label,x:x,y:y,w:width,h:height})
    }, 1000);
}

// let a =await mouseMonitor((monitor,mouse)=>{
//     console.log(monitor,mouse)
// })
import { currentMonitor, Monitor } from '@tauri-apps/api/window';
type mousecallback = (monitor:Monitor|null,mouse:{x:number,f:number})=>void
export const mouseMonitor = async function(f:mousecallback){
    const monitor = await currentMonitor();
    const unlisten = listen("mouse-move",(e:{payload:{message:string}})=>{
        let mouse = JSON.parse(e.payload.message)
        f(monitor,mouse)
    })

    return unlisten
}