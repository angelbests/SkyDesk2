<script setup lang="ts">
import { PhysicalPosition } from '@tauri-apps/api/dpi';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { onMounted, reactive, ref } from 'vue';
import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import { Command } from '@tauri-apps/plugin-shell';
const shortcutstore = ref<{
    wheels:any[]
}>({
    wheels:[]
})
const app = getCurrentWebviewWindow();
app.setShadow(false)
const position = reactive({
    x: 0,
    y: 0,
})
onMounted(async () => {
    document.addEventListener("contextmenu", (e) => {
        e.preventDefault()
    })
    document.addEventListener("selectstart", (e) => {
        e.preventDefault()
    })
    shortcutstore.value = JSON.parse(localStorage.getItem("shortcut") as string)
    if (shortcutstore.value.wheels.length < 8) {
        let j = shortcutstore.value.wheels.length
        for (let i = 0; i < (8 - j); i++) {
            shortcutstore.value.wheels.push({
                targetPath: "",
                iconLocationPeFile: "",
                iconLocation: "",
                lnkPath: "",
                icoPath: "",
                name: "",
            }
            )
        }
    }
    window.addEventListener("storage", (e) => {
        console.log(e)
        if (e.key == "shortcut") {
            shortcutstore.value = JSON.parse(e.newValue as string)
            if (shortcutstore.value.wheels.length < 8) {
                let j = shortcutstore.value.wheels.length
                for (let i = 0; i < (8 - j); i++) {
                    shortcutstore.value.wheels.push({
                        targetPath: "",
                        iconLocationPeFile: "",
                        iconLocation: "",
                        lnkPath: "",
                        icoPath: "",
                        name: "",
                    }
                    )
                }
                console.log(shortcutstore.value.wheels)
            }
        }
    })
    await invoke("wheelclick")
    await listen("wheel-click", async (e: any) => {
        let scaleFactor = await getCurrentWebviewWindow().scaleFactor()
        if (e.payload.message == "ButtonRelease(Middle)") {
            await app.hide()
        } else if (e.payload.message == "ButtonPress(Middle)") {
            await app.setFocus()
            await app.setPosition(new PhysicalPosition(Math.trunc(position.x - 120 * scaleFactor), Math.trunc(position.y - 120 * scaleFactor)))
            await app.show()
        }
    })

    await listen('mouse-move', (event: any) => {
        let str = event.payload.message.substring(11, event.payload.message.length - 1)
        let arr = str.split(',')
        let x = arr[0].split(":")[1]
        let y = arr[1].split(":")[1]
        position.x = parseInt(x)
        position.y = parseInt(y)
    });
})

const index = ref(-2);
const exec = async function (item: any, i: number) {
    index.value = i;
    app.hide()
    if (item.lnkPath) {
        console.log(item)
        let res = await Command.create("powershell", `& "${item.lnkPath}"`, { "encoding": 'GBK' }).execute()
        console.log(res)
    } else {
        await Command.create("powershell", item.targetPath).execute()
    }
    index.value = -2
}

</script>

<template>
    <svg width="240" height="240" viewBox="-120 -120 240 240" xmlns="http://www.w3.org/2000/svg"
        transform="rotate(22.5)">
        <g @mouseenter="exec(item, i)" v-for="(item, i) in shortcutstore.wheels" :transform="'rotate(' + 45 * (1 + i) + ')'">
            <path d="M 0 0 L 120 0 A 120 120 0 0 1 85 85 Z"
                :fill="index == i ? 'rgba(123,123,123,0)' : 'rgba(123,123,123,0.1)'" />
            <image style="border-radius: 50%;width: 40px;height: 40px;" class="image" v-if="item.icoPath ? true : false"
                x="-20px" y="65px" :href="convertFileSrc(item.icoPath)" :transform="'rotate(' + (22.5 - 90) + ')'"></image>
        </g>
        <circle cx='0px' cy='0px' r='30px' fill="white" fill-opacity="1"></circle>
    </svg>
</template>

<style>
.image {
    border-radius: 50%;
}
</style>