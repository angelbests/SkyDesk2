<script setup lang="ts" >
import { LogicalPosition, LogicalSize } from '@tauri-apps/api/dpi';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { onMounted, ref, watch } from 'vue';
import { systemStore } from '../../stores/window';
getCurrentWebviewWindow().setSize(new LogicalSize(80,45))
const net = ref({
    speed_r:0,
    speed_s:0
})
const  systemstore = systemStore()
onMounted(()=>{
    getCurrentWebviewWindow().setPosition(new LogicalPosition(systemstore.netspeed.x,systemstore.netspeed.y))
    getCurrentWebviewWindow().listen("tauri://move",async (event:any)=>{
        const factor = await getCurrentWebviewWindow().scaleFactor()
        systemstore.netspeed.x = event.payload.x/factor
        systemstore.netspeed.y = event.payload.y/factor
    })
    window.addEventListener("storage",(e)=>{
        if(e.key == "system"){
            systemstore.$hydrate();
        }
    })
    listen("netspeed",(e)=>{
        let res = JSON.parse(e.payload as string)
        net.value.speed_r = res.speed_r;
        net.value.speed_s = res.speed_s
    })
    document.addEventListener("selectstart",(e)=>{
        e.preventDefault()
    })

    if(systemstore.netspeed.show){
        setTimeout(() => {
            getCurrentWebviewWindow().show()
        }, 2500);
    }
})

watch(systemstore,()=>{
    if(systemstore.netspeed.show){
        getCurrentWebviewWindow().show()
    }else{
        getCurrentWebviewWindow().hide()
    }
})
</script>

<template>
    <div class="window" data-tauri-drag-region>
        <div data-tauri-drag-region style="width: 100px;display: flex;align-items: center;">
            <v-icon data-tauri-drag-region>mdi-arrow-down-thin</v-icon>{{ Math.trunc(net.speed_r/1024)<1024?Math.trunc(net.speed_r/1024)+'KB/s':Math.trunc(net.speed_r/1024/1024*10)/10+'MB/s' }}
        </div>
        <div style="width: 100px;display: flex;align-items: center;">
            <v-icon data-tauri-drag-region>mdi-arrow-up-thin</v-icon>{{ Math.trunc(net.speed_s/1024)<1024?Math.trunc(net.speed_s/1024)+'KB/s':Math.trunc(net.speed_s/1024/1024*10)/10+'MB/s' }}
        </div>
    </div>
</template>

<style>
.window{
    width: 100vw;
    height: 100vh;
    background: rgba(223,223,223,0.8);
    font-size: 13px;
    color:black;
    display: flex;
    justify-content: center;
    flex-direction: column;
    cursor: default;
    align-items: flex-start;
}
</style>