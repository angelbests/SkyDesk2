<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { relaunch } from "@tauri-apps/plugin-process";
import { listen, Event } from "@tauri-apps/api/event";
import { systemStore, windowStore, noteStore, wallpaperStore, shortcutStore, weatherStore, captureStore, } from "../stores/window";
import { disable, enable } from "@tauri-apps/plugin-autostart";
import { maininit } from "../functions/maininit";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { NetSpeed } from "../types/netspeedType";
const systemstore = systemStore();
const drawer = ref(true);
const colorshow = ref(false);
const settingshow = ref(false);
const net = ref<NetSpeed>({ speed_r: 0, speed_s: 0, });
listen("netspeed", (e: Event<NetSpeed>) => {
  net.value.speed_r = e.payload.speed_r;
  net.value.speed_s = e.payload.speed_s;
});
const toggleMaximizeBool = ref(false);
onMounted(async () => {
  await maininit();
  // 设置窗口拖拽
  document
    .getElementById("toolbar")
    ?.querySelector(".v-toolbar__content")
    ?.setAttribute("data-tauri-drag-region", "true");
  document
    .getElementById("toolbar")
    ?.querySelector(".v-toolbar-title__placeholder")
    ?.setAttribute("data-tauri-drag-region", "true");
  document
    .getElementById("toolbar")
    ?.addEventListener("selectstart", function (e) {
      e.preventDefault();
    });
  let dom = document.getElementById("app") as HTMLElement;
  if (systemstore.btnbackground) {
    dom.style.setProperty("--btn-background", systemstore.btnbackground);
  } else {
    dom.style.setProperty("--btn-background", "transparent");
  }
  if (systemstore.fontcolor) {
    dom.style.setProperty("--font-color", systemstore.fontcolor);
  } else {
    dom.style.setProperty("--font-color", "black");
  }
});

const toggleMaximize = async function () {
  await getCurrentWebviewWindow().toggleMaximize();
  toggleMaximizeBool.value = await getCurrentWebviewWindow().isMaximized();
};

const minus = async function () {
  await getCurrentWebviewWindow().minimize();
};

const closeApp = async function () {
  if (getCurrentWebviewWindow().label == "main") {
    getCurrentWebviewWindow().hide();
  } else {
    getCurrentWebviewWindow().destroy();
  }
};

const autostartsetting = function (e: any) {
  if (e) {
    enable();
  } else {
    disable();
  }
};

const refresh = function () {
  localStorage.clear();
  systemStore().$reset();
  windowStore().$reset();
  noteStore().$reset();
  wallpaperStore().$reset();
  shortcutStore().$reset();
  weatherStore().$reset();
  captureStore().$reset();
  relaunch();
};

const getImage = async function (i: number) {
  let res = await open({
    filters: [{ extensions: ["png", "jpg", "jpeg"], name: "Image" }],
    title: "选择背景图片",
  });
  if (res) {
    switch (i) {
      case 1:
        systemstore.programbcakground = `url('${convertFileSrc(res)}')`;
        break;
      case 2:
        systemstore.leftbackground = `url('${convertFileSrc(res)}')`;
        break;
      case 3:
        systemstore.topbackground = `url('${convertFileSrc(res)}')`;
        break;
      case 4:
        systemstore.shortcutbackground = `url('${convertFileSrc(res)}')`;
        break;
      case 5:
        systemstore.btnbackground = `url('${convertFileSrc(res)}')`;
        break;
      case 7:
        systemstore.btnbarbackground = `url('${convertFileSrc(res)}')`;
        break;
    }
    let dom = document.getElementById("app") as HTMLElement;
    if (systemstore.btnbackground) {
      dom.style.setProperty("--btn-background", systemstore.btnbackground);
    } else {
      dom.style.setProperty("--btn-background", "transparent");
    }
  }
};

