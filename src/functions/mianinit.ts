import { BaseDirectory, mkdir } from '@tauri-apps/plugin-fs';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { register,isRegistered } from "@tauri-apps/plugin-global-shortcut"
import { invoke } from '@tauri-apps/api/core';
import { createtray,traystart } from '../functions/tray';
import { initWindow } from '../functions/window';
import { exit } from '@tauri-apps/plugin-process';
export const maininit =async function(){
    // 检查并创建文件夹
    await mkdir("lnk",{"baseDir":BaseDirectory.AppData,"recursive":true})
    await mkdir("window",{baseDir:BaseDirectory.AppData,"recursive":true})
    await mkdir("wallpapers",{baseDir:BaseDirectory.AppData,"recursive":true})
    await mkdir("wallpapers\\image",{baseDir:BaseDirectory.AppData,"recursive":true})
    await mkdir("wallpapers\\html",{baseDir:BaseDirectory.AppData,"recursive":true})
    await mkdir("wallpapers\\video",{baseDir:BaseDirectory.AppData,"recursive":true})
    await mkdir("note",{baseDir:BaseDirectory.AppData,"recursive":true})
    await mkdir("ollama",{baseDir:BaseDirectory.AppData,"recursive":true})
    await mkdir("skydesk2",{baseDir:BaseDirectory.Picture,"recursive":true})
    await mkdir("skydesk2",{baseDir:BaseDirectory.Video,"recursive":true})
    // 注册快捷按键
    let res =await isRegistered('Control+1')   
    if(!res){
        register('Control+1',async ()=>{
            await getCurrentWebviewWindow().show();
            await getCurrentWebviewWindow().setFocus()
        })
    }
    res =await isRegistered('Control+2')
    if(!res){
        register('Control+2',async ()=>{
            await getCurrentWebviewWindow().hide();
        })
    }
    // 开启网速事件
    await invoke("netspeed")
    // 创建托盘
    await createtray()
    // 检查托盘启动
    await traystart()
    // 初始化窗口
    await initWindow()
    // 任务栏关闭窗口
    getCurrentWebviewWindow().onCloseRequested(()=>{
        exit()
    })
    // 恢复阴影
    getCurrentWebviewWindow().setShadow(true)
}

export const allinit =async function(){
    // 禁止右键
    document.addEventListener("contextmenu",(e)=>{
        e.preventDefault()
    })
    // 禁止快捷按键
    document.onkeydown = function(e){
        console.log(e)
        if(e.key == "F7"){
            return false
        }
        // 屏蔽刷新
        if(e.key == "F5"){
            return false
        }
        // 屏蔽刷新
        if(e.ctrlKey && e.key == "r"){
            return false
        }
        // 禁止打印
        if(e.ctrlKey && e.key == "p"){
            return false
        }
        // 禁止查找
        if(e.key == "F3"){
            return false
        }
        // 禁止强制刷新
        if(e.ctrlKey && e.shiftKey && e.key == "R"){
            
            return false
        }
        // 查看源代码
        if(e.ctrlKey && e.key == "u"){
            return false
        }
        // 查找网页
        if(e.ctrlKey && e.key == "f"){
            return false
        }
        // 查找下一个
        if(e.ctrlKey && e.key == "g"){
            return false
        }
        // 查找上一个
        if(e.ctrlKey && e.shiftKey && e.key == "G"){
            return false
        }
        // 下载
        if(e.ctrlKey && e.key == "j"){
            return false
        }
    }
    // 禁止文本选择
    if(getCurrentWebviewWindow().label.indexOf('note-')>=0) return
    document.addEventListener("selectstart",(e)=>{
        e.preventDefault();
    })
}