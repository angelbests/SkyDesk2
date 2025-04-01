<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { wallpaperStore } from "../../stores/window";
import { listen, Event } from "@tauri-apps/api/event";
import "qweather-icons/font/qweather-icons.css";
import { currentMonitor, Monitor, monitorFromPoint } from "@tauri-apps/api/window";
import PCMPlayer from "pcm-player";
import { info } from "@tauri-apps/plugin-log";
import { MouseAction, MouseEvent } from "../../types/desktopType";
const wallpaperstore = wallpaperStore();
const index = ref(0);
const route = useRoute();
const show = ref(false);
const path = ref();
const type = ref();
onMounted(async () => {
  document.title = "skydesk2-wallpaper"
  draw();
  const monitor = await currentMonitor();
  index.value = wallpaperstore.wallpaperConfig.findIndex(
    (item) => item.monitor == monitor?.name
  );
  let dom = document.getElementById("wallpapervideo") as HTMLVideoElement;
  dom.volume = wallpaperstore.wallpaperConfig[index.value].config.audio / 100;
  window.addEventListener("storage", (e) => {
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
//////////////////////音频频谱////////////////

const player = new PCMPlayer({
  inputCodec: "Int16",
  channels: 2,
  sampleRate: 32000,
  flushTime: 0,
  fftSize: 512,
});
player.volume(0);
listen("audio_chunk", (e: { payload: number[] }) => {
  player.feed(new Uint8Array(e.payload));
});

const bufferLength = player.analyserNode.frequencyBinCount / 2;
let getdataArray = new Uint8Array(bufferLength * 2);
let dataArray = new Uint8Array(bufferLength);
function draw() {
  requestAnimationFrame(draw);
  // 时域数据
  // player.analyserNode.getByteTimeDomainData(dataArray);
  // 频率数据
  player.analyserNode.getByteFrequencyData(getdataArray);
  const canvas = document.getElementById("music_canvas") as HTMLCanvasElement;
  if (canvas == null) {
    return;
  }
  const canvasCtx = canvas.getContext("2d");
  if (canvasCtx == null) {
    return;
  }

  if (playstatus.value == 5) {
    dataArray = new Uint8Array(bufferLength)
  } else {
    dataArray = getdataArray.slice(0, bufferLength)
  }

  const WIDTH = canvas.width,
    HEIGHT = canvas.height;
  canvasCtx.clearRect(0, 0, WIDTH, HEIGHT);
  canvasCtx.fillStyle = "rgba(220,220,220,1)";
  const angle = (Math.PI * 2) / bufferLength;
  canvasCtx.save();
  canvasCtx.translate(canvas.width / 2, canvas.height / 2);
  for (let i = 0; i < bufferLength; i++) {
    canvasCtx.save();
    canvasCtx.rotate(angle * i + Math.PI);
    canvasCtx.beginPath();

    const h = (dataArray[i] / 256) * 60;
    canvasCtx.roundRect(0, 145, 4, h < 4 ? 4 : h, 4);
    canvasCtx.fill();
    canvasCtx.restore();
  }
  canvasCtx.restore();
}

//////////////////////////手势检测/////////////////////////////
let mouseleftdown: {
  x: number, y: number, in: boolean
} | null = null
listen("desktop", async (e: Event<MouseEvent>) => {
  let dom = document.getElementById("music_img");
  if (!dom) return;
  let monitor = await monitorFromPoint(e.payload.x, e.payload.y) as Monitor
  let current = await currentMonitor()
  if (monitor?.name != current?.name) {
    mouseleftdown = null;
    return
  };
  let x = (e.payload.x - monitor.position.x) / monitor.scaleFactor
  let y = (e.payload.y - monitor.position.y) / monitor.scaleFactor
  let { left, top, right, bottom } = dom.getBoundingClientRect()
  if ((left < x) && (right > x) && (top < y) && (bottom > y) && e.payload.mouse == MouseAction.LeftDown) {
    mouseleftdown = {
      x: e.payload.x,
      y: e.payload.y,
      in: true,
    }
  }
  if (e.payload.mouse == MouseAction.LeftUp && mouseleftdown && mouseleftdown.in) {
    info(`up ${mouseleftdown.x}-${e.payload.x}-${e.payload.x - mouseleftdown.x}`)
    if ((e.payload.x - mouseleftdown.x) > 40) {
      invoke("play_control", { appname: wallpaperstore.wallpaperConfig[index.value].config.musicapp, control: 1 })
    } else if ((e.payload.x - mouseleftdown.x) < 40 && (e.payload.x - mouseleftdown.x) > -40) {
      invoke("play_control", { appname: wallpaperstore.wallpaperConfig[index.value].config.musicapp, control: 0 })
    } else if ((e.payload.x - mouseleftdown.x) < -40) {
      invoke("play_control", { appname: wallpaperstore.wallpaperConfig[index.value].config.musicapp, control: -1 })
    }
  }
  if (e.payload.mouse == MouseAction.LeftUp) {
    mouseleftdown = null
  }
})

//////////////////////////////smtc/////////////////////////////////////////
import { Media, Smtc_Control, TimeLine } from "../../functions/smtc";
let smtc = new Smtc_Control();
const media = ref<Media>({ app: "", title: "", artist: "", album_title: "", media_type: 0, thumb: "", })
const timeline = ref<TimeLine>({ app: "", position: 0, start: 0, end: 0 })
smtc.listen_appstatus(async (e) => {
  if (
    !e.payload.cloudmusic &&
    wallpaperstore.wallpaperConfig[index.value].config.musicapp ==
    "cloudmusic.exe"
  ) {
    media.value.thumb = "";
  } else if (
    !e.payload.qqmusic &&
    wallpaperstore.wallpaperConfig[index.value].config.musicapp ==
    "QQMusic.exe"
  ) {
    media.value.thumb = "";
  } else if (
    !e.payload.spotify &&
    wallpaperstore.wallpaperConfig[index.value].config.musicapp ==
    "Spotify.exe"
  ) {
    media.value.thumb = "";
  }
})

smtc.listen_timeline((e) => {
  if (wallpaperstore.wallpaperConfig[index.value].config.musicapp == e.payload.app) {
    timeline.value = e.payload
  }
})

smtc.listen_media((e) => {
  if (wallpaperstore.wallpaperConfig[index.value].config.musicapp == e.payload.app) {
    media.value = {
      ...e.payload,
      thumb: convertFileSrc(e.payload.thumb)
    }
  }
})
const playstatus = ref(5);
smtc.listen_playstatus((e) => {
  if (
    wallpaperstore.wallpaperConfig[index.value].config.musicapp ==
    e.payload.app
  ) {
    playstatus.value = e.payload.status;
  }
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
    <video id="wallpapervideo" v-else class="video" :src="convertFileSrc(path)" autoplay="true" loop="true"></video>
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
    <!-- music -->
    <div class="music" v-show="wallpaperstore.wallpaperConfig[index].config.music && media.thumb" :style="{
      left: `${wallpaperstore.wallpaperConfig[index].config.musicx}%`,
      top: `${wallpaperstore.wallpaperConfig[index].config.musicy}%`,
      fontSize: `${wallpaperstore.wallpaperConfig[index].config.musicfontsize}px`,
    }">
      <div class="music_title">{{ media.title }} - {{ media.artist }}</div>
      <div class="music_pic" :style="{
        animationPlayState: playstatus == 4 ? 'running' : 'paused',
      }">
        <img id="music_img" :src="media.thumb" class="music_pic_img" />
      </div>
      <div id="timeline">
        <v-progress-circular class="music-progress" color="rgba(223,123,103,0.5)" bg-color="rgba(123,123,123,0.3)"
          :model-value="(timeline.position / timeline.end) * 100" :width="20"></v-progress-circular>
      </div>
      <canvas id="music_canvas" class="music_canvas" width="400" height="400"></canvas>
    </div>
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

.music {
  position: absolute;
  z-index: 215;
  left: 50vw;
  top: 10vh;
  width: 300px;
}

.music_title {
  font-size: 16px;
  height: 80px;
  display: flex;
  justify-content: center;
  align-items: center;
  text-wrap: balance;
  text-align: center;
  text-overflow: clip;
  overflow: hidden;
}

.music_pic {
  z-index: 215;
  position: relative;
  transform-origin: 50% 50%;
  animation: 15s normal 0s infinite linear music;
  width: 100%;
  height: 350px;
  display: flex;
  justify-content: center;
  align-items: center;
}

.music_pic_img {
  width: 300px;
  height: 300px;
  border-radius: 50%;
  border: 35px solid white;
  transition: all 1s linear;
}

.music_canvas {
  position: absolute;
  z-index: 213;
  left: -50px;
  top: 55px;
  display: flex;
  justify-content: center;
  align-items: center;
}

#timeline {
  position: absolute;
  z-index: 216;
  left: -25px;
  top: 80px;
  width: 350px;
  height: 350px;
  /* background-color: blue; */
  display: flex;
  justify-content: center;
  align-items: center;
}

.music-progress {
  width: 285px;
  height: 285px;
}

@keyframes rain {
  0% {
    top: 0px;
  }

  100% {
    top: 400px;
  }
}

.rain {
  animation: rain 0.6s infinite linear;
}
</style>