const changebtn = function (i: number) {
  let dom = document.getElementById("app") as HTMLElement;
  if (i == 1) {
    if (systemstore.btnbackground) {
      dom.style.setProperty("--btn-background", systemstore.btnbackground);
    } else {
      dom.style.setProperty("--btn-background", "transparent");
    }
  } else if (i == 2) {
    if (systemstore.fontcolor) {
      dom.style.setProperty("--font-color", systemstore.fontcolor);
    } else {
      dom.style.setProperty("--font-color", "black");
    }
  }
};
const colori = ref();
const colorv = ref();
const palette = function (i: number) {
  colori.value = i;
  document.getElementById("palette")?.click();
};
const timer = ref();
const selectcolor = function () {
  clearTimeout(timer.value);
  timer.value = setTimeout(() => {
    switch (colori.value) {
      case 1:
        systemstore.programbcakground = colorv.value;
        break;
      case 2:
        systemstore.leftbackground = colorv.value;
        break;
      case 3:
        systemstore.topbackground = colorv.value;
        break;
      case 4:
        systemstore.shortcutbackground = colorv.value;
        break;
      case 5:
        systemstore.btnbackground = colorv.value;
        break;
      case 6:
        systemstore.fontcolor = colorv.value;
        break;
      case 7:
        systemstore.btnbarbackground = colorv.value;
    }
    // let dom = document.getElementsByTagName("body");
    let dom = document.getElementById("app") as HTMLElement;
    if (colori.value == 5) {
      console.log(systemstore.btnbackground);
      if (systemstore.btnbackground) {
        dom.style.setProperty("--btn-background", systemstore.btnbackground);
      } else {
        dom.style.setProperty("--btn-background", "transparent");
      }
    } else if (colori.value == 6) {
      if (systemstore.fontcolor) {
        dom.style.setProperty("--font-color", systemstore.fontcolor);
      } else {
        dom.style.setProperty("--font-color", "transparent");
      }
    }
  }, 50);
};

const wheel_status = function (e: any) {
  console.log(e);
  invoke("wheel_status", { bool: e });
};
</script>

