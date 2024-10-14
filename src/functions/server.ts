import { invoke } from '@tauri-apps/api/core';
import { appDataDir, resolve } from '@tauri-apps/api/path';
import { readDir } from '@tauri-apps/plugin-fs';
export const openserver = async function(){
    let path = await resolve(await appDataDir(),"wallpapers\\html")
    invoke("open_server",{str:path});
}

export const serversacn = async function(){
    let path = await resolve(await appDataDir(),"wallpapers")
    const res = await readDir(path)
    return res;
}

