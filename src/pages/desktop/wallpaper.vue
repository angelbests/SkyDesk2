<script setup lang="ts">
import { onMounted, ref, toRefs, watch } from "vue";
import { useRoute } from "vue-router";
import { convertFileSrc } from "@tauri-apps/api/core";
import { wallpaperStore, weatherStore } from "../../stores/window";
import { getWeather } from "../../api/weather";
import { listen } from "@tauri-apps/api/event";
import "qweather-icons/font/qweather-icons.css";
import { currentMonitor } from "@tauri-apps/api/window";
const weatherstore = weatherStore();
const wallpaperstore = wallpaperStore();
const index = ref(0);
const { city, apikey, citycode } = toRefs(weatherstore);
const route = useRoute();
const show = ref(false);
const path = ref();
const type = ref();
const net = ref({
  speed_r: 0,
  speed_s: 0,
});
const cpu = ref(0);
const memory = ref(0);
const date = new Date();
let week = "";
switch (date.getDay()) {
  case 0:
    week = "星期天";
    break;
  case 1:
    week = "星期一";
    break;
  case 2:
    week = "星期二";
    break;
  case 3:
    week = "星期三";
    break;
  case 4:
    week = "星期四";
    break;
  case 5:
    week = "星期五";
    break;
  case 6:
    week = "星期六";
    break;
}
const time = ref<{
  year: string;
  month: string;
  day: string;
  week: string;
  hour: string;
  minute: string;
  second: string;
}>({
  year: date.getFullYear() + "",
  month:
    date.getMonth() + 1 > 9
      ? date.getMonth() + 1 + ""
      : "0" + (date.getMonth() + 1),
  day: date.getDate() < 10 ? "0" + date.getDate() : date.getDate() + "",
  week: week,
  hour: date.getHours() < 10 ? "0" + date.getHours() : date.getHours() + "",
  minute:
    date.getMinutes() < 10 ? "0" + date.getMinutes() : date.getMinutes() + "",
  second:
    date.getSeconds() < 10 ? "0" + date.getSeconds() : date.getSeconds() + "",
});
onMounted(async () => {
  const monitor = await currentMonitor();
  index.value = wallpaperstore.wallpaperConfig.findIndex(
    (item) => item.monitor == monitor?.name
  );
  let dom = document.getElementById("wallpapervideo") as HTMLVideoElement;
  dom.volume = wallpaperstore.wallpaperConfig[index.value].config.audio / 100;
  window.addEventListener("storage", (e) => {
    if (e.key == "weather") {
      weatherstore.$hydrate();
    }
    if (e.key == "wallpaper") {
      wallpaperstore.$hydrate();
      let dom = document.getElementById("wallpapervideo") as HTMLVideoElement;
      dom.volume =
        wallpaperstore.wallpaperConfig[index.value].config.audio / 100;
    }
  });
  path.value = route.query.path;
  type.value = route.query.type;
  setTimeout(() => {
    show.value = true;
  }, 500);

  setInterval(async () => {
    let res = await getWeather(citycode.value);
    console.log(res);
    if (res.code == 200) {
      w.value = res.now;
    }
  }, 60 * 60 * 60);
  listen("netspeed", (e) => {
    let res = JSON.parse(e.payload as string);
    net.value.speed_r = res.speed_r;
    net.value.speed_s = res.speed_s;
  });
  setInterval(() => {
    let date = new Date();
    let week = "";
    switch (date.getDay()) {
      case 0:
        week = "星期天";
        break;
      case 1:
        week = "星期一";
        break;
      case 2:
        week = "星期二";
        break;
      case 3:
        week = "星期三";
        break;
      case 4:
        week = "星期四";
        break;
      case 5:
        week = "星期五";
        break;
      case 6:
        week = "星期六";
        break;
    }
    time.value = {
      year: date.getFullYear() + "",
      month:
        date.getMonth() + 1 > 9
          ? date.getMonth() + 1 + ""
          : "0" + (date.getMonth() + 1),
      day: date.getDate() < 10 ? "0" + date.getDate() : date.getDate() + "",
      week: week,
      hour: date.getHours() < 10 ? "0" + date.getHours() : date.getHours() + "",
      minute:
        date.getMinutes() < 10
          ? "0" + date.getMinutes()
          : date.getMinutes() + "",
      second:
        date.getSeconds() < 10
          ? "0" + date.getSeconds()
          : date.getSeconds() + "",
    };
  }, 1000);

  listen("cpu", (e) => {
    let str = e.payload;
    cpu.value = Math.trunc(Math.round(Number(str)));
  });

  listen("memory", (e) => {
    let str = e.payload;
    memory.value = Math.trunc(Number(str) * 100);
  });
});
const w = ref<{
  temp: string; // 温度
  feelsLike: string; // 体感温度
  icon: string; // 天气图标
  text: string; // 文本描述
  windDir: string; // 风向
  windScale: string; // 风等级
  windSpeed: string; // 风速
  humidity: string; // 相对湿度
  precip: string; // 过去1小时降水量，默认单位：毫米
  pressure: string; // 大气压强
  vis: string; // 能见度
  cloud: string; // 云量
  dew: string; // 露点温度
}>({
  temp: "", // 温度
  feelsLike: "", // 体感温度
  icon: "", // 天气图标
  text: "", // 文本描述
  windDir: "", // 风向
  windScale: "", // 风等级
  windSpeed: "", // 风速
  humidity: "", // 相对湿度
  precip: "", // 过去1小时降水量，默认单位：毫米
  pressure: "", // 大气压强
  vis: "", // 能见度
  cloud: "", // 云量
  dew: "", // 露点温度
});
watch(
  city,
  async () => {
    if (apikey.value && city) {
      let res = await getWeather(citycode.value);
      console.log(res);
      if (res.code == 200) {
        w.value = res.now;
      }
    }
  },
  {
    immediate: true,
  }
);
</script>

