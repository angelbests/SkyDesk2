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
        const h = dataArray[i] / 255 * 35;
        canvasCtx.roundRect(0, 95, 4, h < 4 ? 4 : h, 4);
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
const new_thumb = ref("")
smtc.listen_media((e) => {
    if (!Boolean(e.payload.thumb)) return
    if (wallpaperstore.wallpaperConfig[index.value].config.musicapp == e.payload.app) {
        new_thumb.value = convertFileSrc(e.payload.thumb)
        let dom1 = document.getElementById("vinyl1")
        let dom2 = document.getElementById("vinyl2")
        if (!dom1) return
        if (!dom2) return
        dom1.style.animation = "vinyl1 0.5s linear 1"
        dom2.style.animation = "vinyl2 0.5s linear 1"
        setTimeout(() => {
            dom1.style.animation = "none"
            dom2.style.animation = "none"
            media.value = {
                ...e.payload,
                thumb: convertFileSrc(e.payload.thumb)
            }
        }, 500)
        // tonearmControl(5)
        // setTimeout(() => {
        //     tonearmControl(4)
        // }, 100)
    }
})
const playstatus = ref(5);
smtc.listen_playstatus((e) => {
    if (
        wallpaperstore.wallpaperConfig[index.value].config.musicapp ==
        e.payload.app
    ) {
        playstatus.value = e.payload.status;
        tonearmControl(playstatus.value)
    }
    info(JSON.stringify(e.payload))
})

const tonearmControl = function (status: number) {
    let tonearmanimate = document.getElementById('tonearmanimate') as SVGAnimateMotionElement | null
    let svg = document.getElementById('tonearm') as SVGSVGElement | null
    if (!svg) return
    if (!tonearmanimate) return

    if (status == 5) {
        svg.pauseAnimations()
        tonearmanimate.removeAttribute("values")
        tonearmanimate.setAttribute("to", "-13,280,40")
        tonearmanimate.setAttribute("repeatCount", "1")
        tonearmanimate.setAttribute("dur", "0.1s")
        tonearmanimate.beginElement()
        svg.unpauseAnimations()
    } else if (status == 4) {
        tonearmanimate.removeAttribute("to")
        tonearmanimate.setAttribute("values", "-13,280,40;0,280,40")
        tonearmanimate.setAttribute("repeatCount", "1")
        tonearmanimate.setAttribute("dur", "0.1s")
        svg.unpauseAnimations()
        tonearmanimate.beginElement()
        setTimeout(() => {
            tonearmanimate.setAttribute("values", "0,280,40;2,280,40;0,280,40")
            tonearmanimate.setAttribute("repeatCount", "indefinite")
            tonearmanimate.setAttribute("dur", "5s")
            tonearmanimate.beginElement()
        }, 300)
    }
    info(status.toString())
}

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
    <!--  :style="{ background: `url(${media.thumb})`, backgroundSize: 'cover' }" -->
    <div class="music_vinyl" v-show="media.thumb"
        :style="{ background: `url(${media.thumb})`, backgroundSize: 'cover' }">
        <svg id="tonearm" style="position: absolute;z-index: 300;" width="400" height="300" viewBox="0 0 400 300">
            <g id="tonearmg" transform="rotate(0,280,40)" style="filter: drop-shadow(0px 0px 2px black);">
                <path d=" M 282 10 C 270 230, 270 200, 245 235" stroke="#304352" fill="none" stroke-width="4"
                    stroke-linecap="round">
                </path>
                <g style="filter: drop-shadow(0px 0px 1px black);">
                    <rect x="270" y="120" width="14" height="20" fill="gray" transform="rotate(50,144,144)" rx="2"
                        ry="2" />
                </g>
                <circle cx="280" cy="40" r="10" fill="gray" />
                <circle cx="280" cy="40" r="4" fill="#304352" />
                <animateTransform id="tonearmanimate" attributeName="transform" attributeType="XML" type="rotate"
                    values="0,280,40;2,280,40;0,280,40" dur="6s" repeatCount="indefinite" fill="freeze"
                    begin="indefinite" />
            </g>

        </svg>
        <div id="vinyl1" class="vinyl">
            <!-- 封面 -->
            <img id="music_img" :style="{ animationPlayState: playstatus == 4 ? 'running' : 'paused' }"
                :src="media.thumb" class="music_pic_img" />
            <!-- 时间线进度 -->
            <v-progress-circular class="music-progress" bg-color="#00f2fe" color="#304352"
                :model-value="(timeline.position / timeline.end) * 100" :width="10" :size="190"></v-progress-circular>
            <div class="circle_border" :style="{ animationPlayState: playstatus == 4 ? 'running' : 'paused' }"></div>
        </div>
        <div id="vinyl2" class="vinyl">
            <!-- 封面 -->
            <img id="music_img" :style="{ animationPlayState: playstatus == 4 ? 'running' : 'paused' }" :src="new_thumb"
                class="music_pic_img" />
            <!-- 环形柱状图 -->
            <canvas v-show="playstatus == 4 ? true : false" id="music_canvas" class="music_canvas" width="270"
                height="270"></canvas>
            <!-- 时间线进度 -->
            <v-progress-circular class="music-progress" bg-color="#00f2fe" color="#304352" tag="div"
                :model-value="(timeline.position / timeline.end) * 100" :width="10" :size="190"></v-progress-circular>
            <div class="circle_border" :style="{ animationPlayState: playstatus == 4 ? 'running' : 'paused' }"></div>
        </div>
    </div>

</template>

<style>
.music_vinyl {
    position: absolute;
    z-index: 215;
    width: 320px;
    height: 320px;
    background: rgba(203, 203, 223, 0.9);
    border-radius: 50px;
    filter: drop-shadow(0px 0px 10px black);
    border: 5px solid white;
    padding: 0px;
    margin: 0px;
}

.vinyl {
    position: absolute;
    z-index: 216;
    left: -5px;
    top: -5px;
    width: 320px;
    height: 320px;
    display: flex;
    justify-content: center;
    align-items: center;
    /* animation: 1s normal 0s infinite linear vinyl; */
}

@keyframes vinyl1 {
    0% {
        top: -5px;
        left: -5px;
        opacity: 1;
    }

    100% {
        top: 290px;
        left: -100px;
        opacity: 0;
    }
}

@keyframes vinyl2 {
    0% {
        top: -280px;
        left: -100px;
        opacity: 0;
    }

    100% {
        top: -5px;
        left: -5px;
        opacity: 1;
    }

}

.music_pic_img {
    position: absolute;
    z-index: 211;
    animation: 10s normal 0s infinite linear music;
    width: 170px;
    height: 170px;
    border-radius: 50%;
}

.music_canvas {
    position: absolute;
    z-index: 209;
    border-radius: 50%;
}

.music-progress {
    box-sizing: content-box;
    position: absolute;
    z-index: 210;
}

.circle_border {
    position: absolute;
    z-index: 208;
    width: 270px;
    height: 270px;
    border-radius: 50%;
    background-image: linear-gradient(60deg, #304352 0%, black 40%, #434322 60%, #304352 100%);
    /* background-image: linear-gradient(to right, #434322 0%, #304352 100%); */
    animation: 10s normal 0s infinite linear music;
    filter: drop-shadow(0px 0px 10px black);
}
</style>