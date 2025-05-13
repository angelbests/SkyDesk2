<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { convertFileSrc } from "@tauri-apps/api/core";
import { wallpaperStore } from "../../stores/wallpaper";
import { listen, Event } from "@tauri-apps/api/event";
import { currentMonitor, Monitor } from "@tauri-apps/api/window";
import MusicDisk from "../../components/wallpaper/MusicDisk.vue";
import MusicVinyl from "../../components/wallpaper/MusicVinyl.vue";
import MusicTape from "../../components/wallpaper/MusicTape.vue";
import Weather from "../../components/wallpaper/Weather.vue";
import WeatherD7 from "../../components/wallpaper/WeatherD7.vue";
import Date from "../../components/wallpaper/Date.vue";
import { MouseAction, MouseEvent } from "../../types/desktopType"
import { startSakura, stopp } from "../../functions/sakura";
// import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
// import { invoke } from "@tauri-apps/api/core";
// invoke("open_devtool", { label: getCurrentWebviewWindow().label })
const wallpaperstore = wallpaperStore();
const index = ref<number>(0);
const route = useRoute();
const path = ref();
const type = ref();
const dom = ref<any>()
const monitor = ref<Monitor>()
const unlisten = ref<any>()
onMounted(() => {
  document.title = "skydesk2-wallpaper"
  path.value = route.query.path;
  type.value = route.query.type;
  setTimeout(async () => {
    await init()
    if (wallpaperstore.wallpaperConfig[index.value].config.sakura) {
      startSakura()
    }

    if (wallpaperstore.wallpaperConfig[index.value].config.action) {
      listen_desktop()
    }
  }, 50)
});

const init = async function () {
  monitor.value = await currentMonitor() as Monitor
  index.value = wallpaperstore.wallpaperConfig.findIndex(
    (item) => item.monitor == monitor.value?.name
  );
  if (type.value == 'image') {
    dom.value = document.getElementById("wallpaperimg")
  } else if (type.value == 'video') {
    dom.value = document.getElementById("wallpapervideo")
    dom.value.volume = wallpaperstore.wallpaperConfig[index.value].config.audio / 100;
  }
  window.addEventListener("storage", (e) => {
    if (e.key == "wallpaper") {
      wallpaperstore.$hydrate();
      if (type.value == 'video') {
        dom.value.volume = wallpaperstore.wallpaperConfig[index.value].config.audio / 100;
      }
      if (wallpaperstore.wallpaperConfig[index.value].config.sakura) {
        startSakura()
      } else {
        stopp()
      }

      if (wallpaperstore.wallpaperConfig[index.value].config.action) {
        listen_desktop()
      } else {
        unlisten.value()
      }
    }
  });


}

// 鼠标跟随 //////////////////////////////////////////
const listen_desktop = function () {
  let rx: number = 0;
  let ry: number = 0;
  let tx: number = 0;
  let ty: number = 0;
  unlisten.value = listen("desktop", async (e: Event<MouseEvent>) => {
    if (monitor.value?.name !== e.payload.monitor.name) return
    if (e.payload.mouse == MouseAction.Move) {
      let { x, y } = e.payload
      x = (x - monitor.value.position.x) / monitor.value.scaleFactor
      y = (y - monitor.value.position.y) / monitor.value.scaleFactor
      let poix = x / window.innerWidth - 0.5; // -0.5 ~ 0.5
      let poiy = y / window.innerHeight - 0.5;
      rx = (poiy * 5);
      ry = (-poix * 5);
      tx = (x / window.innerWidth - 0.5) * 100;  // [-15px, 15px]
      ty = (y / window.innerHeight - 0.5) * 100;
    }
  })
  function animate() {
    if (wallpaperstore.wallpaperConfig[index.value].config.action) {
      dom.value.style.transform = `translate3d(${tx}px, ${ty}px,0) rotateX(${rx}deg) rotateY(${ry}deg)`;
    } else {
      dom.value.style.transform = `translate3d(0px, 0px,0) rotateX(0deg) rotateY(0deg)`;
    }
    requestAnimationFrame(animate);
  }
  animate()
}
// import { get_all_festival } from '../../functions/calendar'

// const today = ref<{
//   year: number
//   month: number
//   day: number
// }>({
//   year: (new window.Date()).getFullYear(),
//   month: new window.Date().getMonth() + 1,
//   day: new window.Date().getDate()
// })
// const festivals = get_all_festival(today.value.year, today.value.month, today.value.day)

</script>