<template>
  <div class="window">
    <img v-if="type == 'image'" :src="convertFileSrc(path)" class="image" />
    <video
      id="wallpapervideo"
      v-else
      class="video"
      :src="convertFileSrc(path)"
      autoplay="true"
      loop="true"
    ></video>
    <!-- 天气 -->
    <div
      class="weather"
      v-show="wallpaperstore.wallpaperConfig[index].config.weather"
      :style="{
        left: `${wallpaperstore.wallpaperConfig[index].config.weatherx}%`,
        top: `${wallpaperstore.wallpaperConfig[index].config.weathery}%`,
      }"
    >
      <div style="width: 200px; height: 30px; text-align: center">
        {{ city }}
      </div>
      <div style="width: 200px; display: flex; height: 60px">
        <div
          style="
            width: 100px;
            height: 50px;
            line-height: 50px;
            text-align: center;
          "
        >
          {{ w.text }} {{ w.temp }}°
        </div>
        <div
          style="
            width: 100px;
            height: 50px;
            text-align: center;
            line-height: 50px;
          "
        >
          <i
            style="font-size: 30px"
            :class="['qi-' + (w.icon ? w.icon : 100), 'weather-icon']"
          ></i>
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
    <div
      class="netspeed"
      v-show="wallpaperstore.wallpaperConfig[index].config.netspeed"
      :style="{
        left: `${wallpaperstore.wallpaperConfig[index].config.netspeedx}%`,
        top: `${wallpaperstore.wallpaperConfig[index].config.netspeedy}%`,
        fontSize: `${wallpaperstore.wallpaperConfig[index].config.netspeedfontsize}px`,
      }"
    >
      <div data-tauri-drag-region style="display: flex; align-items: center">
        <v-icon data-tauri-drag-region>mdi-arrow-down-thin</v-icon
        >{{
          Math.trunc(net.speed_r / 1024) < 1024
            ? Math.trunc(net.speed_r / 1024) + "KB/s"
            : Math.trunc((net.speed_r / 1024 / 1024) * 10) / 10 + "MB/s"
        }}
      </div>
      <div style="display: flex; align-items: center">
        <v-icon data-tauri-drag-region>mdi-arrow-up-thin</v-icon
        >{{
          Math.trunc(net.speed_s / 1024) < 1024
            ? Math.trunc(net.speed_s / 1024) + "KB/s"
            : Math.trunc((net.speed_s / 1024 / 1024) * 10) / 10 + "MB/s"
        }}
      </div>
    </div>
    <!-- time -->
    <div
      class="time"
      v-show="wallpaperstore.wallpaperConfig[index].config.time"
      :style="{
        left: `${wallpaperstore.wallpaperConfig[index].config.timex}%`,
        top: `${wallpaperstore.wallpaperConfig[index].config.timey}%`,
        fontSize: `${wallpaperstore.wallpaperConfig[index].config.timefontsize}px`,
      }"
    >
      <div class="hour">
        {{ time.hour }}:{{ time.minute }}:{{ time.second }}
      </div>
    </div>
    <div
      class="date"
      v-show="wallpaperstore.wallpaperConfig[index].config.date"
      :style="{
        left: `${wallpaperstore.wallpaperConfig[index].config.datex}%`,
        top: `${wallpaperstore.wallpaperConfig[index].config.datey}%`,
        fontSize: `${wallpaperstore.wallpaperConfig[index].config.datefontsize}px`,
      }"
    >
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
    <div
      class="cpu"
      v-show="wallpaperstore.wallpaperConfig[index].config.cpu"
      :style="{
        left: `${wallpaperstore.wallpaperConfig[index].config.cpux}%`,
        top: `${wallpaperstore.wallpaperConfig[index].config.cpuy}%`,
        fontSize: `${wallpaperstore.wallpaperConfig[index].config.cpufontsize}px`,
      }"
    >
      CPU：{{ cpu }}%
    </div>
    <div
      class="memory"
      v-show="wallpaperstore.wallpaperConfig[index].config.memory"
      :style="{
        left: `${wallpaperstore.wallpaperConfig[index].config.memoryx}%`,
        top: `${wallpaperstore.wallpaperConfig[index].config.memoryy}%`,
        fontSize: `${wallpaperstore.wallpaperConfig[index].config.memoryfontsize}px`,
      }"
    >
      内存：{{ memory }}%
    </div>
  </div>
</template>

<style scoped>
@font-face {
  font-family: HarmonyOS_Sans_TC_Light;
  src: url("fonts/HarmonyOS_Sans_TC_Light.ttf") format("truetype");
}
.window {
  font-family: HarmonyOS_Sans_TC_Light simsun;
  width: 100vw;
  height: 100vh;
  position: relative;
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
