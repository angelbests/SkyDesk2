<script setup lang="ts">
import { LogicalSize } from "@tauri-apps/api/dpi";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { exit, relaunch } from "@tauri-apps/plugin-process";
import { uuid } from "../../functions";
import { createWindow } from "../../functions/window";
getCurrentWebviewWindow().setSize(new LogicalSize(100, 110));
const createnote = async function () {
  let label = "note-" + uuid();
  let w = await createWindow(label, {
    x: 200,
    y: 200,
    width: 330,
    height: 330,
    minWidth: 330,
    minHeight: 100,
    shadow: false,
    decorations: false,
    transparent: true,
    skipTaskbar: true,
    url: "/#/pages/desktop/note",
    title: "note"
  });
  w?.center()
};
</script>

<template>
  <div class="window">
    <v-btn style="width: 100px; height: 30px; margin-bottom: 5px" size="small" @click="createnote">便签</v-btn>
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
