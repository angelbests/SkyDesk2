<script setup lang="ts">
import { computed, onMounted, ref } from "vue"
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow"
import { relaunch } from "@tauri-apps/plugin-process"
import { windowStore } from "../stores/window"
import { systemStore } from "../stores/system"
import { noteStore } from "../stores/note"
import { wallpaperStore } from "../stores/wallpaper"
import { shortcutStore } from "../stores/shortcut"
import { weatherStore } from "../stores/weather"
import { captureStore } from "../stores/capture"
import { disable, enable } from "@tauri-apps/plugin-autostart"
import { maininit } from "../functions/maininit"
import { convertFileSrc, invoke } from "@tauri-apps/api/core"
import { open } from "@tauri-apps/plugin-dialog"
import { copyFile } from "@tauri-apps/plugin-fs"
import { Netspeed, NetSpeed } from "../functions/sysinfo"
import { uuid } from "../functions"
import { appDataDir, resolve } from "@tauri-apps/api/path"
import { createWindow } from "../functions/window"
const systemstore = systemStore()
const drawer = ref(true)
const colorshow = ref(false)
const settingshow = ref(false)
const toggleMaximizeBool = ref(false)
onMounted(async () => {
  await maininit()
  appbardraging()
  get_version()
})

window.addEventListener("drop", e => {
  e.preventDefault()
})

window.addEventListener("dragover", e => {
  e.preventDefault()
})

getCurrentWebviewWindow().listen("create-note", async () => {
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
})

//#region  //appbar控制
const appbardraging = function () {
  // 设置窗口拖拽
  document
    .getElementById("toolbar")
    ?.querySelector(".v-toolbar__content")
    ?.setAttribute("data-tauri-drag-region", "true")
  document
    .getElementById("toolbar")
    ?.querySelector(".v-toolbar-title__placeholder")
    ?.setAttribute("data-tauri-drag-region", "true")
  document.getElementById("toolbar")?.addEventListener("selectstart", function (e) {
    e.preventDefault()
  })
  let dom = document.getElementById("app") as HTMLElement
  if (systemstore.btnbackground) {
    dom.style.setProperty("--btn-background", systemstore.btnbackground)
  } else {
    dom.style.setProperty("--btn-background", "transparent")
  }
  if (systemstore.fontcolor) {
    dom.style.setProperty("--font-color", systemstore.fontcolor)
  } else {
    dom.style.setProperty("--font-color", "black")
  }
}

const toggleMaximize = async function () {
  await getCurrentWebviewWindow().toggleMaximize()
  toggleMaximizeBool.value = await getCurrentWebviewWindow().isMaximized()
}

const minus = async function () {
  await getCurrentWebviewWindow().minimize()
}

const closeApp = async function () {
  if (getCurrentWebviewWindow().label == "main") {
    getCurrentWebviewWindow().hide()
  } else {
    getCurrentWebviewWindow().destroy()
  }
}
//#endregion

//#region
// 开机自启
const autostartsetting = function (e: any) {
  if (e) {
    enable()
  } else {
    disable()
  }
}
//#endregion

//#region
// 清除用户信息
const refresh = function () {
  localStorage.clear()
  systemStore().$reset()
  windowStore().$reset()
  noteStore().$reset()
  wallpaperStore().$reset()
  shortcutStore().$reset()
  weatherStore().$reset()
  captureStore().$reset()
  relaunch()
}
//#endregion

//#region ui颜色设置
const getImage = async function (i: number) {
  let res = await open({
    filters: [{ extensions: ["png", "jpg", "jpeg"], name: "Image" }],
    title: "选择背景图片",
  })
  if (!res) return
  let path = await resolve(await appDataDir(), "window\\" + uuid() + ".png")
  await copyFile(res, path)
  if (res) {
    switch (i) {
      case 1:
        systemstore.programbcakground = `url('${convertFileSrc(path)}')`
        break
      case 2:
        systemstore.leftbackground = `url('${convertFileSrc(path)}')`
        break
      case 3:
        systemstore.topbackground = `url('${convertFileSrc(path)}')`
        break
      case 4:
        systemstore.shortcutbackground = `url('${convertFileSrc(path)}')`
        break
      case 5:
        systemstore.btnbackground = `url('${convertFileSrc(path)}')`
        break
      case 7:
        systemstore.btnbarbackground = `url('${convertFileSrc(path)}')`
        break
    }
    let dom = document.getElementById("app") as HTMLElement
    if (systemstore.btnbackground) {
      dom.style.setProperty("--btn-background", systemstore.btnbackground)
    } else {
      dom.style.setProperty("--btn-background", "transparent")
    }
  }
}

