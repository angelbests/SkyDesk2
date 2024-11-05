import { WebviewWindow, getCurrentWebviewWindow} from "@tauri-apps/api/webviewWindow";
import { WindowOptions } from "@tauri-apps/api/window";
import { WebviewOptions } from "@tauri-apps/api/webview";
import { windowStore } from "../stores/window";
export const createWindow = async function(label:string,option:Omit<WebviewOptions, "x" | "y" | "width" | "height"> & WindowOptions){
    const windowstore = windowStore();
    let indexbool = windowstore.windows.findIndex((item)=>{
        return item.label === label
    })
    if(indexbool<0){
        windowstore.windows.push({
            label,
            option
        })
        const w = new WebviewWindow(label,option)
        if(w.label.indexOf("wallpaper-")<0){
            listenMove(w)
            listenSize(w)
        }
        listenClose(w)
        return w;
    }else{
        return null;
    } 
}

export const initWindow = async function(){
    const windowstore = windowStore();
    windowstore.windows.forEach(e => {
        let w = new WebviewWindow(e.label,e.option);
        if(w.label.indexOf("wallpaper-")<0){
            listenMove(w)
            listenSize(w)
        }
        listenClose(w)
    });
}

const listenMove = async function(w:WebviewWindow){
    let label = w.label
    const factor = await getCurrentWebviewWindow().scaleFactor()
    await w.listen("tauri://move",function(event:any){
        const windowstore = windowStore()
        let index = windowstore.windows.findIndex((item)=>{
            return item.label === label
        })
        windowstore.windows[index].option.x = event.payload.x/factor
        windowstore.windows[index].option.y = event.payload.y/factor
        console.log(event.payload)
    })
}

const listenClose =async function(w:WebviewWindow){
    let label = w.label
    await w.listen("tauri://close-requested",(event)=>{
        const windowstore = windowStore()
        let index = windowstore.windows.findIndex((item)=>{
            return item.label === label
        })
        windowstore.windows.splice(index,1)
        console.log(event)
        w.destroy()
    })
}

const listenSize = async function(w:WebviewWindow) {
    let label = w.label
    const factor = await getCurrentWebviewWindow().scaleFactor()
    await w.listen("tauri://resize",(event:any)=>{
        const windowstore = windowStore()
        let index = windowstore.windows.findIndex((item)=>{
            return item.label === label
        })
        windowstore.windows[index].option.width = event.payload.width/factor
        windowstore.windows[index].option.height = event.payload.height/factor
        console.log(event.payload)
    })
}