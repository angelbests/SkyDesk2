<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { onMounted, ref } from "vue";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
getCurrentWebviewWindow().show();
const net = ref({
  speed_r: 0,
  speed_s: 0,
});
const cpu = ref(0);
const memory = ref(0);
onMounted(async () => {
  listen("netspeed", (e) => {
    let res = JSON.parse(e.payload as string);
    net.value.speed_r = res.speed_r;
    net.value.speed_s = res.speed_s;
  });
  listen("cpu", (e) => {
    let str = e.payload;
    cpu.value = Math.trunc(Math.round(Number(str)));
  });
  listen("memory", (e) => {
    let str = e.payload;
    memory.value = Math.trunc(Number(str) * 100);
  });
});
</script>

<template>
  <div class="window">
    <div class="item">
      <v-icon>mdi-arrow-down-thin</v-icon
      >{{
        Math.trunc(net.speed_r / 1024) < 1024
          ? Math.trunc(net.speed_r / 1024) + "KB/s"
          : Math.trunc((net.speed_r / 1024 / 1024) * 10) / 10 + "MB/s"
      }}
    </div>
    <div class="item">
      <div style="width: 30px">CPU</div>
      {{ cpu }}%
    </div>
    <div class="item">
      <v-icon>mdi-arrow-up-thin</v-icon
      >{{
        Math.trunc(net.speed_s / 1024) < 1024
          ? Math.trunc(net.speed_s / 1024) + "KB/s"
          : Math.trunc((net.speed_s / 1024 / 1024) * 10) / 10 + "MB/s"
      }}
    </div>
    <div class="item">
      <div style="width: 30px">内存</div>
      {{ memory }}%
    </div>
  </div>
</template>

<style>
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
}
.item {
  width: 50vw;
  height: 50vh;
  display: flex;
  justify-content: flex-start;
  align-items: center;
  /* border: 1px solid black; */
}
</style>
