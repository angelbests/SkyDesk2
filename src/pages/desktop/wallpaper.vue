<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { convertFileSrc } from "@tauri-apps/api/core";
import { wallpaperStore } from "../../stores/wallpaper";
import { listen, Event } from "@tauri-apps/api/event";
import { currentMonitor, monitorFromPoint } from "@tauri-apps/api/window";
import MusicDisk from "../../components/wallpaper/MusicDisk.vue";
import MusicVinyl from "../../components/wallpaper/MusicVinyl.vue";
import MusicTape from "../../components/wallpaper/MusicTape.vue";
import Weather from "../../components/wallpaper/Weather.vue";
import Date from "../../components/wallpaper/Date.vue";

import { Netspeed, NetSpeed } from "../../functions/sysinfo";
import { MouseAction, MouseEvent } from "../../types/desktopType"
// import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
// import { invoke } from "@tauri-apps/api/core";
const wallpaperstore = wallpaperStore();
const index = ref(0);
const route = useRoute();
const path = ref();
const type = ref();
onMounted(async () => {
  document.title = "skydesk2-wallpaper"
  path.value = route.query.path;
  type.value = route.query.type;
  const monitor = await currentMonitor();
  index.value = wallpaperstore.wallpaperConfig.findIndex(
    (item) => item.monitor == monitor?.name
  );
  if (type.value == 'video') {
    let dom = document.getElementById("wallpapervideo") as HTMLVideoElement;
    dom.volume = wallpaperstore.wallpaperConfig[index.value].config.audio / 100;
  }
  window.addEventListener("storage", (e) => {
    if (e.key == "wallpaper") {
      wallpaperstore.$hydrate();
      if (type.value == 'video') {
        let dom = document.getElementById("wallpapervideo") as HTMLVideoElement;
        dom.volume = wallpaperstore.wallpaperConfig[index.value].config.audio / 100;
      }
    }
  });
  animate()
  // invoke("open_devtool", { label: await getCurrentWebviewWindow().label })

});

////////////////////cpu/////////////////////
const cpu = ref(0);
listen("cpu", (e) => {
  let str = e.payload;
  cpu.value = Math.trunc(Math.round(Number(str)));
});

///////////////////////////memeory///////////////////////
const memory = ref(0);
listen("memory", (e) => {
  let str = e.payload;
  memory.value = Math.trunc(Number(str) * 100);
});

//////////////////////////网速////////////////////////////////


const net = ref<NetSpeed>({ speed_r: 0, speed_s: 0 })
const netspeed = new Netspeed();
netspeed.listen_netspeed((e) => {
  net.value = e.payload
})

const speed_r = computed(() => {
  return Math.trunc(net.value.speed_r / 1024) < 1024 ? Math.trunc(net.value.speed_r / 1024) + "KB/s" : Math.trunc((net.value.speed_r / 1024 / 1024) * 10) / 10 + "MB/s"
})

const speed_s = computed(() => {
  return Math.trunc(net.value.speed_s / 1024) < 1024 ? Math.trunc(net.value.speed_s / 1024) + "KB/s" : Math.trunc((net.value.speed_s / 1024 / 1024) * 10) / 10 + "MB/s"
})

// 鼠标跟随 //////////////////////////////////////////
let rx: number = 0;
let ry: number = 0
let tx: number = 0;
let ty: number = 0
listen("desktop", async (e: Event<MouseEvent>) => {
  let dom: any
  if (type.value == 'image') {
    dom = document.getElementById("wallpaperimg")

  } else if (type.value == 'video') {
    dom = document.getElementById("wallpapervideo")
  }
  if (!dom) return
  if (e.payload.mouse == MouseAction.Move) {
    let { x, y } = e.payload
    console.log(x, y)
    let monitor = (await monitorFromPoint(x, y))
    let current = await currentMonitor()
    if (!monitor) return
    if (!current) return
    if (current.name != monitor.name) return
    x = (x - monitor.position.x) / monitor.scaleFactor
    y = (y - monitor.position.y) / monitor.scaleFactor
    let poix = x / window.innerWidth - 0.5; // -0.5 ~ 0.5
    let poiy = y / window.innerHeight - 0.5;
    rx = (poiy * 5); // 可调节旋转幅度
    ry = (-poix * 5);
    // dom.style.transform = ``;
    tx = (x / window.innerWidth - 0.5) * 100;  // [-15px, 15px]
    ty = (y / window.innerHeight - 0.5) * 100;
  }
})

