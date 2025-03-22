<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { computed, onMounted, ref, watch } from "vue";
import type { Event } from "@tauri-apps/api/event";
import { NetSpeed, Netspeed } from "../../functions/sysinfo";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { systemStore } from "../../stores/window";
import { LogicalPosition, LogicalSize } from "@tauri-apps/api/dpi";
import { MouseAction, MouseEvent } from "../../types/desktopType";
import { currentMonitor, Monitor, monitorFromPoint } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { Smtc_Control } from "../../functions/smtc";
import { wallpaperStore, shortcutStore } from "../../stores/window";
const wallpaperstore = wallpaperStore()
const shortcutstore = shortcutStore()
const app = ref("")
let smtc = new Smtc_Control();
const playstatus = ref(5);
const systemstore = systemStore();
const cpu = ref(0);
const memory = ref(0);
const net = ref<NetSpeed>({
  speed_r: 0,
  speed_s: 0,
});
type Rect = {
  x: number,
  y: number,
  width: number,
  height: number,
}
const rect = ref<Rect>({
  x: 0,
  y: 0,
  width: 0,
  height: 0
})
getCurrentWebviewWindow().setPosition(new LogicalPosition(0, 0))
getCurrentWebviewWindow().setSize(new LogicalSize(270, 45))
getCurrentWebviewWindow().setAlwaysOnTop(true)
new Netspeed().listen_netspeed((e) => {
  net.value = e.payload
})
const transmitted = computed(() => {
  return Math.trunc(net.value.speed_s / 1024) < 1024 ? Math.trunc(net.value.speed_s / 1024) + "KB/s" : Math.trunc((net.value.speed_s /
    1024 / 1024) * 10) / 10 + "MB/s"
})
const received = computed(() => {
  return Math.trunc(net.value.speed_r / 1024) < 1024 ? Math.trunc(net.value.speed_r / 1024) + "KB/s" : Math.trunc((net.value.speed_r / 1024 /
    1024) * 10) / 10 + "MB/s"
})
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

listen("taskbar", (e: Event<Rect>) => {
  rect.value = e.payload
})

onMounted(() => {
  window.addEventListener("storage", (e) => {
    if (e.key == "system") {
      systemstore.$hydrate();
    } else if (e.key == "wallpaper") {
      wallpaperstore.$hydrate();
    } else if (e.key == "shortcut") {
      shortcutstore.$hydrate()
    }
  });
})

watch(systemstore, () => {
  if (systemstore.taskbar) {
    getCurrentWebviewWindow().show();
  } else {
    getCurrentWebviewWindow().hide();
  }
}, {
  immediate: true,
  deep: true
});


smtc.listen_appstatus(async (e) => {
  if (
    !e.payload.cloudmusic &&
    wallpaperstore.wallpaperConfig[0].config.musicapp ==
    "cloudmusic.exe"
  ) {
    app.value = "cloudmusic.exe"
  } else if (
    !e.payload.qqmusic &&
    wallpaperstore.wallpaperConfig[0].config.musicapp ==
    "QQMusic.exe"
  ) {
    app.value = "QQMusic.exe"
  } else if (
    !e.payload.spotify &&
    wallpaperstore.wallpaperConfig[0].config.musicapp ==
    "Spotify.exe"
  ) {
    app.value = "Spotify.exe"
  } else {
    app.value = ""
  }
})

smtc.listen_playstatus((e) => {
  if (
    wallpaperstore.wallpaperConfig[0].config.musicapp ==
    e.payload.app
  ) {
    playstatus.value = e.payload.status;
  }
})

import { Command } from "@tauri-apps/plugin-shell";
listen("desktop", async (e: Event<MouseEvent>) => {
  let dom = document.getElementById("previous")
  if (!dom) return;
  let monitor = await monitorFromPoint(e.payload.x, e.payload.y) as Monitor
  let current = await currentMonitor()
  if (monitor?.name != current?.name) {
    return
  };
  let x = e.payload.x
  let y = e.payload.y
  let width = dom.clientWidth * monitor.scaleFactor;
  let height = dom.clientHeight * monitor.scaleFactor;
  if (e.payload.mouse == MouseAction.LeftUp) {
    if (rect.value.x < x && (rect.value.x + width) > x && rect.value.y < y && (rect.value.y + height) > y) {
      if (app.value) {
        shortcutstore.shortcutsTemp.filter((e) => {
          if (e.targetPath.indexOf(app.value) > 0) {
            Command.create("powershell", `& "${e.targetPath}"`, {
              encoding: "GBK",
            }).execute();
          }
        })
      } else {
        invoke("play_control", { appname: wallpaperstore.wallpaperConfig[0].config.musicapp, control: -1 })
      }
    } else if ((rect.value.x + width) < x && (rect.value.x + width * 2) > x && rect.value.y < y && (rect.value.y + height) > y) {
      if (app.value) {
        shortcutstore.shortcutsTemp.filter((e) => {
          if (e.targetPath.indexOf(app.value) > 0) {
            Command.create("powershell", `& "${e.targetPath}"`, {
              encoding: "GBK",
            }).execute();
          }
        })
      } else {
        invoke("play_control", { appname: wallpaperstore.wallpaperConfig[0].config.musicapp, control: 0 })
      }
    } else if ((rect.value.x + width * 2) < x && (rect.value.x + width * 3) > x && rect.value.y < y && (rect.value.y + height) > y) {
      if (app.value) {
        shortcutstore.shortcutsTemp.filter((e) => {
          if (e.targetPath.indexOf(app.value) > 0) {
            Command.create("powershell", `& "${e.targetPath}"`, {
              encoding: "GBK",
            }).execute();
          }
        })
      } else {
        invoke("play_control", { appname: wallpaperstore.wallpaperConfig[0].config.musicapp, control: 1 })
      }
    }
  }
})

</script>

<template>
  <div class="window">
    <div class="music">
      <div id="previous" style="height:100%;display: flex;align-items: center;">
        <v-icon>mdi-skip-previous</v-icon>
      </div>
      <div id="play" style="height: 100%;display: flex;align-items: center;">
        <v-icon v-if="playstatus == 4">mdi-pause</v-icon>
        <v-icon v-else>mdi-play</v-icon>
      </div>

      <div id="next" style="height: 100%;display: flex;align-items: center;">
        <v-icon>mdi-skip-next</v-icon>
      </div>
    </div>
    <div class="sysinfo">
      <div class="item">
        <v-icon>mdi-arrow-down-thin</v-icon>{{
          received }}
      </div>
      <div class="item">
        <div style="width: 30px">CPU</div>
        {{ cpu }}%
      </div>
      <div class="item">
        <v-icon>mdi-arrow-up-thin</v-icon>{{ transmitted }}
      </div>
      <div class="item">
        <div style="width: 30px">内存</div>
        {{ memory }}%
      </div>
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

.music {
  width: 40vw;
  height: 100vh;
  display: flex;
  justify-content: space-evenly;
  align-content: space-evenly;
  font-size: 25px;
}

.sysinfo {
  position: relative;
  top: 4px;
  width: calc(60vw - 15px);
  margin-left: 15px;
  height: calc(100vh - 8px);
  display: grid;
  grid-template-columns: 55% 45%;
  grid-template-rows: repeat(2, calc(50vh - 4px));
}

.item {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: start;
  align-items: start;
}
</style>
