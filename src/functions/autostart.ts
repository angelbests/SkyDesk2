import { isEnabled,enable,disable } from '@tauri-apps/plugin-autostart';
import { systemStore } from '../stores/window';

export const setautostart = async function(){
    let issupport = await isEnabled();
    const system = systemStore();
    if(issupport){
        if(system.autostart){
            enable()
        }else{
            disable()
        }
    }
}
