<script setup lang="ts">
import { captureStore, systemStore, windowStore } from "../../stores/window";
import { storeToRefs } from "pinia";
import { uuid } from "../../functions";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { onMounted } from "vue";
import { convertFileSrc } from "@tauri-apps/api/core";
const windowstore = windowStore();
const { monitors } = storeToRefs(windowstore);
const capturestore = captureStore();
const systemstore = systemStore();
import GridContainer from "../../components/GridContainer.vue";
onMounted(() => {
  window.addEventListener("storage", (e) => {
    if (e.key == "capture") {
      capturestore.$hydrate();
    }
    if (e.key == "system") {
      systemstore.$hydrate();
    }
  });
});
const selectcapture = function () {
  let wins = [];
  monitors.value.forEach((item) => {
    let label = "capture-" + uuid();
    wins.push(
      new WebviewWindow(label, {
        x: item.position.x / item.scaleFactor,
        y: item.position.y / item.scaleFactor,
        width: item.size.width / item.scaleFactor,
        height: item.size.height / item.scaleFactor,
        shadow: false,
        decorations: false,
        transparent: true,
        alwaysOnTop: true,
        resizable: false,
        url: "/#/pages/desktop/capture",
      })
    );
  });
};
import { Command } from "@tauri-apps/plugin-shell";
import { videoDir } from "@tauri-apps/api/path";
import { remove } from "@tauri-apps/plugin-fs";
// const openvideo = function(path:string){
//     open(path)
// }

const opendir = async function () {
  let path = (await videoDir()) + "\\skydesk2";
  Command.create("start", ["/c", "start", `${path}`]).execute();
};

const delvideo = function (path: string) {
  let index = capturestore.video.findIndex((item) => {
    return item.path == path;
  });
  capturestore.video.splice(index, 1);
  remove(path);
};
</script>

<template>
  <div class="window">
    <v-card :style="{
      background: systemstore.btnbarbackground,
      backgroundSize: 'cover',
    }" class="btnbar">
      <v-btn style="margin-right: 20px" @click="selectcapture">
        <template v-slot:prepend>
          <v-icon>mdi-record-circle-outline</v-icon>
        </template>
        录屏
      </v-btn>
    </v-card>
    <v-progress-linear color="black" :indeterminate="false"></v-progress-linear>
    <div style="
        width: 100%;
        height: calc(100% - 64px);
        display: flex;
        overflow: hidden;
        padding: 10px;
      ">
      <div class="video">
        <GridContainer style="width: 100%; height: 100%; min-height: 100%" v-model="capturestore.video"
          :gridheight="240" :gridwidth="320" :padding="10">
          <template v-slot="{ item }">
            <v-card width="320" height="240" variant="elevated" elevation="10" style="width: 100%; height: 100%">
              <v-card-text style="width: 100%; height: 80%; padding: 0px">
                <video style="width: 100%; height: 100%; object-fit: cover" :src="convertFileSrc(item.path)"></video>
              </v-card-text>
              <v-card-actions style="height: 20%; padding: 0px 0px 0px 10px">
                <v-btn size="small" border="opacity-50 sm" @click="opendir">文件夹</v-btn>
                <v-btn size="small" border="opacity-50 sm" @click="delvideo(item.path)">删除</v-btn>
              </v-card-actions>
            </v-card>
          </template>
        </GridContainer>
      </div>
    </div>
  </div>
</template>

<style scoped>
.window {
  width: 100%;
  height: 100%;
}

.btnbar {
  width: 100%;
  height: 60px;
  display: flex;
  align-items: center;
  box-sizing: border-box;
  padding: 0 20px;
  filter: drop-shadow(0px 2px 5px gray);
}

.video {
  width: 100%;
  display: flex;
  justify-content: center;
  overflow-x: hidden;
  overflow-y: scroll;
}
</style>
