<script setup lang="ts">
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { ref, onMounted } from "vue";
const toggle = ref(false);

onMounted(async () => {
  toggle.value = await getCurrentWebviewWindow().isMaximized();
});

const toggleMaximize = function () {
  getCurrentWebviewWindow().toggleMaximize();
  toggle.value = !toggle.value;
};
</script>

<template>
  <div @click="toggleMaximize" class="toggleMaximize">
    <img v-show="!toggle" src="./../assets/FluentSquare48Regular.png" draggable="false" style="width: 100%" />
    <img v-show="toggle" src="./../assets/FluentSquareMultiple48Regular.png" draggable="false" style="width: 100%" />
  </div>
</template>

<style>
.toggleMaximize {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
}
</style>
