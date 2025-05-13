<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { Media, Smtc_Control } from "../../functions/smtc";
import { convertFileSrc } from "@tauri-apps/api/core";
import { wallpaperStore } from "../../stores/wallpaper";
import { info } from "@tauri-apps/plugin-log";
import { currentMonitor } from "@tauri-apps/api/window";
import { desktopMouseControl } from "../../functions/wallpaper";
const wallpaperstore = wallpaperStore()
const index = ref(0);
const media = ref<Media>({ app: "", title: "", artist: "", album_title: "", media_type: 0, thumb: "", })
const new_thumb = ref("");
const history_thumb = ref("");
const show = ref(false)
let cancel_listen_desktop: any
let smtc = new Smtc_Control();
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
    cancel_listen_desktop = await desktopMouseControl("music_disk", index.value)
})

smtc.listen_media((e) => {
    if (!Boolean(e.payload.thumb)) return
    if (wallpaperstore.wallpaperConfig[index.value].config.musicapp == e.payload.app) {
        info(show.value.toString())
        let dom = document.getElementById("disk")
        let dom2 = document.getElementById("disk2")
        let dom3 = document.getElementById("disk_thumb");
        if (!dom) return
        if (!dom2) return
        if (!dom3) return
        info(JSON.stringify(e.payload))
        history_thumb.value = history_thumb.value ? media.value.thumb : convertFileSrc(e.payload.thumb);
        new_thumb.value = convertFileSrc(e.payload.thumb)
        dom.style.animation = 'music_disk 0.5s linear 1'
        dom2.style.animation = 'music_disk2 0.5s linear 1'
        const preload = new Image();
        preload.src = new_thumb.value;
        preload.onload = () => {
            media.value = {
                ...e.payload,
                thumb: convertFileSrc(e.payload.thumb)
            }
            show.value = true
        }
        setTimeout(() => {
            dom.style.animation = 'none'
            dom2.style.animation = 'none'
        }, 500)
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

smtc.listen_appstatus(async (e) => {
    if (
        !e.payload.cloudmusic &&
        wallpaperstore.wallpaperConfig[index.value].config.musicapp ==
        "cloudmusic.exe"
    ) {
        show.value = false
    } else if (
        !e.payload.qqmusic &&
        wallpaperstore.wallpaperConfig[index.value].config.musicapp ==
        "QQMusic.exe"
    ) {
        show.value = false
    } else if (
        !e.payload.spotify &&
        wallpaperstore.wallpaperConfig[index.value].config.musicapp ==
        "Spotify.exe"
    ) {
        show.value = false
    }
})

onUnmounted(() => {
    smtc.cancel_listen_media()
    smtc.cancel_listen_apps()
    smtc.cancel_listen_playstatus()
    cancel_listen_desktop()
})
</script>

<template>
    <div id="music_disk" class="music_disk_container" v-show="show">
        <svg width="0" height="0">
            <!-- objectBoundingBox  userSpaceOnUs-->
            <mask id="cutout-mask" maskUnits="userSpaceOnUse" x="0" y="0" width="300" height="300">
                <!-- 整张图为白色（可见） -->
                <rect width="300" height="300" fill="white" />
                <!-- 挖掉中间的下半圆（黑色区域） -->
                <path d="
          M 260 50 
          A 1 1 0 0 0 260 250
        " fill="black" opacity="0.7" />
                <path d="
          M 260 50
          L 300 50
          L 300 250
          L 260 250
          Z
        " fill="black" opacity="0.7" />
            </mask>
        </svg>
        <svg width="0" height="0">
            <mask id="cutout-mask2" maskUnits="userSpaceOnUse" x="0" y="0" width="290" height="290">
                <!-- 整张图为白色（可见） -->
                <rect width="280" height="280" fill="white" />
                <!-- 挖掉中间的下半圆（黑色区域） -->
                <circle cx="140" cy="140" r="10" fill="black" />
                <circle cx="140" cy="140" r="130" stroke="black" stroke-width="40" fill="none" />
            </mask>
        </svg>
        <!-- disk -->
        <div style="z-index: 301;left: 15px;top: 15px;" class="disk_div">
            <img :src="history_thumb ? history_thumb : new_thumb" class="disk">
        </div>
        <div style="position: absolute;z-index: 301;" class="w">
            <img :src="history_thumb" style="top: 15px;left: 15px;" class="disk_thumb" />
        </div>
        <!-- disk -->
        <div style="z-index: 301;left: 10px;top: 10px;" class="disk_shadow disk_div">
            <img :src="new_thumb" class="disk">
        </div>
        <div style="position: absolute;z-index: 301;" class="disk_shadow">
            <img :src="media.thumb" style="top: 10px;left: 10px;" class="disk_thumb" />
        </div>
        <!-- disk -->
        <div id="disk" style="z-index: 302;left: 5px;top: 5px;" class="disk_shadow disk_div">
            <img :src="history_thumb ? history_thumb : new_thumb" class="disk">
        </div>
        <div style="position: absolute;z-index: 302;" class="disk_shadow">
            <img :src="history_thumb" style="top: 5px;left: 5px;" class="disk_thumb" />
        </div>
        <!-- disk -->
        <div id="disk2" style="z-index: 304;left: 0px;" class="disk_shadow disk_div">
            <img :src="new_thumb" class="disk" :style="{ animationPlayState: playstatus == 4 ? 'running' : 'paused' }">
        </div>
        <!-- <div style="position: absolute;z-index: 310;background: rgba(223,232,223,0.9);width: 300px;height: 300px;"></div> -->
        <div style="position: absolute;z-index: 310;" class="disk_shadow">
            <img id="disk_thumb" :src="media.thumb" class="disk_thumb" />
        </div>
    </div>

</template>

<style>
.music_disk_container {
    position: absolute;
    left: 1400px;
    top: 600px;
    z-index: 300;
    box-sizing: border-box;
    width: 400px;
    height: 360px;
}

.disk_shadow {
    filter: drop-shadow(0px 0px 10px black);
}

.disk_div {
    position: relative;
    left: 0px;
    top: 0px;
}

.disk {
    width: 280px;
    height: 280px;
    position: absolute;
    border-radius: 50%;
    left: 120px;
    top: 10px;
    animation: 10s normal 0s infinite linear music;
    mask: url(#cutout-mask2);
    mask-type: alpha;
    border: 50px solid rgba(213, 213, 213, 1);

}

.disk_thumb {
    width: 300px;
    height: 300px;
    position: absolute;
    left: 0;
    top: 0;
    mask: url(#cutout-mask);
    mask-type: alpha;
    border-radius: 10px;
    border: 20px solid rgba(213, 213, 213, 1);
}

.path_rect {
    background: rgba(123, 123, 123, 0.1);
}

@keyframes music_disk {
    0% {
        left: 0px;
        z-index: 305;
    }

    50% {
        left: -100px;
        z-index: 302;
    }

    100% {
        left: 0px;
        z-index: 302;
    }
}

@keyframes music_disk2 {
    0% {
        left: 0px;
        z-index: 303;
    }

    50% {
        left: 240px;
        z-index: 302;
    }

    100% {
        left: 0px;
        z-index: 302;
    }
}
</style>