<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/core';
import { VueDraggable } from 'vue-draggable-plus'
import RightBar from '../../components/RightBar.vue';
import { getCurrentWebviewWindow, WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { LogicalSize } from '@tauri-apps/api/dpi';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-shell';
const app = getCurrentWebviewWindow()
const show = ref(false)
const shortcutWindows = ref(JSON.parse(localStorage.getItem(app.label) || "{}"));
onMounted(async () => {
    app.hide()
    app.setSize(new LogicalSize(shortcutWindows.value.setting.w, shortcutWindows.value.setting.h))
    app.show();
    show.value = true

    await listen(app.label+"-setting",(e:{
        payload:{key:string,value:string}
    })=>{
        let {key,value} = e.payload
        if(key == "background"){
            shortcutWindows.value.setting[key] = value
        }else if(key == 'blur'){
            shortcutWindows.value.setting[key] = value
        }else if(key == 'r'){
            shortcutWindows.value.setting[key] = value
        }else if(key == 'c'){
            shortcutWindows.value.setting[key] = value
        }else if(key == 'h' || key == 'w'){
            shortcutWindows.value.setting[key] = value
            shortcutWindows.value.setting.font = false
            app.setSize(new LogicalSize(shortcutWindows.value.setting.w,shortcutWindows.value.setting.h))
            shortcutWindows.value.setting.h = shortcutWindows.value.setting.h
        }else if(key == 'font'){
            shortcutWindows.value.setting[key] = value
            if(shortcutWindows.value.setting.font){
                app.setSize(new LogicalSize(shortcutWindows.value.setting.w,shortcutWindows.value.setting.h + 30*shortcutWindows.value.setting.r))
                shortcutWindows.value.setting.h = shortcutWindows.value.setting.h + 30*shortcutWindows.value.setting.r
            }else{
                app.setSize(new LogicalSize(shortcutWindows.value.setting.w,shortcutWindows.value.setting.h - 30*shortcutWindows.value.setting.r))
                shortcutWindows.value.setting.h = shortcutWindows.value.setting.h - 30*shortcutWindows.value.setting.r
            }
        }else if(key == 'alwaysOnTop'){
            if(value){
                app.setAlwaysOnBottom(false)
                app.setAlwaysOnTop(true)
            }else{

                app.setAlwaysOnTop(false),
                app.setAlwaysOnBottom(true)
            }
            shortcutWindows.value.setting[key] = value
        }
        localStorage.setItem(app.label,JSON.stringify(shortcutWindows.value))
        console.log(key,value)
    })

})

const dragover = function (e: DragEvent) {
    e.preventDefault()
}

// 拖拽添加shortcut
const drop = function (e: DragEvent) {
    if (e.dataTransfer?.getData("lnk")) {
        let lnk = JSON.parse(e.dataTransfer?.getData("lnk"))
        let arr = shortcutWindows.value.shortcuts.filter((item: { name: any; }) => {
            return item.name == lnk.name
        })
        if (arr.length == 0) {
            shortcutWindows.value.shortcuts.push(lnk)
        }
    }
    localStorage.setItem(app.label,JSON.stringify(shortcutWindows.value))
}

// 关闭窗口
const close = function () {
    localStorage.removeItem(getCurrentWebviewWindow().label)
    getCurrentWebviewWindow().close()
}

// 打开设置
const opensetting = function () {
    new WebviewWindow(app.label + '-setting', {
        width: 400,
        height: 380,
        center: true,
        transparent: true,
        decorations: false,
        parent:app.label,
        maximizable:false,
        resizable:false,
        url: "/#/pages/desktop/shortcutSetting?label=" + app.label
    })
}

const deleteicon = function(){
    shortcutWindows.value.shortcuts = []
    localStorage.setItem(app.label,JSON.stringify(shortcutWindows.value))
}

const openshorcut = function(item: { lnkPath: string; }){
    open(item.lnkPath)
}

const mouseenter = function(i:number){
    let el =  document.getElementById('img'+i);
    if(el){
        el.style.width = el.clientWidth + 10 + 'px'
        el.style.height = el.clientHeight + 10 + 'px'
    }
}

const mouseleave = function(i:number){
    let el =  document.getElementById('img'+i);
    if(el){
        el.style.width = shortcutWindows.value.setting.w / shortcutWindows.value.setting.c - 30 + 'px'
        el.style.height = shortcutWindows.value.setting.w / shortcutWindows.value.setting.c - 30 + 'px'
    }
}
</script>

<template>
    <div v-show="show">
        <div class="wallpaper"
            :style="{ background: shortcutWindows.setting.background, backgroundSize: 'cover', backgroundRepeat: 'no-repeat', backgroundPosition: 'center' }">
        </div>
        <div class="window" :style="{ backdropFilter: `blur(${shortcutWindows.setting.blur}px)` }"
            @dragover="dragover($event)" @drop="drop($event)">
            <VueDraggable
                :style="{ width: shortcutWindows.setting.w + 'px', height: shortcutWindows.setting.h + 'px', gridTemplateColumns: `repeat(auto-fill, ${shortcutWindows.setting.w / shortcutWindows.setting.c}px)`, gridTemplateRows: `repeat(auto-fill, ${shortcutWindows.setting.h / shortcutWindows.setting.r}px)` }"
                class="lnklist" v-model="shortcutWindows.shortcuts" :animation="150"
                group="shortcut">
                <div v-for="(item,i) in shortcutWindows.shortcuts" :key="item.lnkPath">
                    <div class="imgdiv"
                        @click="openshorcut(item)"
                        :style="{ width: (shortcutWindows.setting.w / shortcutWindows.setting.c - 10) + 'px', height: (shortcutWindows.setting.h / shortcutWindows.setting.r - (shortcutWindows.setting.font ? 30 : 10)) + 'px' }">
                        <img class="img"
                            @mouseenter="mouseenter(i)"
                            @mouseleave="mouseleave(i)"
                            :id="'img'+i"
                            :style="{ width: (shortcutWindows.setting.w / shortcutWindows.setting.c - 30) + 'px', height: (shortcutWindows.setting.h / shortcutWindows.setting.r - (shortcutWindows.setting.font ? 60 : 30)) + 'px' }"
                            :src="item.icoPath == '' ? '/icons/ToggleMaximize1.png' : convertFileSrc(item.icoPath)" />
                    </div>
                    <div v-show="shortcutWindows.setting.font"
                        :style="{
                            fontSize: '12px', height: '30px', textWrap: 'balance', textAlign: 'center', width: (shortcutWindows.setting.w / shortcutWindows.setting.c - 10) + 'px',
                            textOverflow: 'clip', overflow: 'hidden', lineHeight: '15px', filter: 'drop-shadow(0px 5px 5px gray)'
                        }">
                        {{ item.name }}
                    </div>
                </div>
            </VueDraggable>
            <right-bar :border-radius="'15px'">
                <div style="display: flex;justify-content: space-evenly;align-content: space-evenly;width: 100%;height: 100%;flex-wrap: wrap;"
                    data-tauri-drag-region>
                    <v-btn @click="close" size="small" icon=" mdi-close"></v-btn>
                    <v-btn @click="opensetting" size="small" icon="mdi-cog-outline"></v-btn>
                    <v-btn @click="deleteicon" size="small" icon="mdi-delete-outline"></v-btn>
                </div>
            </right-bar>
        </div>
    </div>
</template>

<style>
.wallpaper {
    width: 100vw;
    height: 100vh;
    border-radius: 15px;
}

.window {
    width: 100vw;
    height: 100vh;
    border-radius: 5px;
    position: absolute;
    left: 0;
    top: 0;
    z-index: 200;
}

.lnklist {
    display: grid;
    grid-template-columns: repeat(auto-fill, 60px);
    grid-template-rows: repeat(auto-fit, 60px);
    grid-auto-flow: row;
    justify-items: center;
    align-items: center;
    justify-content: center;
    transition: all 0.1s linear;
}

.imgdiv {
    display: flex;
    justify-content: center;
    align-items: center;
    filter: drop-shadow(0px 0px 5px rgba(123,123,123,0.7));
}

.img {
    border-radius: 5px;
    transition: all 0.1s linear;
}
</style>