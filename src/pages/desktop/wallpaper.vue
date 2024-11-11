<script setup lang="ts" >
import { onMounted, ref } from 'vue';
import { screenshot } from '../../functions';
import { useRoute } from 'vue-router';
import { convertFileSrc } from '@tauri-apps/api/core';
const route = useRoute()
const show = ref(false)
const path = ref()
const type = ref()
onMounted(async ()=>{
    path.value = route.query.path
    type.value = route.query.type
    setTimeout(async ()=>{
        show.value = true
        screenshot()
    },500)
})
</script>

<template>
    <div class="window" >
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