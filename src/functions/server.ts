import { invoke } from '@tauri-apps/api/core';
import { appDataDir, resolve } from '@tauri-apps/api/path';
import { readDir } from '@tauri-apps/plugin-fs';
import { info,error } from '@tauri-apps/plugin-log';
export const openserver = async function(){
    let path = await resolve(await appDataDir(),"wallpapers\\html")
    try {
        await invoke("open_server",{str:path});
        info("服务启动成功：http://127.0.0.1:11434/")
    } catch (e) {
        error("服务器启动："+ e )
    }
}

export const serversacn = async function(){
    let path = await resolve(await appDataDir(),"wallpapers")
    const res = await readDir(path)
    return res;
}

