<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { initWindow } from '../functions/window';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { exit, relaunch } from '@tauri-apps/plugin-process';
import { Monitor } from '@tauri-apps/api/window';
import { createtray,traystart } from '../functions/tray';
import { listen } from '@tauri-apps/api/event';
import { systemStore,windowStore,noteStore,wallpaperStore,shortcutStore  } from '../stores/window';
import { disable, enable } from '@tauri-apps/plugin-autostart';
import { invoke } from '@tauri-apps/api/core';
import { register,isRegistered } from "@tauri-apps/plugin-global-shortcut"
const systemstore= systemStore()
const monitors = ref<{  
    title:string, 
    value:string,
    icon:string 
    monitor?: Monitor
}[]>([])
const windowstore = windowStore()
const drawer = ref(true)
const settingshow = ref(false)
const net = ref({
    speed_r:0,
    speed_s:0
})
const toggleMaximizeBool = ref(false)
onMounted(async ()=>{  
    let res =await isRegistered('Control+1')   
    if(!res){
        register('Control+1',async ()=>{
            await getCurrentWebviewWindow().show();
            await getCurrentWebviewWindow().setFocus()
        })
    }
    res =await isRegistered('Control+2')
    if(!res){
        register('Control+2',async ()=>{
            await getCurrentWebviewWindow().hide();
        })
    }
    invoke("netspeed")
    listen("netspeed",(e)=>{
        let res = JSON.parse(e.payload as string)
        net.value.speed_r = res.speed_r;
        net.value.speed_s = res.speed_s;
    })
    // 创建托盘
    await createtray()
    await traystart()
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
    document.addEventListener("selectstart",(e)=>{
        e.preventDefault();
    })
})

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

const autostartsetting = function(e: any){
    if(e){
        enable()
    }else{
        disable()
    }
}

const refresh = function(){
    const system = systemStore()
    system.autostart = false
    system.traystart = false
    localStorage.clear()
    const window = windowStore()
    window.windows = []
    window.monitors = []
    const note = noteStore()
    note.note = []
    const wallpaper = wallpaperStore()
    wallpaper.config = []
    wallpaper.wallpaperList = []
    wallpaper.status = false
    const shortcut = shortcutStore()
    shortcut.shortcutsTemp = []
    shortcut.shortcuts = []
    shortcut.wheels = []
    relaunch()
}
</script>

<template> 
    <v-layout style="background: white;"> 
        <v-app-bar :absolute="true" :height="48" id="toolbar" class="toolbar" title="SkyDesk2" data-tauri-drag-region>
            <template v-slot:prepend>
                <v-app-bar-nav-icon @click="drawer=!drawer"></v-app-bar-nav-icon>
            </template>
            <div style="width: 100px;display: flex;align-items: center;" data-tauri-drag-region>
                <v-icon data-tauri-drag-region>mdi-arrow-down-thin</v-icon>{{ Math.trunc(net.speed_r/1024)<1024?Math.trunc(net.speed_r/1024)+'KB/s':Math.trunc(net.speed_r/1024/1024*10)/10+'MB/s' }}
            </div>
            <div style="width: 100px;display: flex;align-items: center;" data-tauri-drag-region>
                <v-icon data-tauri-drag-region>mdi-arrow-up-thin</v-icon>{{ Math.trunc(net.speed_s/1024)<1024?Math.trunc(net.speed_s/1024)+'KB/s':Math.trunc(net.speed_s/1024/1024*10)/10+'MB/s' }}
            </div>
            <v-btn icon >
                <v-icon>mdi-help-circle-outline</v-icon>
            </v-btn>
            <v-btn icon @click="settingshow = true">
                <v-icon>mdi-cog-outline</v-icon>
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
                <v-list style="height: 100%;" :items="monitors">
                    <v-list-item prepend-icon="mdi-apps" title="快捷" :href="'/#/pages/setting/shortcut'"></v-list-item>
                    <v-list-item prepend-icon="mdi-wallpaper" title="壁纸" :href="'/#/pages/setting/wallpaper'"></v-list-item>
                    <v-list-item prepend-icon="mdi-note-outline" title="便签" :href="'/#/pages/setting/note'"></v-list-item>
                    <v-list-item prepend-icon="mdi-robot-outline" title="AI" :href="'/#/pages/setting/ollama'"></v-list-item>
                    <v-list-item prepend-icon="mdi-wallpaper" title="录屏" :href="'/#/pages/setting/capture'"></v-list-item>
                    <v-list-item prepend-icon="mdi-calendar-range" title="日历" :href="'/#/pages/setting/datenote'"></v-list-item>
                    <!-- <v-list-item prepend-icon="mdi-note-outline" title="系统" :href="'/#/pages/setting/system'"></v-list-item> -->
                    <!-- <v-list-item prepend-icon="mdi-note-outline" title="剪贴板" :href="''"></v-list-item> -->
                </v-list>
            </v-navigation-drawer>
            <router-view v-slot="{ Component }" :key="$route.fullPath" style="width: auto;height: 100%;box-sizing: border-box;padding: 10px;">
                <transition name="fade" mode="out-in" appear>
                    <keep-alive>
                        <component :is="Component" />
                    </keep-alive>
                </transition>
            </router-view>
        </v-main>

        <v-dialog v-model="settingshow" >
            <div style="display: flex;justify-content: center" @click.self="settingshow = false">
                <v-card style="width: 400px;">
                    <v-card-title>
                        <div style="display: flex;flex-direction: row;">
                            <div style="width: 50%;text-align: left;">
                                设置
                            </div>
                            <div style="width: 50%;text-align: right;">
                                <v-icon icon="mdi-window-close" @click="settingshow = false"></v-icon>
                            </div>
                        </div>
                    </v-card-title>
                    <v-card-item>
                        <v-list lines="one" select-strategy="classic">
                            <v-list-item >
                                <template v-slot:append>
                                    <v-switch color="info" v-model="systemstore.autostart" @update:model-value="autostartsetting" hide-details></v-switch>
                                </template>
                                <v-list-item-title>开机自启</v-list-item-title>
                            </v-list-item>
                        </v-list>
                        <v-list lines="one" select-strategy="classic">
                            <v-list-item >
                                <template v-slot:append>
                                    <v-switch color="info" v-model="systemstore.traystart" hide-details></v-switch>
                                </template>
                                <v-list-item-title>启动到托盘</v-list-item-title>
                            </v-list-item>
                        </v-list>
                        <v-list lines="one" select-strategy="classic">
                            <v-list-item >
                                <template v-slot:append>
                                    <v-switch color="info" v-model="systemstore.netspeed.show" hide-details></v-switch>
                                </template>
                                <v-list-item-title>网速控件</v-list-item-title>
                            </v-list-item>
                        </v-list>
                        <v-list lines="one" select-strategy="classic" >
                            <v-list-item>
                                <template v-slot:append>
                                    <v-btn @click="refresh">
                                        <template v-slot:prepend>
                                            <v-icon>mdi-refresh</v-icon>
                                        </template>
                                        清除
                                    </v-btn>
                                </template>
                                <v-list-item-title>清除用户信息</v-list-item-title>
                            </v-list-item>
                        </v-list>
                    </v-card-item>
                    <v-card-actions>
                        
                    </v-card-actions>
                </v-card>
            </div>
        </v-dialog>
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

.fade-leave-active,
.fade-enter-active {
  transition: all 0.3s;
}
 
.fade-enter-from {  
  opacity: 0;
  transform: translateX(-30px);
}
.fade-enter-to { 
  opacity: 1;
  transform: translateX(0px);
}
 
.fade-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
</style>