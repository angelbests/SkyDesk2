<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { onMounted, ref } from "vue";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { systemStore } from "../stores/window";
import type { NetSpeed } from "../types/netspeedType";
import type { Event } from "@tauri-apps/api/event";
getCurrentWebviewWindow().show();
const system = systemStore();
const net = ref<NetSpeed>({
  speed_r: 0,
  speed_s: 0,
});
const cpu = ref(0);
const memory = ref(0);
onMounted(async () => {
  listen("netspeed", (e: Event<NetSpeed>) => {
    net.value.speed_r = e.payload.speed_r;
    net.value.speed_s = e.payload.speed_s;
  });
  listen("cpu", (e) => {
    let str = e.payload;
    cpu.value = Math.trunc(Math.round(Number(str)));
  });
  listen("memory", (e) => {
    let str = e.payload;
    memory.value = Math.trunc(Number(str) * 100);
  });
  if (system.taskbar) {
    getCurrentWebviewWindow().show();
  } else {
    getCurrentWebviewWindow().hide();
  }
});
window.addEventListener("storage", (e) => {
  if (e.key == "system") {
    system.$hydrate();
    if (system.taskbar) {
      getCurrentWebviewWindow().show();
    } else {
      getCurrentWebviewWindow().hide();
    }
  }
});
</script>

<template>
  <div class="window">
    <div class="item">
      <v-icon>mdi-arrow-down-thin</v-icon>{{
        Math.trunc(net.speed_r / 1024) < 1024 ? Math.trunc(net.speed_r / 1024) + "KB/s" : Math.trunc((net.speed_r / 1024 /
          1024) * 10) / 10 + "MB/s" }} </div>
        <div class="item">
          <div style="width: 30px">CPU</div>
          {{ cpu }}%
        </div>
        <div class="item">
          <v-icon>mdi-arrow-up-thin</v-icon>{{
            Math.trunc(net.speed_s / 1024) < 1024 ? Math.trunc(net.speed_s / 1024) + "KB/s" : Math.trunc((net.speed_s /
              1024 / 1024) * 10) / 10 + "MB/s" }} </div>
            <div class="item">
              <div style="width: 30px">内存</div>
              {{ memory }}%
            </div>
        </div>
</template>

<style>
:root {
  --margin-top: 4px;
}

.window {
  background: transparent;
  width: 100vw;
  height: 100vh;
  font-size: 13px;
  color: black;
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  cursor: default;
  padding: var(--margin-top);
}

.item {
  width: calc(50vw - var(--margin-top));
  height: calc(50vh - var(--margin-top));
  display: flex;
  justify-content: flex-start;
  align-items: center;
}
</style>
