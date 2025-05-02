<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { Media, Smtc_Control, TimeLine } from "../functions/smtc";
import { wallpaperStore } from "../stores/wallpaper";
import { convertFileSrc } from "@tauri-apps/api/core";
import { info } from "@tauri-apps/plugin-log";
import { currentMonitor } from "@tauri-apps/api/window"
import { desktopMouseControl } from "../functions/wallpaper";
import { listen } from "@tauri-apps/api/event";
import PCMPlayer from "pcm-player";
const wallpaperstore = wallpaperStore()
const index = ref(0)
let cancel_listen_desktop: any
let cancel_listen_audio_chunk: any

onMounted(async () => {
    const monitor = await currentMonitor();
    index.value = wallpaperstore.wallpaperConfig.findIndex(
        (item) => item.monitor == monitor?.name
    );
    window.addEventListener("storage", (e) => {
        if (e.key == "wallpaper") {
            wallpaperstore.$hydrate();
        }
    });
    cancel_listen_desktop = await desktopMouseControl("music_img", index.value)
    draw()
})
//////////////////////音频频谱////////////////
const player = new PCMPlayer({
    inputCodec: "Int16",
    channels: 2,
    sampleRate: 32000,
    flushTime: 0,
    fftSize: 256,
});

listen("audio_chunk", (e: { payload: number[] }) => {
    player.feed(new Uint8Array(e.payload));
    // console.log(e.payload)
});
player.volume(0);
const bufferLength = player.analyserNode.frequencyBinCount;
let getdataArray = new Uint8Array(bufferLength);
let dataArray = new Uint8Array(bufferLength);
function draw() {
    requestAnimationFrame(draw);
    // 时域数据
    // player.analyserNode.getByteTimeDomainData(getdataArray);
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
    // dataArray = getdataArray.slice(0, bufferLength)
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
        const h = dataArray[i] / 255 * 60;
        canvasCtx.roundRect(0, 143, 4, h < 4 ? 4 : h, 4);
        canvasCtx.fill();
        canvasCtx.restore();
    }
    canvasCtx.restore();
}

//////////////////////////////smtc/////////////////////////////////////////

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
    if (!Boolean(e.payload.thumb)) return
    if (wallpaperstore.wallpaperConfig[index.value].config.musicapp == e.payload.app) {
        media.value = {
            ...e.payload,
            thumb: convertFileSrc(e.payload.thumb)
        }
    }
    playstatus.value = 4
})
const playstatus = ref(5);
smtc.listen_playstatus((e) => {
    if (
        wallpaperstore.wallpaperConfig[index.value].config.musicapp ==
        e.payload.app
    ) {
        playstatus.value = e.payload.status;
    }
    info(JSON.stringify(e.payload))
})

onUnmounted(() => {
    smtc.cancel_listen_media()
    smtc.cancel_listen_apps()
    smtc.cancel_listen_playstatus()
    smtc.cancel_listen_timeline()
    if (cancel_listen_desktop) cancel_listen_desktop()
    if (cancel_listen_audio_chunk) cancel_listen_audio_chunk()
})
</script>

<template>
    <div class="music_vinyl vinyl_shadow" v-show="media.thumb">
        <img id="music_img" :style="{ animationPlayState: playstatus == 4 ? 'running' : 'paused' }" :src="media.thumb"
            class="music_pic_img" />
        <v-progress-circular class="music-progress" color="rgba(223,123,103,0.5)" bg-color="rgba(123,123,123,0.3)"
            :model-value="(timeline.position / timeline.end) * 100" :width="20"></v-progress-circular>
        <canvas v-show="playstatus == 4 ? true : false" id="music_canvas" class="music_canvas" width="400"
            height="400"></canvas>
    </div>
</template>

<style>
.vinyl_shadow {
    filter: drop-shadow(0px 0px 10px black);
}

.music_vinyl {
    position: absolute;
    z-index: 215;
    width: 300px;
    height: 300px;
    display: flex;
    justify-content: center;
    align-items: center;
}

.music_pic_img {
    position: absolute;
    animation: 10s normal 0s infinite linear music;
    width: 290px;
    height: 290px;
    border-radius: 50%;
    border: 30px solid rgba(223, 233, 223, 1);
    transition: all 1s linear;
}

.music_canvas {
    position: absolute
}

.music-progress {
    width: 285px;
    height: 285px;
}
</style>