function animate() {
  let dom: any
  if (type.value == 'image') {
    dom = document.getElementById("wallpaperimg")

  } else if (type.value == 'video') {
    dom = document.getElementById("wallpapervideo")
  }
  if (!dom) return
  dom.style.transform = `translate(${tx}px, ${ty}px) rotateX(${rx}deg) rotateY(${ry}deg) scale(1)`;
  requestAnimationFrame(animate);
}


// let timer: any = undefined
// listen("mouse-move", async (e: Event<{ message: string }>) => {
//   if (timer) {
//     clearTimeout(timer)
//   } else {
//     timer = undefined
//   }
//   timer = setTimeout(async () => {
//     let dom: any
//     if (type.value == 'image') {
//       dom = document.getElementById("wallpaperimg")

//     } else if (type.value == 'video') {
//       dom = document.getElementById("wallpapervideo")
//     }
//     if (!dom) return
//     // if (e.payload.mouse == MouseAction.Move) {
//     let { x, y } = JSON.parse(e.payload.message) as { x: number, y: number }
//     console.log(x, y)
//     let monitor = (await monitorFromPoint(x, y))
//     let current = await currentMonitor()
//     if (!monitor) return
//     if (!current) return
//     if (current.name != monitor.name) return
//     x = (x - monitor.position.x) / monitor.scaleFactor
//     y = (y - monitor.position.y) / monitor.scaleFactor
//     let poix = x / window.innerWidth - 0.5; // -0.5 ~ 0.5
//     let poiy = y / window.innerHeight - 0.5;
//     let rotateX = (poiy * 5).toFixed(4); // 可调节旋转幅度
//     let rotateY = (-poix * 5).toFixed(4);
//     // dom.style.transform = ``;
//     poix = (x / window.innerWidth - 0.5) * 100;  // [-15px, 15px]
//     poiy = (y / window.innerHeight - 0.5) * 100;
//     dom.style.transform = `translate(${poix}px, ${poiy}px) rotateX(${rotateX}deg) rotateY(${rotateY}deg)`;
//     // }
//   }, 0)
// })
</script>

