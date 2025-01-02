<script setup lang="ts">
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { wallpaperStore } from "../../stores/window";
import { onMounted, ref } from "vue";
const wallpaperstore = wallpaperStore();
const tab = ref(0);
onMounted(() => {
  document
    .querySelector(".v-slide-group__content")
    ?.setAttribute("data-tauri-drag-region", "true");
  // window.addEventListener("storage", (e) => {
  //   if (e.key == "wallpaper") {
  //     wallpaperstore.$hydrate();
  //   }
  // });
});

// const config = ref({
//     audio:0,
//     date:true,
//     datex:0,
//     datey:0,
//     time:true,
//     timex:0,
//     timey:0,
//     weather:true,
//     weatherx:0,
//     weathery:0,
//     netspeed:true,
//     netspeedx:0,
//     netspeedy:0,
//     cpu:true,
//     cpux:0,
//     cpuy:0,
//     memory:true,
//     memoryx:0,
//     memoryy:0,
//     disk:true,
//     diskx:0,
//     disky:0,
//     battery:true,
//     batteryx:0,
//     batteryy:0,
// })
</script>

<template>
  <div style="width: 100vw; height: 100vh; background: transparent">
    <v-tabs
      density="compact"
      v-model="tab"
      align-tabs="center"
      color="deep-purple-accent-4"
      bg-color="primary"
    >
      <v-tab
        v-for="(item, index) in wallpaperstore.wallpaperConfig"
        :text="'屏幕' + item.monitor?.replace('\\\\.\\DISPLAY', '')"
        :value="index"
      >
      </v-tab>
    </v-tabs>
    <v-tabs-window v-model="tab">
      <v-tabs-window-item
        v-for="(item, index) in wallpaperstore.wallpaperConfig"
        :value="index"
        style="width: 100vw; height: calc(100vh - 36px)"
      >
        <div
          style="
            overflow: hidden;
            overflow-y: scroll;
            width: 100%;
            height: 100%;
          "
        >
          <v-list lines="one" select-strategy="classic">
            <!-- 声音 -->
            <v-list-subheader style="background: #d1c4e9"
              >声音</v-list-subheader
            >
            <v-list-item>
              <v-list-item-title>静音</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.audio"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-divider></v-divider>
            <!-- 日期 -->
            <v-list-subheader style="background: #d1c4e9"
              >日期</v-list-subheader
            >
            <v-list-item>
              <v-list-item-title>显示</v-list-item-title>
              <template v-slot:append>
                <v-switch
                  color="info"
                  hide-details
                  v-model.lazy="item.config.date"
                ></v-switch>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>X坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.datex"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>Y坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.datey"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>字体大小</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.datefontsize"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-divider></v-divider>
            <v-list-subheader style="background: #d1c4e9"
              >时间</v-list-subheader
            >
            <v-list-item>
              <v-list-item-title>显示</v-list-item-title>
              <template v-slot:append>
                <v-switch
                  color="info"
                  hide-details
                  v-model.lazy="item.config.time"
                ></v-switch>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>X坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.timex"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>Y坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.timey"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>字体大小</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.timefontsize"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-divider></v-divider>
            <v-list-subheader style="background: #d1c4e9"
              >天气</v-list-subheader
            >
            <v-list-item>
              <v-list-item-title>显示</v-list-item-title>
              <template v-slot:append>
                <v-switch
                  color="info"
                  hide-details
                  v-model.lazy="item.config.weather"
                ></v-switch>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>X坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.weatherx"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>Y坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.weathery"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <!-- <v-list-item>
                                <v-list-item-title>字体大小</v-list-item-title>
                                <template v-slot:append>
                                    <div style="width: 280px;">
                                        <v-slider v-model.lazy="item.config.weatherfontsize" min="0" max="100" step="1" thumb-label hide-details></v-slider>
                                    </div>
                                </template>
                            </v-list-item> -->
            <v-divider></v-divider>
            <v-list-subheader style="background: #d1c4e9"
              >网速</v-list-subheader
            >
            <v-list-item>
              <v-list-item-title>显示</v-list-item-title>
              <template v-slot:append>
                <v-switch
                  color="info"
                  hide-details
                  v-model.lazy="item.config.netspeed"
                ></v-switch>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>X坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.netspeedx"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>Y坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.netspeedy"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>字体大小</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.netspeedfontsize"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-divider></v-divider>
            <v-list-subheader style="background: #d1c4e9">CPU</v-list-subheader>
            <v-list-item>
              <v-list-item-title>显示</v-list-item-title>
              <template v-slot:append>
                <v-switch
                  color="info"
                  hide-details
                  v-model.lazy="item.config.cpu"
                ></v-switch>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>X坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.cpux"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>Y坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.cpuy"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>字体大小</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.cpufontsize"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-divider></v-divider>
            <v-list-subheader style="background: #d1c4e9"
              >内存</v-list-subheader
            >
            <v-list-item>
              <v-list-item-title>显示</v-list-item-title>
              <template v-slot:append>
                <v-switch
                  color="info"
                  hide-details
                  v-model.lazy="item.config.memory"
                ></v-switch>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>X坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.memoryx"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>Y坐标</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.memoryy"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
            <v-list-item>
              <v-list-item-title>字体大小</v-list-item-title>
              <template v-slot:append>
                <div style="width: 280px">
                  <v-slider
                    v-model.lazy="item.config.memoryfontsize"
                    min="0"
                    max="100"
                    step="1"
                    thumb-label
                    hide-details
                  ></v-slider>
                </div>
              </template>
            </v-list-item>
          </v-list>
        </div>
      </v-tabs-window-item>
    </v-tabs-window>
    <v-btn
      style="
        position: absolute;
        right: 0;
        top: 0;
        background: transparent;
        box-shadow: none;
        height: 36px;
      "
      size="small"
      @click="getCurrentWebviewWindow().close()"
    >
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
