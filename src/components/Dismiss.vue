<script setup lang="ts">
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
const dismiss = function () {
  getCurrentWebviewWindow().close();
};
</script>

<template>
  <div @click="dismiss" class="dismiss">
    <img src="./../assets/FluentDismiss48Regular.png" draggable="false" style="width: 100%" />
  </div>
</template>

<style>
.dismiss {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
}
</style>