<template>
  <div class="window">
    <!-- wallpaper -->
    <img id="wallpaperimg" v-if="type == 'image'" :src="convertFileSrc(path)" class="image" />
    <video id="wallpapervideo" v-else-if="type == 'video'" class="video" :src="convertFileSrc(path)" autoplay="true"
      loop="true"></video>

    <!-- 网速 -->
    <div class="netspeed" v-show="wallpaperstore.wallpaperConfig[index].config.netspeed" :style="{
      left: `${wallpaperstore.wallpaperConfig[index].config.netspeedx}%`,
      top: `${wallpaperstore.wallpaperConfig[index].config.netspeedy}%`,
      fontSize: `${wallpaperstore.wallpaperConfig[index].config.netspeedfontsize}px`,
    }">
      <div data-tauri-drag-region style="display: flex; align-items: center">
        <v-icon data-tauri-drag-region>mdi-arrow-down-thin</v-icon>{{ speed_r }}
      </div>
      <div style="display: flex; align-items: center">
        <v-icon data-tauri-drag-region>mdi-arrow-up-thin</v-icon>{{ speed_s }}
      </div>
    </div>

    <!-- cpu -->
    <div class="cpu" v-show="wallpaperstore.wallpaperConfig[index].config.cpu" :style="{
      left: `${wallpaperstore.wallpaperConfig[index].config.cpux}%`,
      top: `${wallpaperstore.wallpaperConfig[index].config.cpuy}%`,
      fontSize: `${wallpaperstore.wallpaperConfig[index].config.cpufontsize}px`,
    }">
      CPU：{{ cpu }}%
    </div>
    <!-- memory -->
    <div class="memory" v-show="wallpaperstore.wallpaperConfig[index].config.memory" :style="{
      left: `${wallpaperstore.wallpaperConfig[index].config.memoryx}%`,
      top: `${wallpaperstore.wallpaperConfig[index].config.memoryy}%`,
      fontSize: `${wallpaperstore.wallpaperConfig[index].config.memoryfontsize}px`,
    }">
      内存：{{ memory }}%
    </div>
    <!-- music1 -->
    <MusicVinyl v-if="wallpaperstore.wallpaperConfig[index].config.musictype == 1" :style="{
      left: `${wallpaperstore.wallpaperConfig[index].config.musicx}%`,
      top: `${wallpaperstore.wallpaperConfig[index].config.musicy}%`,
    }"></MusicVinyl>
    <!-- music2-->
    <MusicDisk v-if="wallpaperstore.wallpaperConfig[index].config.musictype == 2" :style="{
      left: `${wallpaperstore.wallpaperConfig[index].config.musicx}%`,
      top: `${wallpaperstore.wallpaperConfig[index].config.musicy}%`,
    }"></MusicDisk>
    <!-- music3 -->
    <MusicTape v-if="wallpaperstore.wallpaperConfig[index].config.musictype == 3" :style="{
      left: `${wallpaperstore.wallpaperConfig[index].config.musicx}%`,
      top: `${wallpaperstore.wallpaperConfig[index].config.musicy}%`,
    }">
    </MusicTape>
    <!-- weather -->
    <Weather v-if="wallpaperstore.wallpaperConfig[index].config.weather" :style="{
      left: `${wallpaperstore.wallpaperConfig[index].config.weatherx}%`,
      top: `${wallpaperstore.wallpaperConfig[index].config.weathery}%`,
    }">
    </Weather>
    <!-- date -->
    <Date v-show="wallpaperstore.wallpaperConfig[index].config.date" :style="{
      left: `${wallpaperstore.wallpaperConfig[index].config.datex}%`,
      top: `${wallpaperstore.wallpaperConfig[index].config.datey}%`,
      color: `${wallpaperstore.wallpaperConfig[index].config.color}`
    }"></Date>
  </div>
</template>

<style scoped>
@font-face {
  font-family: "Quicksand";
  src: url("/font/Quicksand-Light.ttf") format(woff);
  font-weight: normal;
  font-style: normal;
}

.window {
  width: 100vw;
  height: 100vh;
  position: absolute;
  overflow: hidden;
  perspective: 1000px;
  font-family: 'Quicksand';
}

.image {
  width: 120%;
  height: 120%;
  background-size: cover;
  background-position: center;
  position: absolute;
  top: -10%;
  left: -10%;
  transform-origin: center;
  will-change: transform;
  transition: transform 0.3s ease-out;
}

.video {
  width: 120%;
  height: 120%;
  object-fit: cover;
  background-size: cover;
  background-position: center;
  position: absolute;
  top: -10%;
  left: -10%;
  transform-origin: center;
  will-change: transform;
  transition: transform 0.3s ease-out;
}

.netspeed {
  position: absolute;
  z-index: 200;
  width: 80px;
  height: 45px;
  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: flex-start;
  color: rgba(223, 223, 223, 0.5);
}


.cpu {
  position: absolute;
  z-index: 203;
  color: rgba(223, 223, 223, 0.5);
}

.memory {
  position: absolute;
  z-index: 204;
  color: rgba(223, 223, 223, 0.5);
}
</style>
