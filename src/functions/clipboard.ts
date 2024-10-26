
import { writeText, writeHtml, writeImage } from "@tauri-apps/plugin-clipboard-manager"

export const writeClipboard = async function(writeContent:{
    type:"text",
    content:string
} | {
    type:"image",
    content: string | Uint8Array | ArrayBuffer | number[]
} | {
    type:"html",
    content: string, 
    altHtml?: string
}){
    if(writeContent.type == "text"){
        return writeText(writeContent.content)
    }else if(writeContent.type == "html"){
        if(writeContent.altHtml == undefined){
            return await writeHtml(writeContent.content)
        }else{
            return await writeHtml(writeContent.content,writeContent.altHtml)
        }
    }else if(writeContent.type == "image"){
        return await writeImage(writeContent.content)
    }
}

// 有问题
// export const readClipboard = async function() {
//     readImage().then(e=>{
//         console.log(e)
//     }).catch(e=>{
//         console.log(e)
//     })
// }