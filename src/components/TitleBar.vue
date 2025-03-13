<script lang="ts" setup>
import { defineProps, onMounted } from "vue";

defineProps<{
  height?: number | string;
  background?: string;
  borderTopLeftRadius?: string;
  borderTopRightRadius?: string;
}>();

onMounted(() => {
  // document.getElementById('titlebar')?.addEventListener('contextmenu',(e)=>{
  //     e.preventDefault()
  // })
  // document.getElementById('titlebar')?.addEventListener('selectstart',(e)=>{
  //     e.preventDefault()
  // })
});
</script>

<template>
  <div id="titlebar" :style="{
    height: height,
    background: background,
    borderTopLeftRadius: borderTopLeftRadius,
    borderTopRightRadius: borderTopRightRadius,
  }" class="titlebar" data-tauri-drag-region>
    <slot />
  </div>
</template>

<style>
.titlebar {
  width: 100%;
  height: 40px;
  background: red;
  border-top-left-radius: 10px;
  border-top-right-radius: 10px;
}
</style>
