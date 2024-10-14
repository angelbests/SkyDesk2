<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRoute } from 'vue-router';
import {  Monitor} from '@tauri-apps/api/window';
import { windowStore } from '../../stores/window';
import { appDataDir, resolve } from '@tauri-apps/api/path';
import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import { uuid } from '../../functions';
const route = useRoute()
const name = ref()
const windowstore = windowStore()
const monitor = ref<Monitor>()
const image = ref()
onMounted(async () => {
  name.value = route.query.name as string;
  windowstore.monitors.filter((item) => {
    if (item.name === name.value) {
      monitor.value = item
    }
  })
  let path = await resolve(await appDataDir(),"window",name.value.substring(4,12)+'.png',)
  console.log(path)
  await invoke("get_display_capture",{display:name.value,path:path})
  image.value = convertFileSrc(path)+"?"+uuid()
})

</script>

<template>
  <div class="wallpaper">
      <div style="display: flex;align-items: center;width: 100%;height: 100%;justify-content: center;background: gray;">
        <div :style="{ width:'99%',aspectRatio: monitor?.size.width as number / (monitor?.size.height as number), background:`url(`+image+`)`,backgroundSize:'cover'}">
        </div>
      </div>
  </div>
</template>

<style>
.wallpaper {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: row;
}

@media (aspect-ratio: 1/1) {
  div {
    background: #f9a;
    /* red */
  }
}
</style>
