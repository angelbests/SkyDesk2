import { Command } from '@tauri-apps/plugin-shell';
import { readDir } from "@tauri-apps/plugin-fs";
import { basename, extname, homeDir, resolve } from "@tauri-apps/api/path";

export const getlnks =async function(){
    let lnks = [];
    let lnkFiles = await getLnkFile();
    // 拼接shell脚本
    let lnkFilesstr = "$lnkFiles = @("
    for(let i=0;i<lnkFiles.length;i++){
        lnkFilesstr = lnkFilesstr + `\"${lnkFiles[i]}\"`
        if(i == lnkFiles.length-1){
            lnkFilesstr = lnkFilesstr+");"
        }else{
            lnkFilesstr = lnkFilesstr+","
        }
    }
    let forstr = lnkFilesstr +  `
        $shell = New-Object -ComObject WScript.Shell;
        $results = @();
        foreach ($lnkFile in $lnkFiles) {
            $shortcut = $shell.CreateShortcut($lnkFile);
            $targetPath = $shortcut.TargetPath;
            $iconLocation = $shortcut.IconLocation;
            $results += [PSCustomObject]@{
                TargetPath = $targetPath
                IconLocation = $iconLocation
                LnkFile = $lnkFile
            };
        };
        $results | ConvertTo-Json
    `;
    let outputtarget = await Command.create('powershell', [
        `${forstr}`,
    ],{
        "encoding":"GBK"
    }).execute();
    let res:{
        IconLocation:string
        LnkFile:string
        TargetPath:string
    }[] = JSON.parse(outputtarget.stdout);
    for(let i=0;i<res.length;i++){
        lnks.push({
            targetPath:res[i].TargetPath,
            iconLocationPeFile:res[i].IconLocation==null?"":res[i].IconLocation.split(',')[0],
            iconLocation:res[i].IconLocation==null?"":res[i].IconLocation.split(',')[1],
            lnkPath:lnkFiles[i],
            icoPath:"",
            name:(await basename(lnkFiles[i])).replace("."+await extname(lnkFiles[i]),"")
            
        })
    }
    console.log(`lnks：${lnks}`)
    return lnks;
}

// 获取lnk文件列表
const getLnkFile =async function(){
    let lnkFiles = [];
    let files = [];
    const lnkPath = [
        "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs",
        await homeDir() + "\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs"
    ]

    let desktop = await readDir(await homeDir() + "\\desktop");
    for(let i=0;i<desktop.length;i++){
        if(desktop[i].isFile){
            files.push(await resolve(await homeDir() + "\\desktop",desktop[i].name))
        }
    }

    for(let i=0;i<lnkPath.length;i++){
        files.push(...await scanFiles(lnkPath[i]))
    }
    for(let i=0;i<files.length;i++){
        try {
            let ext =await extname(files[i])
            if((ext == 'lnk' || ext == 'url')&&((files[i].indexOf("卸载")==-1)||(files[i].toLowerCase().indexOf("Uninstall")==-1))){
                lnkFiles.push(files[i])
            }
        } catch (error) {
           console.log(error)
        }
    }

    lnkFiles = lnkFiles.filter(item=>{
        if(item.indexOf("卸载")<0){
            return true
        }
    })

    lnkFiles = lnkFiles.filter(item=>{
        if((item.toLowerCase()).indexOf("uninstall")<0){
            return true
        }
    })
    console.log(lnkFiles)
    return lnkFiles;
}

// 扫描文件夹内所有的文件
const scanFiles = async function(dir:string){ 
    let rdirs:string[] = []
    let dirs = await readDir(dir)
    if(dirs){
        for(let i = 0;i<dirs.length;i++){
            let idir = await resolve(dir,dirs[i].name)
            if(dirs[i].isDirectory){
                rdirs.push(...await scanFiles(idir))
            }else{
                rdirs.push(idir)
            }
        }
    }
    return rdirs;
}

// 使用shman 获取lnk文件的信息
// export const getLnkInfo2 =async function(){
//     let lnks = [];
//     await mkdir("lnk",{"baseDir":BaseDirectory.AppData,"recursive":true})
//     let path = await resolve(await appDataDir(),"lnk","lnk.txt")
//     let res = await Command.sidecar(
//         "bin/shman",
//         [
//             "/stab",
//             path
//         ]
//     ).execute();
//     if(res.code == 0){
//         let unit8buffer= await readFile(path); 
//         let txt = new TextDecoder("GBK").decode(unit8buffer)
//         let arr = txt.split("\r\n");
//         for(let i=0;i<txt.length;i++){
//             if(arr[i]){
//                 let str = arr[i].split("\t")
//                 lnks.push(
//                     str
//                 )
//             }
//         }
//     }
// }