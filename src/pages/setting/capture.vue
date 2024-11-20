<script setup lang="ts" >
import { captureStore, windowStore } from '../../stores/window';
import { storeToRefs } from 'pinia';
import { uuid } from '../../functions';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { onMounted } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/core';
const windowstore = windowStore()
const { monitors } = storeToRefs(windowstore)
const capturestroe = captureStore()
onMounted(()=>{
    window.addEventListener("storage",(e)=>{
        if(e.key == "capture"){
            capturestroe.$hydrate()
        }
    })
})
const selectcapture = function(){
    let wins = [];
    monitors.value.forEach(item=>{
        let label = "capture-"+uuid()
        wins.push(
            new WebviewWindow(label,{
                x:item.position.x/item.scaleFactor,
                y:item.position.y/item.scaleFactor,
                width:item.size.width/item.scaleFactor,
                height:item.size.height/item.scaleFactor,
                shadow:false,
                decorations:false,
                transparent:true,
                alwaysOnTop:true,
                url:"/#/pages/desktop/capture",
            })
        )
    })
}
import { open } from '@tauri-apps/plugin-shell';
import { videoDir } from '@tauri-apps/api/path';
import { remove } from '@tauri-apps/plugin-fs';
const openvideo = function(path:string){
    open(path)
}

const opendir =async function(){
    let path = await videoDir() + "\\skydesk2"
    open(path)
}

const delvideo = function(path:string){
    let index = capturestroe.video.findIndex(item=>{
        return item.path == path
    })
    capturestroe.video.splice(index,1)
    remove(path)
}
</script>

<template>
    <div class="window">
        <v-card
            style="width: 100%;height: 60px;display: flex;align-items: center;box-sizing: border-box;padding: 0 20px;filter:drop-shadow(0px 2px 5px gray)">
            <v-btn style="margin-right: 20px;" @click="selectcapture">
                <template v-slot:prepend>
                    <v-icon>mdi-record-circle-outline</v-icon>
                </template>
                录屏
            </v-btn>
        </v-card>
        <div style="width: 100%;height: calc(100% - 60px);display: flex;overflow: hidden;background: white;">
            <div class="video">
                <div class="video-list">
                    <v-card prepend-icon="" width="400" height="305" variant="elevated" elevation="10" v-for="item in capturestroe.video">
                        <v-card-text style="position: relative;">
                            <video style="width: 100%;height: 220px;" :src="convertFileSrc(item.path)"></video>
                            <div style="width: 100%;position: absolute;left: 15px;top: 240px;z-index: 50;color: gray;">
                                {{ item.name }}
                            </div>
                        </v-card-text>
                        <v-card-actions>
                            <v-btn @click="openvideo(item.path)">打开</v-btn>
                            <v-btn @click="opendir">文件夹</v-btn>
                            <v-btn @click="delvideo(item.path)">删除</v-btn>
                        </v-card-actions>
                    </v-card>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.window{
    width: 100%;
    height: 100%;
}

.video{
    width: 100%;
    background: white;
    display: flex;
    justify-content: center;
    overflow-x: hidden;
    overflow-y: scroll;
}
.video-list{
    width: 100%;
    height: auto;
    display: grid;
    grid-template-columns: repeat(auto-fill, 420px);
    grid-template-rows: repeat(auto-fit, 320px);
    grid-auto-flow: row;
    justify-items: center;
    align-items: center;
    justify-content: center;
}
</style>