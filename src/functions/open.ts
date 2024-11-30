import { Command } from '@tauri-apps/plugin-shell';
export const exec = async function (item: any) {
    if (item.lnkPath) {
        console.log(item)
        let res = await Command.create("powershell", `& "${item.lnkPath}"`, { "encoding": 'GBK' }).execute()
        console.log(res)
    } else {
        await Command.create("powershell", item.targetPath).execute()
    }
}