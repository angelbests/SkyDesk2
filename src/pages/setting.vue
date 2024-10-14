<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { initWindow } from '../functions/window';
import { getAllWebviewWindows, getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { exit } from '@tauri-apps/plugin-process';
import { wallpaperStore, windowStore } from '../stores/window';
import { Monitor } from '@tauri-apps/api/window';
import { shortcutStore } from '../stores/window';
import { createtray } from '../functions/tray';
createtray()
const monitors = ref<{
    title:string,
    value:string,
    icon:string
    monitor?: Monitor
}[]>([])
const windowstore = windowStore()
onMounted(async ()=>{
    // 初始化窗口
    await initWindow()
    // 设置窗口拖拽
    document.getElementById("toolbar")?.querySelector(".v-toolbar__content")?.setAttribute("data-tauri-drag-region","true")
    document.getElementById("toolbar")?.querySelector(".v-toolbar-title__placeholder")?.setAttribute("data-tauri-drag-region","true")
    document.getElementById("toolbar")?.addEventListener("selectstart",function(e){
        e.preventDefault()
    }) 
        
    getCurrentWebviewWindow().setShadow(true)
    // 任务栏关闭窗口
    getCurrentWebviewWindow().onCloseRequested(()=>{
        exit()
    })
    
    // getCurrentWebviewWindow().setAlwaysOnTop(true)
    for(let i = 0;i<windowstore.monitors.length;i++){
        monitors.value.push({
            title: "显示器 " + (i+1),
            value: windowstore.monitors[i].name as string,
            monitor: windowstore.monitors[i],
            icon: 'mdi-monitor'
        })
    }
    
    let all =await getAllWebviewWindows();
    all.filter(item=>{
        if(item.label == "tray"){
            item.listen("tauri://blur",(_e)=>{
                item.hide()
            })
        }
    })
})



const toggleMaximizeBool = ref(false)
const toggleMaximize =async function(){
    await getCurrentWebviewWindow().toggleMaximize()
    toggleMaximizeBool.value = await getCurrentWebviewWindow().isMaximized()
}

const minus = async function(){
    await getCurrentWebviewWindow().minimize()
}

const closeApp = async function() {
    if(getCurrentWebviewWindow().label == 'main'){
        getCurrentWebviewWindow().hide()
    }else{
        getCurrentWebviewWindow().destroy()
    }
}

const drawer = ref(true)

const listClick = function(e:{ id: unknown; value: boolean; path: unknown[]; }){
    console.log(e)
}

const  shortcutstore =  shortcutStore()
const reset = async function(){
    wallpaperStore().wallpaperList = []
    wallpaperStore().config = []
    shortcutstore.shortcutsTemp = []
    shortcutstore.shortcuts = []
    windowstore.windows = []
    let res = await getAllWebviewWindows()
    for(let i = 0 ;i<res.length;i++){
        if(res[i].label != 'main' && res[i].label != 'wheel'){
            res[i].destroy()
        }
    }
    // relaunch()
}
</script>

<template> 
    <v-layout style="background: white;"> 
        <v-app-bar :absolute="true" :height="48" id="toolbar" class="toolbar" title="SkyDesk2" data-tauri-drag-region>
            <template v-slot:prepend>
                <v-app-bar-nav-icon @click="drawer=!drawer"></v-app-bar-nav-icon>
            </template>
            <v-btn icon @click="reset">
                <v-icon>mdi-reload</v-icon>
            </v-btn>
            <v-btn icon @click="minus">
                <v-icon>mdi-window-minimize</v-icon>
            </v-btn>
            <v-btn icon @click="toggleMaximize">
                <v-icon v-show="toggleMaximizeBool">mdi-window-restore</v-icon>
                <v-icon v-show="!toggleMaximizeBool">mdi-window-maximize</v-icon>
            </v-btn>
            <v-btn icon @click="closeApp">
                <v-icon>mdi-window-close</v-icon>
            </v-btn>
        </v-app-bar>

        <v-main style="height:calc(100vh);background-color: wheat;overflow-y: auto;width: 100%;">
            <v-navigation-drawer :style="{boxShadow:drawer?'5px 0px 5px rgba(123,123,123,0.5)':'none'}" width="200" temporary v-model="drawer" :permanent="true" expand-on-hover>
                <v-list style="height: 100%;" :items="monitors" @click:select="listClick">
                    <!-- <v-list-item prepend-icon="mdi-cog-outline" title="设置" :href="'/#/pages/setting/system'"></v-list-item> -->

                    <v-list-item prepend-icon="mdi-apps" title="快捷" :href="'/#/pages/setting/shortcut'"></v-list-item>
                    <v-list-item prepend-icon="mdi-wallpaper" title="壁纸" :href="'/#/pages/setting/wallpaper'"></v-list-item>
                    <v-list-item prepend-icon="mdi-note-outline" title="笔记" :href="'/#/pages/setting/note'"></v-list-item>
                    <!-- <v-list-item prepend-icon="mdi-wallpaper" title="录屏" :href="'/#/pages/setting/capture'"></v-list-item> -->
                    <!-- <v-list-item prepend-icon="mdi-application-outline" title="窗口" :href="'/#/pages/setting/window'"></v-list-item>
                    <v-list-group value="显示器">
                        <template v-slot:activator="{ props }">
                            <v-list-item
                                v-bind="props"
                                prepend-icon="mdi-monitor"
                                title="显示器"
                            ></v-list-item>
                        </template>
                        <v-list-item
                            v-for="(item, i) in monitors"
                            :key="i"
                            :prepend-icon="item.icon"
                            :title="item.title as string"
                            :value="item.value"
                            :href="'/#/pages/setting/monitor?name='+(item.monitor?.name as string)"
                        >
                        </v-list-item>
                    </v-list-group> -->
                </v-list>
            </v-navigation-drawer>
            <router-view v-slot="{ Component }" :key="$route.fullPath" style="width: auto;height: 100%;box-sizing: border-box;padding: 10px;">
                <keep-alive>
                    <component :is="Component" />
                </keep-alive>
            </router-view>
        </v-main>
    </v-layout>
</template>

<style>
.home{
    width: 100vw;
    height: 100vh;
    overflow: hidden;
}
.toolbar{
    cursor:default;
    
}
.v-list-group__items{
    --parent-padding:calc(0px)
}
</style>