// 色盘控制
const changebtn = function (i: number) {
  let dom = document.getElementById("app") as HTMLElement
  if (i == 1) {
    if (systemstore.btnbackground) {
      dom.style.setProperty("--btn-background", systemstore.btnbackground)
    } else {
      dom.style.setProperty("--btn-background", "transparent")
    }
  } else if (i == 2) {
    if (systemstore.fontcolor) {
      dom.style.setProperty("--font-color", systemstore.fontcolor)
    } else {
      dom.style.setProperty("--font-color", "black")
    }
  }
}
const colori = ref()
const colorv = ref()
const palette = function (i: number) {
  colori.value = i
  document.getElementById("palette")?.click()
}
const timer = ref()
const selectcolor = function () {
  clearTimeout(timer.value)
  timer.value = setTimeout(() => {
    switch (colori.value) {
      case 1:
        systemstore.programbcakground = colorv.value
        break
      case 2:
        systemstore.leftbackground = colorv.value
        break
      case 3:
        systemstore.topbackground = colorv.value
        break
      case 4:
        systemstore.shortcutbackground = colorv.value
        break
      case 5:
        systemstore.btnbackground = colorv.value
        break
      case 6:
        systemstore.fontcolor = colorv.value
        break
      case 7:
        systemstore.btnbarbackground = colorv.value
    }
    // let dom = document.getElementsByTagName("body");
    let dom = document.getElementById("app") as HTMLElement
    if (colori.value == 5) {
      console.log(systemstore.btnbackground)
      if (systemstore.btnbackground) {
        dom.style.setProperty("--btn-background", systemstore.btnbackground)
      } else {
        dom.style.setProperty("--btn-background", "transparent")
      }
    } else if (colori.value == 6) {
      if (systemstore.fontcolor) {
        dom.style.setProperty("--font-color", systemstore.fontcolor)
      } else {
        dom.style.setProperty("--font-color", "transparent")
      }
    }
  }, 50)
}
//#endregion

//#region 轮盘开关
const wheel_status = function (e: any) {
  console.log(e)
  invoke("wheel_status", { bool: e })
}
//#endregion

//#region  软件更新
const helpshow = ref(false)
const version = ref("")
const updateversion = ref("")
import { check } from "@tauri-apps/plugin-updater"
import { openUrl } from "@tauri-apps/plugin-opener"
import { getVersion } from "@tauri-apps/api/app"

const open_github = function () {
  // https://github.com/angelbests/SkyDesk2
  openUrl("https://github.com/angelbests/SkyDesk2")
}

const get_version = async function () {
  version.value = await getVersion()
  checkupdate()
}
const checkupdate = async function () {
  const update = await check()
  if (update) {
    updateversion.value = update.version
  }
}

const updatestatus = ref(false)
const updatestr = ref("")
const updateprogram = async function () {
  if (updatestatus.value) return
  const update = await check()
  if (update) {
    updateversion.value = update.version
    console.log(`found update ${update.version} from ${update.date} with notes ${update.body}`)
    let downloaded = 0
    let contentLength = 0
    await update.downloadAndInstall((event) => {
      updatestatus.value = true
      switch (event.event) {
        case "Started":
          contentLength = event.data.contentLength || 0
          updatestr.value = "下载开始"
          break
        case "Progress":
          downloaded += event.data.chunkLength
          updatestr.value = Math.trunc((downloaded / contentLength) * 100).toString()
          break
        case "Finished":
          updatestr.value = "下载完成"
          console.log("下载完成")
          break
      }
    })
    updatestr.value = "update installed"
    console.log("update installed")
    await relaunch()
  }
}
//#endregion

//#region  网速监听

// import { LogicalPosition } from "@tauri-apps/api/dpi";
const net = ref<NetSpeed>({ speed_r: 0, speed_s: 0 })
new Netspeed().listen_netspeed((e) => {
  net.value = e.payload
})
const speed_r = computed(() => {
  return Math.trunc(net.value.speed_r / 1024) < 1024
    ? Math.trunc(net.value.speed_r / 1024) + "KB/s"
    : Math.trunc((net.value.speed_r / 1024 / 1024) * 10) / 10 + "MB/s"
})
const speed_s = computed(() => {
  return Math.trunc(net.value.speed_s / 1024) < 1024
    ? Math.trunc(net.value.speed_s / 1024) + "KB/s"
    : Math.trunc((net.value.speed_s / 1024 / 1024) * 10) / 10 + "MB/s"
})
//#endregion

const hovertop = async function () {
  await createWindow("hovertop", {
    x: 400,
    y: 0,
    width: 400,
    height: 500,
    decorations: false,
    transparent: true,
    dragDropEnabled: true,
    shadow: true,
    maximizable: false,
    resizable: true,
    skipTaskbar: true,
    alwaysOnTop: true,
    url: "/#/pages/desktop/hovertop",
    title: "hovertop",
  })
}
</script>

