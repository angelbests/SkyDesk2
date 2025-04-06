<script setup lang="ts">
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { onMounted, ref, toRefs } from 'vue';
import { shortcutStore } from '../../stores/window';
import { exec } from "../../functions/open";
import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import GridContainer from '../../components/GridContainer.vue';
import RightBar from '../../components/RightBar.vue';
import { currentMonitor } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';
const shortcutstore = shortcutStore()
const { shortcuts } = toRefs(shortcutstore)
const tab = ref(0);
onMounted(() => {
    window.addEventListener("storage", (e) => {
        if (e.key == "shortcut") {
            shortcutstore.$hydrate();
        }
    });
})

// 关闭窗口
const close = async function () {
    getCurrentWebviewWindow().hide();
    getCurrentWebviewWindow().close();
};

// 监测是否在边上
let loading = false
let show = false
let timer: any;
listen("mouse-move", async (e: { payload: { message: string } }) => {
    console.log(loading)
    if (loading) return
    let { x, y } = JSON.parse(e.payload.message as string);
    let position = await getCurrentWebviewWindow().outerPosition();
    let size = await getCurrentWebviewWindow().outerSize();
    let cmonitor = await currentMonitor();
    if (!position) return;
    if (!size) return;
    if (!cmonitor) return;
    let mouseX = x / cmonitor.scaleFactor
    let mouseY = y / cmonitor.scaleFactor
    let windowLeftTopX = position.x / cmonitor.scaleFactor
    let windowLeftTopY = position.y / cmonitor.scaleFactor
    let windowRightBottomX = (position.x + size.width) / cmonitor.scaleFactor
    let windowRightBottomY = (position.y + size.height) / cmonitor.scaleFactor
    let bool = windowLeftTopX <= mouseX && windowRightBottomX >= mouseX && windowLeftTopY <= mouseY && windowRightBottomY >= mouseY
    if (timer != undefined) {
        clearTimeout(timer)
    }
    if (bool && show) {
        loading = true
        show = false
        invoke("hovertop", { label: "hovertop", show: true })
    } else if (!bool && !show) {
        timer = setTimeout(() => {
            loading = true
            show = true
            invoke("hovertop", { label: "hovertop", show: false })
        }, 100)

    }
})

listen("hovertop_status", (e: { payload: boolean }) => {
    loading = e.payload
})
</script>

<template>
    <div class="main">
        <RightBar :background="'rgba(123,123,123,0.3)'">
            <div style="
            display: flex;
            justify-content: space-evenly;
            align-content: space-evenly;
            width: 100%;
            height: 100%;
            flex-wrap: wrap;
          " data-tauri-drag-region>
                <v-btn @click="close" style="width: 30px; height: 30px" icon>
                    <v-icon size="mini">mdi-close</v-icon>
                </v-btn>
            </div>
        </RightBar>
        <v-tabs id="shortcuttab" density="compact" v-model="tab" center-active style="
            height: 36px;
            width: 100%;
            background: white;
            ;
          " hide-slider show-arrows>
            <v-tab v-for="item in shortcuts" :value="item.index">
                <div style="display: flex; justify-content: center">
                    <div style="width: calc(100% - 20px); margin-right: 10px">
                        {{ item.title }}
                    </div>
                </div>
            </v-tab>
        </v-tabs>
        <v-tabs-window v-model="tab" style="width: 100%; height: calc(100% - 36px); padding: 10px;">
            <v-tabs-window-item v-for="item1 in shortcuts" style="width: 100%; height: 100%; min-height: 100%">
                <GridContainer v-model="item1.shortcut" :animation="150" :gridwidth="90" :gridheight="90"
                    :group="{ name: 'shortcut', pull: 'clone' }">
                    <template v-slot="{ item }">
                        <div :style="{ height: '80px', backgroundSize: 'cover', }" class="shortcut-container">
                            <div class="icon-div" @click="exec(item)">
                                <img class="icon"
                                    :src="item.icoPath == '' ? '/icons/program.png' : convertFileSrc(item.icoPath)" />
                            </div>
                            <div
                                style="font-size: 10px;width: 60px;height: 30px;filter: drop-shadow(0px 5px 5px gray);text-wrap: balance;text-align: center; text-overflow: clip;overflow: hidden;">
                                {{ item.name }}
                            </div>
                        </div>
                    </template>
                </GridContainer>
            </v-tabs-window-item>
        </v-tabs-window>
    </div>
</template>

<style scoped>
.main {
    width: 100vw;
    height: 100vh;
    border-radius: 10px;
    backdrop-filter: blur(12px);
}

.icon:hover {
    width: 40px;
    height: 40px;
}

.shortcut-container {
    width: 80px;
    height: 80px;
    margin: 5px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    border-radius: 10px;
    box-shadow: 0px 0px 15px 2px rgba(223, 223, 223, 0.2);
    transition: height 0.1s linear;
}

.icon-div {
    width: 40px;
    height: 40px;
    display: flex;
    justify-content: center;
    filter: drop-shadow(0px 5px 5px gray);
}

.icon {
    width: 35px;
    height: 35px;
    border-radius: 5px;
    transition: all 0.1s linear;
}
</style>
<style>
html,
body {
    border-radius: 10px;
    background-image: linear-gradient(to right top, #65dfc9, #6cdbeb);
}
</style>