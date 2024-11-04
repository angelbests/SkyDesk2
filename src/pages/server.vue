<script setup lang="ts" >
import { openserver } from '../functions/server';
import { onMounted } from 'vue';
import { wallpaperStore } from '../stores/window';
import { setWindowToMonitor } from "../functions/monitor";

const wallpapers = wallpaperStore()
onMounted(async ()=>{
    await openserver()
    setTimeout(async() => {
        wallpapers.config.forEach(async item=>{
        await setWindowToMonitor(
            item.label, 
            item.monitor.position.x as number,
            item.monitor.position.y as number,
            item.monitor.size.width as number,
            item.monitor.size.height as number
        )
    }) 
    }, 1000);
})
</script >

<template>
    <div style="width: 100%;height: 100%;background: white;">
        server
    </div>
</template>