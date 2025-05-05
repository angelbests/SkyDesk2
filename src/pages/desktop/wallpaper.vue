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
import MusicTape from "../../components/MusicTape.vue";
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
  // invoke("open_devtool", { label: await getCurrentWebviewWindow().label })

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
import { weatherNowWallpaper, weatherD7Wallpaper, WeatherType, WeatherD7Type } from "../../functions/weather";
import { weatherIcon, windIcon } from "../../functions/weatnerIcon";
const w = ref<WeatherType>({ temp: "", feelsLike: "", icon: "", text: "", windDir: "", windScale: "", windSpeed: "", humidity: "", precip: "", pressure: "", vis: "", cloud: "", dew: "", city: "", });
weatherNowWallpaper((e) => {
  w.value = e
  icon1.value = getWeatherIcon(Number(e.icon))
  wind.value = getWindIcon(Number(e.windScale))
  console.log(e)
}).then((e) => {
  w.value = e
  icon1.value = getWeatherIcon(Number(e.icon))
  wind.value = getWindIcon(Number(e.windScale))
  console.log(e)
})
const ws = ref<{
  city: string,
  daily: WeatherD7Type[]
}>({
  city: "",
  daily: []
})
weatherD7Wallpaper((e) => {
  ws.value = e
  icon2.value = getWeatherIcon(Number(e.daily[0].iconDay))
  console.log(e)
}).then((e) => {
  ws.value = e
  icon2.value = getWeatherIcon(Number(e.daily[0].iconDay))
  console.log(e)
})
const icon1 = ref("");
const icon2 = ref("");
const getWeatherIcon = function (id: number) {
  let res = weatherIcon.filter(e => {
    return e.code == id
  })
  return `/svg/${res[0].icon}.svg`
}
const wind = ref("")
const getWindIcon = function (id: number) {
  let res = windIcon.filter(e => {
    return e.code == id
  })
  return `/svg/${res[0].icon}.svg`
}
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

// import { download } from "@tauri-apps/plugin-upload"
// import { pictureDir } from "@tauri-apps/api/path";
// let str = "https://a.hecdn.net/img/common/icon/202106d/";

// let i = 1;

// let timer = setInterval(async () => {
//   i = i + 1
//   console.log(str + i + ".png")
//   let path = await pictureDir() + "\\icon\\" + i + ".png"
//   download(str + i + ".png", path)
//   if (i >= 3000) {
//     clearInterval(timer)
//   }
// }, 1000)


</script>

<template>
  <div class="window">
    <!-- date -->
    <div v-show="wallpaperstore.wallpaperConfig[index].config.date" :style="{
      left: `${wallpaperstore.wallpaperConfig[index].config.datex}%`,
      top: `${wallpaperstore.wallpaperConfig[index].config.datey}%`,
      color: `${wallpaperstore.wallpaperConfig[index].config.color}`
    }" class="date">
      <div class="date_time">
        {{ time.hour }} <span class="date_dot">:</span> {{ time.minute }}
      </div>
      <div class="date_date">
        {{ time.year }} - {{ time.month }} - {{ time.day }}
      </div>
      <div class="date_week">{{ time.week }}</div>
    </div>
    <!-- wallpaper -->
    <img v-if="type == 'image'" :src="convertFileSrc(path)" class="image" />
    <video v-else-if="type == 'video'" id="wallpapervideo" class="video" :src="convertFileSrc(path)" autoplay="true"
      loop="true"></video>
    <!-- weather -->
    <div v-if="wallpaperstore.wallpaperConfig[index].config.weather && ws.daily.length > 0" :style="{
      left: `${wallpaperstore.wallpaperConfig[index].config.weatherx}%`,
      top: `${wallpaperstore.wallpaperConfig[index].config.weathery}%`,
    }" class="weather">
      <div class="weather_city ">
        {{ w.city }}
      </div>
      <img class="weather_w" :src="icon1">
      <img class="weather_wind" :src="wind">
      <div class="weather_temp">
        {{ w.temp }}
        <div class="weather_temp_c">°C
        </div>
      </div>
      <div class="weather_temp_rang">
        {{ ws.daily[0].tempMin }}
        <div class="weather_temp_rang_c">°C</div>
        <div style="margin: 0px 8px;">
          -
        </div>
        {{ ws.daily[0].tempMax }}
        <div class="weather_temp_rang_c">°C</div>
      </div>
      <div class="weather_text">
        {{ w.text }}
      </div>
      <div class="weather_winddir">
        {{ w.windDir }}
      </div>
      <div class="weather_rh">
        {{ w.humidity }}% RH
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
  position: relative;
  font-family: 'Quicksand';
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

