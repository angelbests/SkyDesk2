<script setup lang="ts" >
import { windowStore } from '../../stores/window';
import { storeToRefs } from 'pinia';
import { uuid } from '../../functions';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
const windowstore = windowStore()
const { monitors } = storeToRefs(windowstore)
const selectcapture = function(){
    let wins = [];
    monitors.value.forEach(item=>{
        let label = "capture-"+uuid()
        wins.push(
            new WebviewWindow(label,{
                x:item.position.x/item.scaleFactor,
                y:item.position.y/item.scaleFactor,
                fullscreen:true,
                shadow:false,
                decorations:false,
                transparent:true,
                alwaysOnTop:true,
                url:"/#/pages/desktop/capture",
                title:"",
            })
        )
    })
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

        </div>
    </div>
</template>

<style>
.window{
    width: 100%;
    height: 100%;
}
</style>