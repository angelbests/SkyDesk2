<script setup lang="ts" >
import {  getAllWebviewWindows, getCurrentWebviewWindow, WebviewWindow} from '@tauri-apps/api/webviewWindow';
import { onMounted, ref } from 'vue';
import { emit } from '@tauri-apps/api/event';
const captureWindow = ref<WebviewWindow>();
onMounted(async ()=>{
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
})

const stop = async function () {
    let str = "STOP"
    await emit("capture", `${str}`);
    getCurrentWebviewWindow().hide()
    captureWindow.value?.hide()
}
</script>

<template>
    <div style="width: 100vw;height: 100vh;">
        <v-btn style="font-size: 12px;" size="small" @click="stop">停止录制</v-btn>
    </div>
</template>

<style>

</style>