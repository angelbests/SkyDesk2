<script setup lang="ts">
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { wallpaperStore } from "../../stores/wallpaper";
import { onMounted, ref } from "vue";
const wallpaperstore = wallpaperStore();
const tab = ref(0);
onMounted(() => {
  document
    .querySelector(".v-slide-group__content")
    ?.setAttribute("data-tauri-drag-region", "true");
});
</script>

<template>
  <div style="width: 100vw; height: 100vh; background: transparent">
    <v-tabs density="compact" v-model="tab" align-tabs="center" color="deep-purple-accent-4" bg-color="primary">
      <v-tab v-for="(item, index) in wallpaperstore.wallpaperConfig"
        :text="'屏幕' + item.monitor?.replace('\\\\.\\DISPLAY', '')" :value="index">
      </v-tab>
    </v-tabs>
    <v-tabs-window v-model="tab">
      <v-tabs-window-item v-for="(item, index) in wallpaperstore.wallpaperConfig" :value="index"
        style="width: 100vw; height: calc(100vh - 36px)">
        <div style="
            overflow: hidden;
            overflow-y: scroll;
            width: 100%;
            height: 100%;
          ">
          <v-list lines="one" select-strategy="classic">
            <!-- 动效 -->
            <v-list-subheader style="background: #d1c4e9">动效</v-list-subheader>
            <v-list-item>
              <v-list-item-title>视差</v-list-item-title>
              <template v-slot:append>
                <v-switch color="info" hide-details v-model="item.config.action"></v-switch>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>樱花</v-list-item-title>
              <template v-slot:append>
                <v-switch color="info" hide-details v-model="item.config.sakura"></v-switch>
              </template>
            </v-list-item>
            <!-- 声音 -->
            <v-list-subheader style="background: #d1c4e9">声音</v-list-subheader>
            <v-list-item>
              <v-list-item-title>声音</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider :model-value="item.config.audio" @end="(e) => { item.config.audio = e }" min="0" max="100"
                    step="1" thumb-label hide-details></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-divider></v-divider>
            <!-- 日期 -->
            <v-list-subheader style="background: #d1c4e9">日期</v-list-subheader>
            <v-list-item>
              <v-list-item-title>显示</v-list-item-title>
              <template v-slot:append>
                <v-switch color="info" hide-details v-model.lazy="item.config.date"></v-switch>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>X坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider :model-value="item.config.datex" @end="e => item.config.datex = e" min="0" max="100"
                    step="1" thumb-label hide-details></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>Y坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider :model-value="item.config.datey" @end="e => item.config.datey = e" min="0" max="100"
                    step="1" thumb-label hide-details></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title> 字体颜色 </v-list-item-title>
              <template v-slot:append>
                <v-radio-group v-model="item.config.datecolor" inline density="compact" hide-details="auto">
                  <v-radio style="width: 100px" label="白色" value="white"></v-radio>
                  <v-radio style="width: 100px" label="黑色" value="black"></v-radio>
                </v-radio-group>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>阴影</v-list-item-title>
              <template v-slot:append>
                <v-switch color="info" hide-details v-model.lazy="item.config.dateshadow"></v-switch>
              </template>
            </v-list-item>
            <!-- 天气 -->
            <v-divider></v-divider>
            <v-list-subheader style="background: #d1c4e9">天气</v-list-subheader>
            <v-list-item>
              <v-list-item-title>显示</v-list-item-title>
              <template v-slot:append>
                <v-switch color="info" hide-details v-model.lazy="item.config.weather"></v-switch>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>X坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider :model-value="item.config.weatherx" @end="e => item.config.weatherx = e" min="0" max="100"
                    step="1" thumb-label hide-details></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>Y坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider :model-value="item.config.weathery" @end="e => item.config.weathery = e" min="0" max="100"
                    step="1" thumb-label hide-details></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>阴影</v-list-item-title>
              <template v-slot:append>
                <v-switch color="info" hide-details v-model.lazy="item.config.weathershadow"></v-switch>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title> 预报 </v-list-item-title>
              <template v-slot:append>
                <v-radio-group v-model="item.config.weatherd7" inline density="compact" hide-details="auto">
                  <v-radio style="width: 100px" label="当天" value="1"></v-radio>
                  <v-radio style="width: 100px" label="七日" value="7"></v-radio>
                </v-radio-group>
              </template>
            </v-list-item>
            <v-divider></v-divider>
            <!-- 音乐 -->
            <v-list-subheader style="background: #d1c4e9">音乐</v-list-subheader>
            <v-list-item>
              <v-list-item-title>显示</v-list-item-title>
              <template v-slot:append>
                <v-switch color="info" hide-details v-model.lazy="item.config.music"></v-switch>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>X坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider :model-value="item.config.musicx" @end="e => item.config.musicx = e" min="0" max="100"
                    step="1" thumb-label hide-details></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>Y坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider :model-value="item.config.musicy" @end="e => item.config.musicy = e" min="0" max="100"
                    step="1" thumb-label hide-details></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>阴影</v-list-item-title>
              <template v-slot:append>
                <v-switch color="info" hide-details v-model.lazy="item.config.musicshadow"></v-switch>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title> 设备 </v-list-item-title>
              <template v-slot:append>
                <v-radio-group v-model="item.config.musictype" inline density="compact" hide-details="auto">
                  <v-radio style="width: 100px" label="胶片" value="1"></v-radio>
                  <v-radio style="width: 100px" label="光盘" value="2"></v-radio>
                  <!-- <v-radio style="width: 100px" label="磁带" value="3"></v-radio> -->
                </v-radio-group>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title> 应用 </v-list-item-title>
              <template v-slot:append>
                <v-radio-group v-model="item.config.musicapp" inline density="compact" hide-details="auto">
                  <v-radio style="width: 100px" label="网易云" value="cloudmusic.exe"></v-radio>
                  <v-radio style="width: 100px" label="QQ" value="QQMusic.exe"></v-radio>
                  <v-radio style="width: 100px" label="Spotify" value="Spotify.exe"></v-radio>
                </v-radio-group>
              </template>
            </v-list-item>
            <!-- 日历 -->
            <v-divider></v-divider>
            <v-list-subheader style="background: #d1c4e9">日历</v-list-subheader>
            <v-list-item>
              <v-list-item-title>显示</v-list-item-title>
              <template v-slot:append>
                <v-switch color="info" hide-details v-model.lazy="item.config.calendar"></v-switch>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>X坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider :model-value="item.config.musicx" @end="e => item.config.calendarx = e" min="0" max="100"
                    step="1" thumb-label hide-details></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>Y坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider :model-value="item.config.musicy" @end="e => item.config.calendary = e" min="0" max="100"
                    step="1" thumb-label hide-details></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>阴影</v-list-item-title>
              <template v-slot:append>
                <v-switch color="info" hide-details v-model.lazy="item.config.calendarshadow"></v-switch>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title> 字体颜色 </v-list-item-title>
              <template v-slot:append>
                <v-radio-group v-model="item.config.calendarcolor" inline density="compact" hide-details="auto">
                  <v-radio style="width: 100px" label="白色" value="white"></v-radio>
                  <v-radio style="width: 100px" label="黑色" value="black"></v-radio>
                </v-radio-group>
              </template>
            </v-list-item>
          </v-list>
        </div>

        calendar: false,
        calendarx: 0,
        calendary: 0,
        calendarcolor: 'white',
      </v-tabs-window-item>
    </v-tabs-window>
    <v-btn style="
        position: absolute;
        right: 0;
        top: 0;
        background: transparent;
        box-shadow: none;
        height: 36px;
      " size="small" @click="getCurrentWebviewWindow().close()">
      <v-icon style="font-size: 30px">mdi-close</v-icon>
    </v-btn>
  </div>
</template>

<style>
html,
body {
  border-radius: 8px;
  background: transparent !important;
}
</style>
