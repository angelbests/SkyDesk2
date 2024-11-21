<script setup lang="ts" >
import {  getAllWebviewWindows, getCurrentWebviewWindow, WebviewWindow} from '@tauri-apps/api/webviewWindow';
import { onMounted, ref } from 'vue';
import { emit, listen } from '@tauri-apps/api/event';
import { captureStore } from '../../stores/window';
const capturestore = captureStore()
const captureWindow = ref<WebviewWindow>();
const message = ref<{
    name:string,
    path:string
}>({
    name:"",
    path:""
})
onMounted(async ()=>{
    window.addEventListener("storage",(e)=>{
        if(e.key == "capture"){
            capturestore.$hydrate()
        }
    })
    let all =await getAllWebviewWindows();
    all.filter(item=>{
        if(item.label == "captureWindow"){
            captureWindow.value = item
        }
    })
    document.addEventListener("contextmenu",(e)=>{
        e.preventDefault()
    })
    document.addEventListener("selectstart",(e)=>{
        e.preventDefault()
    })

    listen("capturemessage",(e:any)=>{
        message.value.name = e.payload.name
        message.value.path = e.payload.path
    })
})

const stop = async function () {
    let str = "STOP"
    await emit("capture", `${str}`);
    await getCurrentWebviewWindow().hide()
    await captureWindow.value?.hide()
    setTimeout(() => {
        capturestore.video.push({
            name:message.value.name,
            path:message.value.path
        })
    }, 5000);
}
</script>

<template>
    <div style="width: 100vw;height: 100vh;">
        <v-btn style="font-size: 12px;" size="small" @click="stop">停止录制</v-btn>
    </div>
</template>

<style>

</style>