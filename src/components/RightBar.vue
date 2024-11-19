<script lang="ts" setup>
import { onMounted, defineModel, defineProps } from "vue";
defineProps<{
    background?:string
    borderRadius?:number|string
}>()
const show = defineModel('show',{
  "default":false
})

const emit = defineEmits({
  rightClick:()=>{}
})

onMounted(async ()=>{
  document.addEventListener("contextmenu",(e)=>{
    e.preventDefault();
    emit('rightClick');
    show.value = !show.value
  })
})
</script>

<template>
    <div :style="{background:background,borderRadius:borderRadius}" v-show="show" class="rightbar" data-tauri-drag-region>
      <slot></slot>
    </div>
</template>

<style>
.rightbar{
    position: absolute;
    z-index: 1000;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    background: rgba(123,133,133,0.3);
    border-radius: 10px;
    transition: all 0.5s linear;
}
</style>