<template>
  <v-layout :style="{
    background: systemstore.programbcakground
      ? systemstore.programbcakground
      : 'transparent',
    backgroundSize: 'cover',
    color: systemstore.fontcolor,
  }">
    <v-app-bar :style="{
      background: systemstore.topbackground
        ? systemstore.topbackground
        : 'transparent',
      backgroundSize: 'cover',
    }" :absolute="true" :height="48" id="toolbar" class="toolbar" data-tauri-drag-region>
      <template v-slot:title>
        <div :style="{ color: systemstore.fontcolor }" data-tauri-drag-region>
          SkyDesk2
        </div>
      </template>
      <template v-slot:prepend>
        <v-app-bar-nav-icon style="background: transparent" @click="drawer = !drawer"></v-app-bar-nav-icon>
      </template>
      <div :style="{
        width: '100px',
        display: 'flex',
        alignItems: 'center',
        color: systemstore.fontcolor,
      }" data-tauri-drag-region>
        <v-icon data-tauri-drag-region>mdi-arrow-down-thin</v-icon>{{
          Math.trunc(net.speed_r / 1024) < 1024 ? Math.trunc(net.speed_r / 1024) + "KB/s" : Math.trunc((net.speed_r / 1024
            / 1024) * 10) / 10 + "MB/s" }} </div>
          <div :style="{
            width: '100px',
            display: 'flex',
            alignItems: 'center',
            color: systemstore.fontcolor,
          }" data-tauri-drag-region>
            <v-icon data-tauri-drag-region>mdi-arrow-up-thin</v-icon>{{
              Math.trunc(net.speed_s / 1024) < 1024 ? Math.trunc(net.speed_s / 1024) + "KB/s" : Math.trunc((net.speed_s /
                1024 / 1024) * 10) / 10 + "MB/s" }} </div>
              <v-btn style="background: transparent" icon>
                <v-icon>mdi-help-circle-outline</v-icon>
              </v-btn>
              <v-btn style="background: transparent" @click="colorshow = true" icon>
                <v-icon>mdi-palette</v-icon>
              </v-btn>
              <v-btn style="background: transparent" icon @click="settingshow = true">
                <v-icon>mdi-cog-outline</v-icon>
              </v-btn>
              <v-btn style="background: transparent" icon @click="minus">
                <v-icon>mdi-window-minimize</v-icon>
              </v-btn>
              <v-btn style="background: transparent" icon @click="toggleMaximize">
                <v-icon v-show="toggleMaximizeBool">mdi-window-restore</v-icon>
                <v-icon v-show="!toggleMaximizeBool">mdi-window-maximize</v-icon>
              </v-btn>
              <v-btn style="background: transparent" icon @click="closeApp">
                <v-icon>mdi-window-close</v-icon>
              </v-btn>
    </v-app-bar>

    <v-main style="height: calc(100vh); overflow-y: auto; width: 100%">
      <v-navigation-drawer :style="{
        boxShadow: drawer ? '5px 0px 5px rgba(123,123,123,0.5)' : 'none',
        background: systemstore.leftbackground
          ? systemstore.leftbackground
          : 'transparent',
        backgroundSize: 'cover',
      }" width="150" temporary v-model="drawer" :permanent="true" expand-on-hover>
        <v-list style="height: 100%">
          <v-list-item :style="{ color: systemstore.fontcolor }" prepend-icon="mdi-apps" title="快捷"
            :href="'/#/pages/setting/shortcut'"></v-list-item>
          <v-list-item :style="{ color: systemstore.fontcolor }" prepend-icon="mdi-wallpaper" title="壁纸"
            :href="'/#/pages/setting/wallpaper'"></v-list-item>
          <v-list-item :style="{ color: systemstore.fontcolor }" prepend-icon="mdi-note-outline" title="便签"
            :href="'/#/pages/setting/note'"></v-list-item>
          <v-list-item :style="{ color: systemstore.fontcolor }" prepend-icon="mdi-robot-outline" title="AI"
            :href="'/#/pages/setting/ollama'"></v-list-item>
          <v-list-item :style="{ color: systemstore.fontcolor }" prepend-icon="mdi-wallpaper" title="录屏"
            :href="'/#/pages/setting/capture'"></v-list-item>
          <v-list-item :style="{ color: systemstore.fontcolor }" prepend-icon="mdi-calendar-range" title="日历"
            :href="'/#/pages/setting/datenote'"></v-list-item>
        </v-list>
      </v-navigation-drawer>
      <router-view v-slot="{ Component }" :key="$route.fullPath"
        style="width: auto; height: 100%; box-sizing: border-box; padding: 10px">
        <transition name="fade" mode="out-in" appear>
          <!-- <keep-alive> -->
          <component :is="Component" />
          <!-- </keep-alive> -->
        </transition>
      </router-view>
    </v-main>
    <v-dialog v-model="colorshow">
      <div style="
          width: 100%;
          height: 100%;
          display: flex;
          justify-content: center;
          position: relative;
        " @click.self="colorshow = false">
        <input id="palette" @input="selectcolor" v-model="colorv" style="position: absolute; left: 10; top: 10"
          type="color" />
        <v-card style="width: 400px">
          <v-card-title>
            <div style="display: flex; flex-direction: row">
              <div style="width: 50%; text-align: left">背景设置</div>
              <div style="width: 50%; text-align: right">
                <v-icon icon="mdi-window-close" @click="colorshow = false"></v-icon>
              </div>
            </div>
          </v-card-title>
          <v-list style="width: 400px">
            <v-list-item>
              <template v-slot:append>
                <v-text-field v-model="systemstore.programbcakground" width="280" hide-details="auto" density="compact">
                  <template v-slot:prepend-inner>
                    <v-btn variant="tonal" size="mini" @click="getImage(1)" style="margin-right: 5px">
                      <template v-slot:append>
                        <v-icon>mdi-image</v-icon>
                      </template>
                    </v-btn>
                    <v-btn variant="tonal" size="mini" @click="palette(1)">
                      <template v-slot:append>
                        <v-icon>mdi-palette</v-icon>
                      </template>
                    </v-btn>
                  </template>
                </v-text-field>
              </template>
              <v-list-item-title>主窗</v-list-item-title>
            </v-list-item>
            <v-divider></v-divider>
            <v-list-item>
              <template v-slot:append>
                <v-text-field v-model="systemstore.leftbackground" width="280" hide-details="auto" density="compact">
                  <template v-slot:prepend-inner>
                    <v-btn variant="tonal" size="mini" @click="getImage(2)" style="margin-right: 5px">
                      <template v-slot:append>
                        <v-icon>mdi-image</v-icon>
                      </template>
                    </v-btn>
                    <v-btn variant="tonal" size="mini" @click="palette(2)">
                      <template v-slot:append>
                        <v-icon>mdi-palette</v-icon>
                      </template>
                    </v-btn>
                  </template>
                </v-text-field>
              </template>
              <v-list-item-title>菜单</v-list-item-title>
            </v-list-item>
            <v-divider></v-divider>
            <v-list-item>
              <template v-slot:append>
                <v-text-field v-model="systemstore.topbackground" width="280" hide-details="auto" density="compact">
                  <template v-slot:prepend-inner>
                    <v-btn variant="tonal" size="mini" @click="getImage(3)" style="margin-right: 5px">
                      <template v-slot:append>
                        <v-icon>mdi-image</v-icon>
                      </template>
                    </v-btn>
                    <v-btn variant="tonal" size="mini" @click="palette(3)">
                      <template v-slot:append>
                        <v-icon>mdi-palette</v-icon>
                      </template>
                    </v-btn>
                  </template>
                </v-text-field>
              </template>
              <v-list-item-title>标题</v-list-item-title>
            </v-list-item>
            <v-divider></v-divider>
            <v-list-item>
              <template v-slot:append>
                <v-text-field v-model="systemstore.shortcutbackground" width="280" hide-details="auto"
                  density="compact">
                  <template v-slot:prepend-inner>
                    <v-btn variant="tonal" size="mini" @click="getImage(4)" style="margin-right: 5px">
                      <template v-slot:append>
                        <v-icon>mdi-image</v-icon>
                      </template>
                    </v-btn>
                    <v-btn variant="tonal" size="mini" @click="palette(4)">
                      <template v-slot:append>
                        <v-icon>mdi-palette</v-icon>
                      </template>
                    </v-btn>
                  </template>
                </v-text-field>
              </template>
              <v-list-item-title>快捷</v-list-item-title>
            </v-list-item>
            <v-divider></v-divider>
            <v-list-item>
              <template v-slot:append>
                <v-text-field v-model="systemstore.btnbackground" @update:model-value="changebtn(1)" width="280"
                  hide-details="auto" density="compact">
                  <template v-slot:prepend-inner>
                    <v-btn variant="tonal" size="mini" @click="getImage(5)" style="margin-right: 5px">
                      <template v-slot:append>
                        <v-icon>mdi-image</v-icon>
                      </template>
                    </v-btn>
                    <v-btn variant="tonal" size="mini" @click="palette(5)">
                      <template v-slot:append>
                        <v-icon>mdi-palette</v-icon>
                      </template>
                    </v-btn>
                  </template>
                </v-text-field>
              </template>
              <v-list-item-title>按钮</v-list-item-title>
            </v-list-item>
            <v-divider></v-divider>
            <v-list-item>
              <template v-slot:append>
                <v-text-field v-model="systemstore.btnbarbackground" @update:model-value="changebtn(2)" width="280"
                  hide-details="auto" density="compact">
                  <template v-slot:prepend-inner>
                    <v-btn variant="tonal" size="mini" @click="getImage(7)" style="margin-right: 5px">
                      <template v-slot:append>
                        <v-icon>mdi-image</v-icon>
                      </template>
                    </v-btn>
                    <v-btn variant="tonal" size="mini" @click="palette(7)">
                      <template v-slot:append>
                        <v-icon>mdi-palette</v-icon>
                      </template>
                    </v-btn>
                  </template>
                </v-text-field>
              </template>
              <v-list-item-title>按钮栏</v-list-item-title>
            </v-list-item>
            <v-divider></v-divider>
            <v-list-item>
              <template v-slot:append>
                <v-text-field v-model="systemstore.fontcolor" @update:model-value="changebtn(2)" width="280"
                  hide-details="auto" density="compact">
                  <template v-slot:prepend-inner>
                    <v-btn variant="tonal" size="mini" @click="palette(6)">
                      <template v-slot:append>
                        <v-icon>mdi-palette</v-icon>
                      </template>
                    </v-btn>
                  </template>
                </v-text-field>
              </template>
              <v-list-item-title>文本</v-list-item-title>
            </v-list-item>
            <v-divider></v-divider>
          </v-list>
        </v-card>
      </div>
    </v-dialog>
    <!-- 设置 -->
    <v-dialog v-model="settingshow">
      <div style="display: flex; justify-content: center" @click.self="settingshow = false">
        <v-card style="width: 400px">
          <v-card-title>
            <div style="display: flex; flex-direction: row">
              <div style="width: 50%; text-align: left">设置</div>
              <div style="width: 50%; text-align: right">
                <v-icon icon="mdi-window-close" @click="settingshow = false"></v-icon>
              </div>
            </div>
          </v-card-title>
          <v-card-item>
            <v-list>
              <v-list-item>
                <template v-slot:append>
                  <v-switch color="info" v-model="systemstore.traystart" hide-details></v-switch>
                </template>
                <v-list-item-title>启动到托盘</v-list-item-title>
              </v-list-item>
              <v-list-item>
                <template v-slot:append>
                  <v-switch color="info" v-model="systemstore.autostart" @update:model-value="autostartsetting"
                    hide-details></v-switch>
                </template>
                <v-list-item-title>开机自启</v-list-item-title>
              </v-list-item>
              <v-list-item>
                <template v-slot:append>
                  <v-switch color="info" v-model="systemstore.netspeed.show" hide-details></v-switch>
                </template>
                <v-list-item-title>网速控件</v-list-item-title>
              </v-list-item>
              <v-list-item>
                <template v-slot:append>
                  <v-switch color="info" v-model="systemstore.taskbar" hide-details></v-switch>
                </template>
                <v-list-item-title>任务栏</v-list-item-title>
              </v-list-item>
              <v-list-item>
                <template v-slot:append>
                  <v-switch color="info" v-model="systemstore.wheel" @update:model-value="wheel_status"
                    hide-details></v-switch>
                </template>
                <v-list-item-title>轮盘开关</v-list-item-title>
              </v-list-item>
              <v-list-item>
                <template v-slot:append>
                  <v-btn @click="refresh">
                    <template v-slot:prepend>
                      <v-icon>mdi-refresh</v-icon>
                    </template>
                    清除
                  </v-btn>
                </template>
                <v-list-item-title>清除用户信息</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-item>
        </v-card>
      </div>
    </v-dialog>
  </v-layout>
</template>

        <style>
        .v-btn {
          color: var(--font-color) !important;
          background: var(--btn-background);
          background-size: cover;
        }

        .v-text-field {
          color: var(--font-color) !important;
        }

        .v-list-item-title {
          color: var(--font-color) !important;
        }

        .v-card {
          color: var(--font-color) !important;
        }

        .home {
          width: 100vw;
          height: 100vh;
          overflow: hidden;
        }

        .toolbar {
          cursor: default;
        }

        .v-list-group__items {
          --parent-padding: calc(0px);
        }

        .fade-leave-active,
        .fade-enter-active {
          transition: all 0.3s;
        }

        .fade-enter-from {
          opacity: 0;
          transform: translateX(-10px);
        }

        .fade-enter-to {
          opacity: 1;
          transform: translateX(0px);
        }

        .fade-leave-to {
          opacity: 0;
          transform: translateX(30px);
        }
      </style>