/* date 日期 */
.date {
  position: absolute;
  z-index: 500;
  width: 400px;
  height: 200px;
  /* background: rgb(223, 223, 233, 0.1); */
  border-radius: 50px;
  /* box-shadow: 0px 0px 10px black; */
  text-align: center;
}

.date_time {
  position: absolute;
  width: 100%;
  height: 60px;
  text-align: center;
  font-size: 90px;
  font-weight: bolder;
  text-align: center;
}

.date_date {
  width: 100%;
  height: 50px;
  text-align: center;
  font-size: 20px;
  left: 0px;
  top: 170px;
  position: absolute;
  text-align: center;
}

.date_week {
  width: 100%;
  height: 50px;
  text-align: center;
  font-size: 18px;
  left: 0px;
  top: 130px;
  position: absolute;
  text-align: center;
}

.date_dot {
  animation: 1s normal 0s infinite ease dot;
}

@keyframes dot {
  0% {
    opacity: 0;
  }

  50% {
    opacity: 0;
  }

  100% {
    opacity: 1;
  }
}
</style>
<style>
.weather {
  width: 200px;
  height: 200px;
  background: rgba(253, 253, 253, 0.1);
  border-radius: 50px;
  position: absolute;
  z-index: 400;
  color: rgba(250, 250, 250, 1);
  box-shadow: 0px 0px 10px black;
}

.weather_city {
  width: 200px;
  height: 30px;
  position: absolute;
  left: 0px;
  top: 7px;
  font-size: 15px;
  font-weight: bold;
  display: flex;
  justify-content: center;
  align-items: center;
  text-shadow: 0px 0px 10px black;
}

.weather_w {
  width: 100px;
  height: 100px;
  position: absolute;
  left: 0px;
  top: 20px;
  filter: drop-shadow(0px 0px 5px black);
}

.weather_wind {
  width: 100px;
  height: 100px;
  position: absolute;
  left: 0px;
  top: 100px;
  filter: drop-shadow(0px 0px 10px gray);
}

.weather_temp {
  width: 100px;
  height: 70px;
  position: absolute;
  left: 100px;
  top: 30px;
  font-size: 30px;
  font-weight: bold;
  display: flex;
  justify-content: center;
  align-items: center;
  text-shadow: 0px 0px 10px black;
}

.weather_temp_c {
  margin-left: 5px;
  height: 30px;
  font-size: 15px;
  display: flex;
  align-items: start;
  text-shadow: 0px 0px 10px black;
}

.weather_temp_rang {
  width: 100px;
  height: 30px;
  position: absolute;
  left: 100px;
  top: 85px;
  font-size: 15px;
  font-weight: bold;
  display: flex;
  justify-content: center;
  align-items: center;
  text-shadow: 0px 0px 10px black;
}

.weather_temp_rang_c {
  margin-left: 1px;
  height: 20px;
  font-size: 8px;
  text-shadow: 0px 0px 10px black;
}

.weather_text {
  width: 100px;
  height: 30px;
  position: absolute;
  left: 100px;
  top: 110px;
  font-size: 15px;
  font-weight: bold;
  display: flex;
  justify-content: center;
  align-items: center;
  text-shadow: 0px 0px 10px black;
}

.weather_winddir {
  width: 100px;
  height: 30px;
  position: absolute;
  left: 100px;
  top: 135px;
  font-size: 15px;
  font-weight: bold;
  display: flex;
  justify-content: center;
  align-items: center;
  text-shadow: 0px 0px 10px black;
}

.weather_rh {
  width: 100px;
  height: 30px;
  position: absolute;
  left: 100px;
  top: 160px;
  font-size: 15px;
  font-weight: bold;
  display: flex;
  justify-content: center;
  align-items: center;
  text-shadow: 0px 0px 10px black;
}
</style>
