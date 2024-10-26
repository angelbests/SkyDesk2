import { Command } from '@tauri-apps/plugin-shell';
import { getlnks } from './lnks';
import { appDataDir, basename, extname, resolve } from '@tauri-apps/api/path';
import { copyFile, exists, mkdir, readDir, readFile, readTextFile, writeFile } from '@tauri-apps/plugin-fs';
import { uuid } from '.';
import { invoke } from '@tauri-apps/api/core';

export const setIcon =async function(){
    let path = await resolve(await appDataDir(),"ico")
    await mkdir(path,{"recursive":true})
    await mkdir(path+"\\other",{"recursive":true})
    let pattern = /%[^%]+%/;
    let lnks = await getlnks()
    let delarr = [];
    for(let i =0;i<lnks.length;i++){
        let ext = await extname(await basename(lnks[i].lnkPath))
        if(ext == 'lnk'){
            if(lnks[i].iconLocationPeFile == ""){
                // 无图标路径
                ext = await extname(lnks[i].targetPath);
                if(ext == "exe" || ext == "dll" || ext == "ocx" || ext == 'cpl' ){
                    let icodir = await getIcon(await resolve(lnks[i].targetPath),path)
                    if(icodir){
                        let res = await readDir(icodir+"\\");
                        res = res.filter(item=>{
                            return item.name.indexOf("ico")>0
                        })
                        res = icoSort(res)
                        lnks[i].icoPath = icodir + res[0].name
                    }else{
                        // 未找到图标文件 或 执行失败 返回false
                    }
                }else{
                    delarr.push(i)
                }

            }else{
                // 有图标路径
                if(pattern.test(lnks[i].iconLocationPeFile)){
                    // 有图标路径 环境变量路径
                    let pexec = pattern.exec(lnks[i].iconLocationPeFile)
                    if(pexec){
                        let systemPara = pexec[0];
                        let systemPath = await getSystemPath(systemPara)
                        if(systemPath){
                            let pePath = (lnks[i].iconLocationPeFile as string).replace(systemPara,systemPath)
                            let icodir = await getIcon(await resolve(pePath),path)
                            if(icodir){
                                if(Number(lnks[i].iconLocation)>=0){
                                    let res = await readDir(icodir);
                                    res = res.filter(item=>{
                                        return item.name.indexOf("ico")>0
                                    })
                                    res = icoSort(res)
                                    lnks[i].icoPath = icodir + res[Number(lnks[i].iconLocation)].name
                                }else if(Number(lnks[i].iconLocation)<0){
                                    let name = await basename(pePath)
                                    name = name.replace("."+await extname(name),"")
                                    lnks[i].icoPath = icodir + name + "_" + Math.abs(Number(lnks[i].iconLocation)) + '.ico'
                                }else{
                                    // 非数字结果
                                }
                            }else{
                                // 未找到图标文件 或 执行失败 返回false
                            }
                        }
                    }
                }else{
                    try {
                        // 有图标路径
                        ext = await extname(await basename(lnks[i].iconLocationPeFile))
                        if(ext =='ico' || ext == 'png'){
                            // 图标路径指向ico路径
                            if(await exists(lnks[i].iconLocationPeFile)){
                                let icoPath = await resolve(path,"other") +"\\"+ uuid() + '.ico'
                                let res = await readFile(lnks[i].iconLocationPeFile);
                                await writeFile(icoPath,res)
                                lnks[i].icoPath = icoPath
                            }else{
                                lnks[i].icoPath = ""
                            }

                        }else if(ext == "exe" || ext == "dll" || ext == "ocx" || ext == 'cpl'){
                            // 图标路径dll,exe 等pefile
                            let icodir = await getIcon(await resolve(lnks[i].iconLocationPeFile),path)
                            if(icodir){
                                if(Number(lnks[i].iconLocation)>=0){
                                    let res = await readDir(icodir);
                                    res = res.filter(item=>{
                                        return item.name.indexOf("ico")>0
                                    })
                                    res = icoSort(res)
                                    lnks[i].icoPath = icodir + res[Number(lnks[i].iconLocation)].name
                                }else if(Number(lnks[i].iconLocation)<0){
                                    let name = await basename(lnks[i].iconLocationPeFile)
                                    name = name.replace("."+await extname(name),"")
                                    lnks[i].icoPath = icodir + name + "_" + Math.abs(Number(lnks[i].iconLocation)) + '.ico'
                                }else{
                                    // 非数字结果
                                }
                            }else{
                                // 未找到图标文件 或 执行失败 返回false
                            }
                        }
                    } catch (e) {
                        let icoPath = await resolve(path,"other") + '\\'+uuid()+'.ico'
                        let res = await readFile(lnks[i].iconLocationPeFile);
                        await writeFile(icoPath,res)
                        lnks[i].icoPath = icoPath
                    }
                }
            }
        }else if(ext == 'url'){
            // url快捷方式
            let urlico = await getUrlInfo(lnks[i].lnkPath)
            if(urlico){
                let icoPath = await resolve(path,"other") + '\\'+uuid()+'.ico'
                let res = await readFile(urlico);
                await writeFile(icoPath,res)
                lnks[i].icoPath = icoPath
            }
        }
    }
    // 倒叙删除不用的lnk
    delarr.reverse()
    for(let i = 0 ;i<delarr.length;i++){
        lnks.splice(delarr[i],1)
    }

    // ico转PNG
    for(let i = 0;i<lnks.length; i++){
        if(await exists(lnks[i].icoPath)){
            if(containsChinese(lnks[i].icoPath)){
                let icoPath = await resolve(path,"other") + '\\'+uuid()+'.ico';
                await copyFile(lnks[i].icoPath,icoPath);
                let res = await invoke("ico_to_png",{
                    "from":icoPath,
                    "to":icoPath.replace(".ico",".png")
                })
                if(res == 1){
                    lnks[i].icoPath = (lnks[i].icoPath as string).replace(".ico",".png")
                }
            }else{
                let res = await invoke("ico_to_png",{
                    "from":lnks[i].icoPath,
                    "to":(lnks[i].icoPath as string).replace(".ico",".png")
                })
                if(res == 1){
                    lnks[i].icoPath = (lnks[i].icoPath as string).replace(".ico",".png")
                }
            }
        }
    }
    return lnks;
}