<template>
  <div class="window">
    <!-- snow -->
    <div style="width: 100%;height: 100%;position: absolute;z-index: 500;">

    </div>
    <!-- wallpaper -->
    <img id="wallpaperimg" v-show="type == 'image'" :src="convertFileSrc(path)" :class="{
      image: true, action: wallpaperstore.wallpaperConfig[index].config.action, unaction: !wallpaperstore.wallpaperConfig[index].config.action
    }" />
    <video id="wallpapervideo" v-show="type == 'video'"
      :class="{ video: true, action: wallpaperstore.wallpaperConfig[index].config.action, unaction: !wallpaperstore.wallpaperConfig[index].config.action }"
      :src="convertFileSrc(path)" autoplay="true" loop="true"></video>
    <!-- music1 -->
    <MusicVinyl
      v-if="wallpaperstore.wallpaperConfig[index].config.musictype == 1 && wallpaperstore.wallpaperConfig[index].config.music"
      :style="{
        left: `${wallpaperstore.wallpaperConfig[index].config.musicx}%`,
        top: `${wallpaperstore.wallpaperConfig[index].config.musicy}%`,
        filter: wallpaperstore.wallpaperConfig[index].config.musicshadow ? 'drop-shadow(0px 0px 10px black)' : ''
      }"></MusicVinyl>
    <!-- music2-->
    <MusicDisk
      v-if="wallpaperstore.wallpaperConfig[index].config.musictype == 2 && wallpaperstore.wallpaperConfig[index].config.music"
      :style="{
        left: `${wallpaperstore.wallpaperConfig[index].config.musicx}%`,
        top: `${wallpaperstore.wallpaperConfig[index].config.musicy}%`,
        filter: wallpaperstore.wallpaperConfig[index].config.musicshadow ? 'drop-shadow(0px 0px 10px black)' : ''
      }"></MusicDisk>
    <!-- music3 -->
    <MusicTape
      v-if="wallpaperstore.wallpaperConfig[index].config.musictype == 3 && wallpaperstore.wallpaperConfig[index].config.music"
      :style="{
        left: `${wallpaperstore.wallpaperConfig[index].config.musicx}%`,
        top: `${wallpaperstore.wallpaperConfig[index].config.musicy}%`,
      }">
    </MusicTape>
    <!-- weather -->
    <Weather
      v-if="(wallpaperstore.wallpaperConfig[index].config.weather && wallpaperstore.wallpaperConfig[index].config.weatherd7 == 1)"
      :style="{
        left: `${wallpaperstore.wallpaperConfig[index].config.weatherx}%`,
        top: `${wallpaperstore.wallpaperConfig[index].config.weathery}%`,
        boxShadow: wallpaperstore.wallpaperConfig[index].config.weathershadow ? '0px 0px 10px black' : 'none'
      }">
    </Weather>
    <WeatherD7
      v-if="(wallpaperstore.wallpaperConfig[index].config.weather && wallpaperstore.wallpaperConfig[index].config.weatherd7 == 7)"
      :style="{
        left: `${wallpaperstore.wallpaperConfig[index].config.weatherx}%`,
        top: `${wallpaperstore.wallpaperConfig[index].config.weathery}%`,
        boxShadow: wallpaperstore.wallpaperConfig[index].config.weathershadow ? '0px 0px 10px black' : ''
      }">
    </WeatherD7> <!-- date -->
    <Date v-if="wallpaperstore.wallpaperConfig[index].config.date" :style="{
      left: `${wallpaperstore.wallpaperConfig[index].config.datex}%`,
      top: `${wallpaperstore.wallpaperConfig[index].config.datey}%`,
      color: `${wallpaperstore.wallpaperConfig[index].config.datecolor}`,
      textShadow: wallpaperstore.wallpaperConfig[index].config.dateshadow ? '0px 0px 10px black' : ''
    }"></Date>
    <!-- <div :style="{
      left: '20px',
      top: '400px',
      color: `${wallpaperstore.wallpaperConfig[index].config.color}`
    }" class="countdown">
      {{ festivals[0].festival }} 还有
      <span style="font-size: 40px;">
        {{ festivals[0].day }}
      </span> 天
    </div> -->
  </div>
</template>

<style scoped>
.countdown {
  position: absolute;
  z-index: 600;
  width: 300px;
  height: 300px;
  background: rgba(253, 253, 253, 0);
  box-shadow: 0px 0px 10px black;
  border-radius: 50px;
  /* background: white; */
}


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
  transform-style: preserve-3d;
}

.image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  will-change: transform;
  transition: all 0.1s ease-out;
}

.video {
  width: 100%;
  height: 100%;
  object-fit: cover;
  will-change: transform;
  transition: all 0.1s ease-out;
}

.unaction {
  width: 100%;
  height: 100%;
  position: absolute;
  top: 0%;
  left: 0%;
  transform: translate3d(0, 0, 0) rotateX(0deg) rotateY(0deg) scale(1);
}

.action {
  width: 120%;
  height: 120%;
  background-size: cover;
  background-position: center;
  position: absolute;
  top: -10%;
  left: -10%;
  transform-origin: center;
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
