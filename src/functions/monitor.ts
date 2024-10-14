// import { Monitor } from "@tauri-apps/api/window";
import { invoke } from '@tauri-apps/api/core';
// 设置tauri的window窗口置于 桌面图标层下
export const setWindowToMonitor = async function(label:string,x:number,y:number,width:number,height:number){
    setTimeout(() => {
        invoke("a",{label:label,x:x,y:y,w:width,h:height})
    }, 1000);
}

