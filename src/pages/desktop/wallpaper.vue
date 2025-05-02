<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { convertFileSrc } from "@tauri-apps/api/core";
import { wallpaperStore } from "../../stores/wallpaper";
import { listen } from "@tauri-apps/api/event";
import "qweather-icons/font/qweather-icons.css";
import { currentMonitor } from "@tauri-apps/api/window";
import MusicDisk from "../../components/MusicDisk.vue";
import MusicVinyl from "../../components/MusicVinyl.vue";
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
});

//////////////////////////日期/////////////////////////////////////
import { get_time, TimeWallpaper } from "../../functions/date";
const time = ref<TimeWallpaper>({ year: "0000", month: "00", day: "00", "week": "星期一", hour: "00", minute: "00", second: "00" });
setInterval(() => {
  time.value = get_time()
}, 1000);
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

///////////////////weather/////////////////////////
import { weather_wallpaper, WeatherType } from "../../functions/weather";
const w = ref<WeatherType>({ temp: "", feelsLike: "", icon: "", text: "", windDir: "", windScale: "", windSpeed: "", humidity: "", precip: "", pressure: "", vis: "", cloud: "", dew: "", city: "", });
weather_wallpaper((e) => {
  w.value = e
}).then((e) => {
  w.value = e
})

//////////////////////////网速////////////////////////////////
import { Netspeed, NetSpeed } from "../../functions/sysinfo";
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

</script>

<template>
  <div class="window">
    <!-- wallpaper -->
    <img v-if="type == 'image'" :src="convertFileSrc(path)" class="image" />
    <video v-else-if="type == 'video'" id="wallpapervideo" class="video" :src="convertFileSrc(path)" autoplay="true"
      loop="true"></video>
    <!-- 天气 -->
    <div class="weather" v-show="wallpaperstore.wallpaperConfig[index].config.weather" :style="{
      left: `${wallpaperstore.wallpaperConfig[index].config.weatherx}%`,
      top: `${wallpaperstore.wallpaperConfig[index].config.weathery}%`,
    }">
      <div style="width: 200px; height: 30px; text-align: center">
        {{ w.city }}
      </div>
      <div style="width: 200px; display: flex; height: 60px">
        <div style="
            width: 100px;
            height: 50px;
            line-height: 50px;
            text-align: center;
          ">
          {{ w.text }} {{ w.temp }}°
        </div>
        <div style="
            width: 100px;
            height: 50px;
            text-align: center;
            line-height: 50px;
          ">
          <i style="font-size: 30px" :class="['qi-' + (w.icon ? w.icon : 100), 'weather-icon']"></i>
        </div>
      </div>
      <div style="width: 200px; height: 60px; display: flex">
        <div style="width: 100px; height: 50px; text-align: center">
          {{ w.windDir }}
        </div>
        <div style="width: 100px; height: 50px; text-align: center">
          {{ w.windScale }}级
        </div>
      </div>
    </div>
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
    <!-- time -->
    <div class="time" v-show="wallpaperstore.wallpaperConfig[index].config.time" :style="{
      left: `${wallpaperstore.wallpaperConfig[index].config.timex}%`,
      top: `${wallpaperstore.wallpaperConfig[index].config.timey}%`,
      fontSize: `${wallpaperstore.wallpaperConfig[index].config.timefontsize}px`,
    }">
      <div class="hour">
        {{ time.hour }}
        <div style="height: 1.65em;width: 23px;text-align: center;">
          :
        </div>
        {{ time.minute }}
        <div style="height: 1.65em;width: 23px;text-align: center;">
          :
        </div>
        {{ time.second }}
      </div>
    </div>
    <div class="date" v-show="wallpaperstore.wallpaperConfig[index].config.date" :style="{
      left: `${wallpaperstore.wallpaperConfig[index].config.datex}%`,
      top: `${wallpaperstore.wallpaperConfig[index].config.datey}%`,
      fontSize: `${wallpaperstore.wallpaperConfig[index].config.datefontsize}px`,
    }">
      <div style="margin-right: 10px">
        {{ time.year }}
        {{ time.month }}
        {{ time.day }}
      </div>
      <div>
        {{ time.week }}
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
  </div>
</template>

<style scoped>
.window {
  width: 100vw;
  height: 100vh;
  position: relative;
  font-family: 'Microsoft YaHei UI LIGHT';
}

.image {
  width: 100vw;
  height: 100vh;
  object-fit: cover;
}

.video {
  width: 100vw;
  height: 100vh;
  object-fit: cover;
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

.time {
  position: absolute;
  z-index: 201;
  color: rgba(223, 223, 223, 0.5);
}

.hour {
  display: flex;
  justify-content: space-evenly;
  align-items: center;
}

.date {
  position: absolute;
  z-index: 202;
  display: flex;
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

.weather {
  display: flex;
  flex-direction: column;
  position: absolute;
  right: 40px;
  top: 40px;
  z-index: 100;
  width: 200px;
  height: 300px;
}
</style>