// 检查是否含中文
const containsChinese = function(str:string) {
    const chineseRegex = /[\u4e00-\u9fff]/;
    return chineseRegex.test(str);
}

// 解析url文件
const getUrlInfo = async function(path:string){
    let icoPath = "";
    let str = "IconFile=";
    let res = await readTextFile(path);
    let arr = res.split("\r\n");
    arr.filter(item=>{
        if(item.indexOf(str)>=0){
            icoPath = item.replace(str,"")
        }
    })
    return icoPath;
}

// 获取系统环境变量对应的路径
const getSystemPath =async function(systemPara:string){
    let res = await Command.create("systemPath",["/c","echo",`${systemPara}`]).execute()
    if(res.code == 0){
        console.log(res.stdout)
        return res.stdout.split("\r\n")[0]
    }else{
        return ""
    }
}

// 生成图标
const getIcon =async function(pePath:string,icoPath:string){
    let res = await Command.sidecar(
        "bin/ResourcesExtract",
        [
            "/Source",
            pePath,
            "/DestFolder",
            icoPath,
            "/ExtractIcons",
            "1",
            "/ExtractCursors",
            "0",
            "/OpenDestFolder",
            "0",
            "/MultiFilesMode",
            "2"
        ]
    )
    .execute()
    if(res.code == 0){
        let icodir = icoPath + "\\" + await basename(pePath) + "\\"
        if(await exists(icodir)){
            return icodir;
        }else{
            return false;
        }
        return 
    }else{
        return false
    }
}

// 图标排序
const icoSort = function(ico:any[]){
    for(let i = 0;i<ico.length;i++){
        for(let j = 0;j<ico.length;j++){
            if(Number.parseInt(ico[i].name.split("_")[1])<Number.parseInt(ico[j].name.split("_")[1])){
                let k = ico[i]
                ico[i] = ico[j]
                ico[j] = k
            }
        }
    }
    return ico   
}