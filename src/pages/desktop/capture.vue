<script setup lang="ts">
import { getAllWebviewWindows, getCurrentWebviewWindow, WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { Canvas, Rect, Point } from 'fabric'; // v6
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { currentMonitor, LogicalPosition, LogicalSize, Monitor } from '@tauri-apps/api/window';
import { resolve, videoDir } from '@tauri-apps/api/path';
import moment from 'moment';
import { emit } from '@tauri-apps/api/event';
let app = getCurrentWebviewWindow();
let c: Canvas
let rect: Rect
const brPoint = ref<{
    x: number,
    y: number
}>()
const startPoint = ref<Point>()
const status = ref(false)
const monitor = ref<Monitor>()
const show = ref(false)
let window_size: {
    x: number,
    y: number,
    width: number,
    height: number,
}
const captureWindow = ref<WebviewWindow>();
const captureBtn = ref<WebviewWindow>();
onMounted(async () => {
    let all = await getAllWebviewWindows();
    all.filter(item => {
        if (item.label == "captureWindow") {
            captureWindow.value = item
        }
        if (item.label == "captureBtn") {
            captureBtn.value = item
        }
    })
    let m = await currentMonitor()
    if (m) {
        monitor.value = m
    }
    let dom = document.getElementById('app')
    app.listen("tauri://blur", () => {
        show.value = false,
            status.value = false
        if (rect) {
            c.remove(rect)
        }
    })
    c = new Canvas("c", {
        width: dom?.clientWidth,
        height: dom?.clientHeight,
    })
    c.renderAll()
    CanvasListen()
})

const CanvasListen = function () {
    c.on("mouse:down", (e) => {
        if (rect) {
            c.remove(rect)
        }
        if (captureWindow.value) {
            captureWindow.value.hide()
        }
        show.value = false
        status.value = true
        startPoint.value = e.scenePoint
        rect = new Rect({
            width: 0,
            height: 0,
            left: e.scenePoint.x,
            top: e.scenePoint.y,
            hasControls: false,
            selectable: false,
            fill: null,
            borderColor: 'black',
            stroke: null
        })
        c.add(rect)
        c.setActiveObject(rect)
        c.requestRenderAll()
    })
    c.on("mouse:move", (e) => {
        if (!status) return
        if (startPoint.value) {
            rect.set({
                width: e.scenePoint.x - startPoint.value.x,
                height: e.scenePoint.y - startPoint.value.y
            })
        }
    })
    c.on("mouse:up", () => {
        status.value = false
        rect.setCoords()
        let dom = document.getElementById('control')
        if (dom) {
            brPoint.value = {
                x: rect.getCenterPoint().x + Math.abs(rect.width) / 2,
                y: rect.getCenterPoint().y + Math.abs(rect.height) / 2
            }

            if (brPoint.value.x <= (c.width - 175)) {
                dom.style.left = (brPoint.value.x - 170) + 'px'
            } else {
                dom.style.left = (brPoint.value.x - 170) + 'px'
            }

            if (brPoint.value.y <= (c.height - 34)) {
                dom.style.top = brPoint.value.y + 'px'
            } else {
                dom.style.top = (brPoint.value.y - 34) + 'px'
            }
            console.log(c.width, brPoint.value, rect.height)
        }
        window_size = {
            x: Math.trunc(rect.getCenterPoint().x - Math.abs(rect.width) / 2),
            y: Math.trunc(rect.getCenterPoint().y - Math.abs(rect.height) / 2),
            width: Math.trunc(Math.abs(rect.width)),
            height: Math.trunc(Math.abs(rect.height)),
        }
        show.value = true
    })
}

const close = async function () {
    let all = await getAllWebviewWindows();
    all.filter(item => {
        if (item.label.indexOf("capture-") == 0 && item.label != app.label) {
            item.destroy();
        }
    })
    app.destroy()
}

const start_capture = async function () {
    if (captureWindow.value && monitor.value) {
        captureWindow.value.setIgnoreCursorEvents(true)
        captureWindow.value.setSize(new LogicalSize(window_size.width + 10, window_size.height + 10))
        captureWindow.value.setPosition(new LogicalPosition(
            window_size.x + monitor.value.position.x / monitor.value.scaleFactor - 5,
            window_size.y + monitor.value.position.y / monitor.value.scaleFactor - 5,
        ))
        captureBtn.value?.setSize(new LogicalSize(80, 30))
        captureBtn.value?.setPosition(new LogicalPosition(
            window_size.x + monitor.value.position.x / monitor.value.scaleFactor + window_size.width / 2 - 40,
            window_size.y + monitor.value.position.y / monitor.value.scaleFactor - 30,
        ));
        let title = moment().format("YYMMDDhhmmss") + '.mp4'
        let path = await resolve(await videoDir(),"skydesk2",title)
        await invoke("start_capture", {
            x: Math.trunc(window_size.x * monitor.value.scaleFactor),
            y: Math.trunc(window_size.y * monitor.value.scaleFactor),
            width: Math.trunc(window_size.width * monitor.value.scaleFactor),
            height: Math.trunc(window_size.height * monitor.value.scaleFactor),
            monitorname: monitor.value?.name,
            path: path
        })
        await emit("capturemessage",{
            name:title,
            path:path
        })
        captureWindow.value.setShadow(true)
        captureWindow.value.show()
        captureBtn.value?.show()
        close()
    }
}

</script>

<template>
    <div class="window">
        <div v-show="show" id="control"
            style="position: absolute;z-index: 100;display: flex;justify-content: flex-end;align-items: center;box-sizing: border-box;padding: 3px;">
            <v-btn style="margin-right: 10px;font-size: 12px;" size="small" @click="start_capture">开始录制</v-btn>
            <v-btn style="margin-right: 10px;font-size: 12px;" size="small" @click="close">退出录制</v-btn>
        </div>
        <canvas id="c"></canvas>
    </div>
</template>

<style>
.window {
    width: 100vw;
    height: 100vh;
    background: rgba(223, 223, 223, 0.1);
}
</style>