<script setup lang="ts" >
import { onMounted, ref } from 'vue';
import { screenshot } from '../../functions';
import { useRoute } from 'vue-router';
import { convertFileSrc } from '@tauri-apps/api/core';
const route = useRoute()
const show = ref(false)
const path = ref()
const type = ref()
import { currentMonitor } from '@tauri-apps/api/window';
const monitor = ref()
onMounted(async ()=>{
    path.value = route.query.path
    type.value = route.query.type
    setTimeout(async ()=>{
        show.value = true
        screenshot()
        monitor.value = await currentMonitor ()
    },500)
})
</script>

<template>
    <div class="window" >
        <div style="position: absolute;z-index: 99;left: 100;top: 100;">
            {{ JSON.stringify(monitor) }}
        </div>
        <img v-if="type=='image'" :src="convertFileSrc(path)" class="image"/>
        <video muted v-else class="video" id="video" ref="video" :src="convertFileSrc(path)" autoplay="true" loop="true"></video>
    </div>
</template>

<style>
.window{
    width: 100vw;
    height: 100vh;
    background-color: black;
}
.image{
    width: 100vw;
    height: 100vh;
    object-fit: cover;
}
.video{
    width: 100vw;
    height: 100vh;
    object-fit: cover;
}
</style>