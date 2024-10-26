<script setup lang="ts" >
import { listen } from '@tauri-apps/api/event';
import { onMounted, ref } from 'vue';
const net = ref({
    speed_r:0,
    speed_s:0
})
onMounted(()=>{
    listen("netspeed",(e:{payload:{speed_r:number,speed_s:number}})=>{
        console.log(e.payload)
        net.value.speed_r = e.payload.speed_r;
        net.value.speed_s = e.payload.speed_s
    })
})
</script>

<template>
    <div class="window">
        {{ net.speed_r }}
        {{ net.speed_s }}
    </div>
</template>

<style>
.window{
    width: 100%;
    height: 100%;
}
</style>