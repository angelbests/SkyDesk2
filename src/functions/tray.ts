import { resourceDir } from '@tauri-apps/api/path';
import { TrayIcon, TrayIconEvent } from '@tauri-apps/api/tray';
import { getAllWebviewWindows, getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
export const createtray = async function(){
    let icodir =await resourceDir();
    console.log(icodir)
    await TrayIcon.new({ 
        tooltip: 'SkyDesk2',
        icon:icodir + "\\icons\\icon.ico",
        action:async (e:TrayIconEvent)=>{
            if(e.type == "Click"){
                if(e.button == "Left"){
                    getCurrentWebviewWindow().show()
                    getCurrentWebviewWindow().setFocus()
                }else if(e.button == "Right"){
                    let all =await getAllWebviewWindows();
                    all.filter(async item=>{
                        if(item.label == "tray"){
                            let factor = await item.scaleFactor()
                            e.position.y = Math.trunc(e.position.y - 30*factor);
                            item.setPosition(e.position)
                            item.show()
                            item.setFocus()
                            
                        }
                    })
                }
            }
        }
    });
}