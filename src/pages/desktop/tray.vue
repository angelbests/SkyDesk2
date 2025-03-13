<script setup lang="ts">
import { LogicalSize } from "@tauri-apps/api/dpi";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { exit, relaunch } from "@tauri-apps/plugin-process";
getCurrentWebviewWindow().setSize(new LogicalSize(100, 100));
</script>

<template>
  <div class="window">
    <v-btn style="width: 100px; height: 30px; margin-bottom: 5px" size="small" @click="relaunch()">重启</v-btn>

    <v-btn style="width: 100px; height: 30px" size="small" @click="exit()">退出</v-btn>
  </div>
</template>

<style>
.window {
  width: 100%;
  height: 100%;
  background: rgba(245, 245, 245, 0.5);
  box-sizing: border-box;
  padding: 5px;
  border-radius: 5px;
}
</style>
