<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/core';
import { VueDraggable } from 'vue-draggable-plus'
import RightBar from '../../components/RightBar.vue';
import { getCurrentWebviewWindow, WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { LogicalPosition, LogicalSize } from '@tauri-apps/api/dpi';
import { listen } from '@tauri-apps/api/event';
import { Command } from '@tauri-apps/plugin-shell';
import { windowStore } from '../../stores/window';
const show = ref(false)
const shortcutWindows = ref(JSON.parse(localStorage.getItem(getCurrentWebviewWindow().label) || "{}"));
onMounted(async () => {
    await getCurrentWebviewWindow().hide()
    await getCurrentWebviewWindow().setSize(new LogicalSize(shortcutWindows.value.setting.w, shortcutWindows.value.setting.h))
    await getCurrentWebviewWindow().show();
    show.value = true

    await listen(getCurrentWebviewWindow().label + "-setting", (e: {
        payload: { key: string, value: string }
    }) => {
        let { key, value } = e.payload
        if (key == "background") {
            shortcutWindows.value.setting[key] = value
        } else if (key == 'blur') {
            shortcutWindows.value.setting[key] = value
        } else if (key == 'r') {
            shortcutWindows.value.setting[key] = value
        } else if (key == 'c') {
            shortcutWindows.value.setting[key] = value
        } else if (key == 'h' || key == 'w') {
            shortcutWindows.value.setting[key] = value
            shortcutWindows.value.setting.font = false
            getCurrentWebviewWindow().setSize(new LogicalSize(shortcutWindows.value.setting.w, shortcutWindows.value.setting.h))
            shortcutWindows.value.setting.h = shortcutWindows.value.setting.h
        } else if (key == 'font') {
            shortcutWindows.value.setting[key] = value
            if (shortcutWindows.value.setting.font) {
                getCurrentWebviewWindow().setSize(new LogicalSize(shortcutWindows.value.setting.w, shortcutWindows.value.setting.h + 30 * shortcutWindows.value.setting.r))
                shortcutWindows.value.setting.h = shortcutWindows.value.setting.h + 30 * shortcutWindows.value.setting.r
            } else {
                getCurrentWebviewWindow().setSize(new LogicalSize(shortcutWindows.value.setting.w, shortcutWindows.value.setting.h - 30 * shortcutWindows.value.setting.r))
                shortcutWindows.value.setting.h = shortcutWindows.value.setting.h - 30 * shortcutWindows.value.setting.r
            }
        } else if (key == 'alwaysOnTop') {
            if (value) {
                getCurrentWebviewWindow().setAlwaysOnBottom(false)
                getCurrentWebviewWindow().setAlwaysOnTop(true)
            } else {

                getCurrentWebviewWindow().setAlwaysOnTop(false),
                getCurrentWebviewWindow().setAlwaysOnBottom(true)
            }
            shortcutWindows.value.setting[key] = value
        }
    })

    await listen("dellnk",(e:{payload:{label:string,lnk:string}})=>{
        console.log(e.payload)
        if(e.payload.label == getCurrentWebviewWindow().label){
            let lnk = JSON.parse(e.payload.lnk)
            let i = shortcutWindows.value.shortcuts.findIndex((item: { targetPath: any; })=>{
                return item.targetPath == lnk.targetPath
            })
            shortcutWindows.value.shortcuts.splice(i,1)
        }
    })

})

watch(shortcutWindows,()=>{
    localStorage.setItem(getCurrentWebviewWindow().label, JSON.stringify(shortcutWindows.value))
},{
    deep:true
})

const dragover = function (e: DragEvent) {
    e.preventDefault()
}

// 拖拽添加shortcut
const drop = function (e: DragEvent) {
    if (e.dataTransfer?.getData("lnk")){
        let lnk = JSON.parse(e.dataTransfer?.getData("lnk"))
        let arr = shortcutWindows.value.shortcuts.filter((item: { name: any; }) => {
            return item.name == lnk.name
        })
        if (arr.length == 0) {
            shortcutWindows.value.shortcuts.push(lnk)
        }
    }
}

// 关闭窗口
const close =async function () {
    localStorage.removeItem(getCurrentWebviewWindow().label)
    const windowstore = windowStore()
    let i = windowstore.windows.findIndex(item=>{
        return item.label == getCurrentWebviewWindow().label
    })
    getCurrentWebviewWindow().setPosition(new LogicalPosition(windowstore.windows[i].option.x as number,windowstore.windows[i].option.y as number))
    getCurrentWebviewWindow().close()
}

// 打开设置
const opensetting = function () {
    new WebviewWindow(getCurrentWebviewWindow().label + '-setting', {
        width: 400,
        height: 380,
        center: true,
        transparent: true,
        decorations: false,
        parent: getCurrentWebviewWindow().label,
        maximizable: false,
        shadow: false,
        resizable: false,
        url: "/#/pages/desktop/shortcutSetting?label=" + getCurrentWebviewWindow().label
    })
}

const deleteicon = function () {
    shortcutWindows.value.shortcuts.splice(0)
}

const openshorcut = async function (item: {
    targetPath: any; lnkPath: string;
}) {
    if (item.lnkPath) {
        console.log(item)
        let res = await Command.create("powershell", `& "${item.lnkPath}"`, { "encoding": 'GBK' }).execute()
        console.log(res)
    } else {
        await Command.create("powershell", item.targetPath).execute()
    }
}

const mouseenter = function (i: number) {
    let el = document.getElementById('img' + i);
    if (el) {
        el.style.width = el.clientWidth + 5 + 'px'
        el.style.height = el.clientHeight + 5 + 'px'
    }
}

const mouseleave = function (i: number) {
    let el = document.getElementById('img' + i);
    if (el) {
        el.style.width = shortcutWindows.value.setting.w / shortcutWindows.value.setting.c - 20 + 'px'
        el.style.height = shortcutWindows.value.setting.w / shortcutWindows.value.setting.c - 20 + 'px'
    }
}

const setdata = function (d: DataTransfer, h: HTMLElement) {
    if (h.dataset.lnk) {
        d.setData("dellnk", JSON.stringify({
            label:getCurrentWebviewWindow().label,
            lnk:h.dataset.lnk
        }))
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
            <VueDraggable :style="{
                width: shortcutWindows.setting.w + 'px', height: shortcutWindows.setting.h + 'px',
                gridTemplateColumns: `repeat(auto-fill, ${shortcutWindows.setting.w / shortcutWindows.setting.c}px)`,
                gridTemplateRows: `repeat(auto-fill, ${shortcutWindows.setting.h / shortcutWindows.setting.r}px)`
            }"
                class="lnklist" v-model="shortcutWindows.shortcuts" :animation="150" :group="{
                    name: 'shortcut'
                }"
                :setData="setdata"
                >
                <div v-for="(item, i) in shortcutWindows.shortcuts" :key="item.lnkPath" :data-lnk="JSON.stringify(item)">
                    <div class="imgdiv" @click="openshorcut(item)"
                        :style="{ width: (shortcutWindows.setting.w / shortcutWindows.setting.c) + 'px', height: (shortcutWindows.setting.h / shortcutWindows.setting.r - (shortcutWindows.setting.font ? 30 : 0)) + 'px' }">
                        <img class="img" @mouseenter="mouseenter(i)" @mouseleave="mouseleave(i)" :id="'img' + i"
                            :style="{ width: (shortcutWindows.setting.w / shortcutWindows.setting.c - 20) + 'px', height: (shortcutWindows.setting.h / shortcutWindows.setting.r - (shortcutWindows.setting.font ? 50 : 20)) + 'px' }"
                            :src="item.icoPath == '' ? '/icons/program.png' : convertFileSrc(item.icoPath)" />
                    </div>
                    <div v-show="shortcutWindows.setting.font" :style="{
                        fontSize: '10px', height: '30px', textWrap: 'balance', textAlign: 'center', width: (shortcutWindows.setting.w / shortcutWindows.setting.c) + 'px',
                        textOverflow: 'clip', overflow: 'hidden'
                    }">
                        {{ item.name }}
                    </div>
                </div>
            </VueDraggable>
            <right-bar :border-radius="'15px'">
                <div style="display: flex;justify-content: space-evenly;align-content: space-evenly;width: 100%;height: 100%;flex-wrap: wrap;"
                    data-tauri-drag-region>
                    <v-btn @click="close" style="width: 30px;height: 30px;" icon>
                        <v-icon size="mini">mdi-close</v-icon>
                    </v-btn>
                    <v-btn @click="opensetting" style="width: 30px;height: 30px;" icon>
                        <v-icon size="mini">mdi-cog-outline</v-icon>
                    </v-btn>
                    <v-btn @click="deleteicon" style="width: 30px;height: 30px;" icon>
                        <v-icon size="mini">mdi-delete-outline</v-icon>
                    </v-btn>
                </div>
            </right-bar>
        </div>
    </div>
</template>

<style scoped>
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
    filter: drop-shadow(0px 0px 5px rgba(123, 123, 123, 0.7));
}

.img {
    border-radius: 5px;
    transition: all 0.1s linear;
}
</style>