<template>
  <v-layout :style="{
    background: systemstore.programbcakground ? systemstore.programbcakground : 'transparent',
    backgroundSize: 'cover',
    color: systemstore.fontcolor,
  }">
    <v-app-bar :style="{
      background: systemstore.topbackground ? systemstore.topbackground : 'transparent',
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
      <div :style="{ width: '100px', display: 'flex', alignItems: 'center', color: systemstore.fontcolor }"
        data-tauri-drag-region>
        <v-icon data-tauri-drag-region>mdi-arrow-down-thin</v-icon>
        {{ speed_r }}
      </div>
      <div :style="{ width: '100px', display: 'flex', alignItems: 'center', color: systemstore.fontcolor }"
        data-tauri-drag-region>
        <v-icon data-tauri-drag-region>mdi-arrow-up-thin</v-icon>
        {{ speed_s }}
      </div>
      <v-btn style="background: transparent" @click="helpshow = true" icon>
        <v-icon>mdi-help-circle-outline</v-icon>
      </v-btn>
      <v-btn style="background: transparent" @click="hovertop" icon>
        <v-icon>mdi-dock-top</v-icon>
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
        <v-icon v-if="toggleMaximizeBool">mdi-window-restore</v-icon>
        <v-icon v-else="!toggleMaximizeBool">mdi-window-maximize</v-icon>
      </v-btn>
      <v-btn style="background: transparent" icon @click="closeApp">
        <v-icon>mdi-window-close</v-icon>
      </v-btn>
    </v-app-bar>

    <v-main style="height: calc(100vh); overflow-y: auto; width: 100%">
      <v-navigation-drawer :style="{
        boxShadow: drawer ? '5px 0px 5px rgba(123,123,123,0.5)' : 'none',
        background: systemstore.leftbackground ? systemstore.leftbackground : 'transparent',
        backgroundSize: 'cover',
      }" width="150" temporary v-model="drawer" :permanent="true" expand-on-hover>
        <v-list style="height: 100%">
          <v-list-item draggable="false" :style="{ color: systemstore.fontcolor }" prepend-icon="mdi-apps" title="快捷"
            :href="'/#/pages/setting/shortcut'"></v-list-item>
          <v-list-item draggable="false" :style="{ color: systemstore.fontcolor }" prepend-icon="mdi-wallpaper"
            title="壁纸" :href="'/#/pages/setting/wallpaper'"></v-list-item>
          <v-list-item draggable="false" :style="{ color: systemstore.fontcolor }" prepend-icon="mdi-note-outline"
            title="便签" :href="'/#/pages/setting/note'"></v-list-item>
          <v-list-item draggable="false" :style="{ color: systemstore.fontcolor }" prepend-icon="mdi-robot-outline"
            title="AI" :href="'/#/pages/setting/ollama'"></v-list-item>
          <v-list-item draggable="false" :style="{ color: systemstore.fontcolor }" prepend-icon="mdi-image-plus-outline"
            title="录屏" :href="'/#/pages/setting/capture'"></v-list-item>
          <v-list-item draggable="false" :style="{ color: systemstore.fontcolor }" prepend-icon="mdi-calendar-range"
            title="日历" :href="'/#/pages/setting/datenote'"></v-list-item>
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
      <div style="width: 100%; height: 100%; display: flex; justify-content: center; position: relative"
        @click.self="colorshow = false">
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
    <v-dialog v-model="helpshow">
      <div style="display: flex; justify-content: center" @click.self="helpshow = false">
        <v-card style="width: 400px">
          <v-card-title>
            <div style="display: flex; flex-direction: row">
              <div style="width: 50%; text-align: left">关于</div>
              <div style="width: 50%; text-align: right">
                <v-icon icon="mdi-window-close" @click="helpshow = false"></v-icon>
              </div>
            </div>
          </v-card-title>
          <v-card-item>
            <div style="width: 100%; height: 150px">
              <v-list>
                <v-list-item>
                  <template v-slot:prepend>
                    <v-btn style="width: 110px" @click="open_github">
                      <template v-slot:prepend>
                        <v-icon>mdi-github</v-icon>
                      </template>
                      Github
                    </v-btn>
                  </template>
                  <template v-slot:append>
                    <div style="text-align: center">SkyDesk2 {{ version }}</div>
                  </template>
                </v-list-item>
                <v-list-item>
                  <template v-slot:prepend>
                    <v-btn v-show="updateversion" style="width: 110px" @click="updateprogram">
                      <template v-slot:prepend>
                        <v-icon>mdi-refresh</v-icon>
                      </template>
                      更新程序
                    </v-btn>
                    <v-btn v-show="!updateversion" style="width: 110px" @click="checkupdate">
                      <template v-slot:prepend>
                        <v-icon>mdi-refresh</v-icon>
                      </template>
                      检查更新
                    </v-btn>
                  </template>
                  <template v-slot:append>
                    <div v-if="!updatestatus" style="text-align: center">
                      {{ updateversion ? version + " -> " + updateversion : version }}
                    </div>
                    <div v-else style="text-align: center">
                      {{ updatestr }}%
                    </div>
                  </template>
                </v-list-item>
              </v-list>
            </div>
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
