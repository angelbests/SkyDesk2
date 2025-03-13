<script setup lang="ts">
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

const minus = function () {
  getCurrentWebviewWindow().minimize();
};
</script>

<template>
  <div @click="minus" class="minus">
    <img src="./../assets/FluentSubtract48Regular.png" draggable="false" style="width: 100%" />
  </div>
</template>

<style>
.minus {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
}
</style>
