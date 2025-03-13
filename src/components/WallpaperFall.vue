<script lang="ts" setup>
import buttonbox from "./ButtonBox.vue";
import { getwallpaper } from "../api/api";
import waterfall from "./WaterFall.vue";
import { onMounted, ref, watch } from "vue";
const wallpaperswaterfall = ref<
  {
    src: string;
    ratio: number;
    width: number;
    height: number;
    thumbs: string;
  }[]
>([]);

onMounted(() => {
  getwallpapers(true);
});

const getwallpapers = async function (reset: boolean) {
  if (reset) {
    wallpaperswaterfall.value.splice(0);
  } else {
    selectvalue.value.page = selectvalue.value.page + 1;
  }
  let res = await getwallpaper({ ...selectvalue.value });
  let arr = [];
  if (res.data != undefined && res.data.length > 0) {
    for (let i = 0; i < res.data.length; i++) {
      arr.push({
        src: res.data[i].path,
        width: res.data[i].dimension_x,
        height: res.data[i].dimension_y,
        ratio: res.data[i].ratio,
        thumbs: res.data[i].thumbs.original,
      });
    }
  }
  if (reset) {
    wallpaperswaterfall.value.push(...arr);
  } else {
    wallpaperswaterfall.value.push(...arr);
  }
};
import RightBar from "./RightBar.vue";

//#region

const selectvalue = ref({
  categories: "",
  sorting: "",
  order: "",
  ratio: "",
  resolutions: "",
  keyword: "",
  page: 0,
});

// 大类
const categories = ref([
  { text: "动漫", value: "010" },
  { text: "常规", value: "100" },
  { text: "人物", value: "001" },
  { text: "空", value: "" },
]);

// 排序
const sorting = ref([
  { text: "相关性", value: "relevance" },
  { text: "随机", value: "random" },
  { text: "浏览量", value: "views" },
  { text: "下载量", value: "favorites" },
  { text: "流行", value: "toplist" },
  { text: "日期", value: "date added" },
  { text: "热度", value: "hot" },
  { text: "空", value: "" },
]);

// 正序 倒序
const order = ref([
  { text: "正序", value: "asc" },
  { text: "倒序", value: "desc" },
  { text: "空", value: "" },
]);

// 分辨率
const ratio = ref([
  {
    text: "5:4",
    value: "5:4",
    resolutions: [
      { text: "1280x1024", value: "1280x1024" },
      { text: "1600x1280", value: "1600x1280" },
      { text: "1920x1536", value: "1920x1536" },
      { text: "2560x2048", value: "2560x2048" },
      { text: "3840x3072", value: "3840x3072" },
      { text: "空", value: "" },
    ],
  },
  {
    text: "4:3",
    value: "4:3",
    resolutions: [
      { text: "1280x960", value: "1280x960" },
      { text: "1600x1200", value: "1600x1200" },
      { text: "1920x1440", value: "1920x1440" },
      { text: "2560x1920", value: "2560x1920" },
      { text: "3840x2880", value: "3840x2880" },
      { text: "空", value: "" },
    ],
  },
  {
    text: "16:10",
    value: "16:10",
    resolutions: [
      { text: "1280x800", value: "1280x800" },
      { text: "1600x1000", value: "1600x1000" },
      { text: "1920x1200", value: "1920x1200" },
      { text: "2560x1600", value: "2560x1600" },
      { text: "3840x2400", value: "3840x2400" },
      { text: "空", value: "" },
    ],
  },
  {
    text: "16:9",
    value: "16:9",
    resolutions: [
      { text: "1280x720", value: "1280x720" },
      { text: "1600x900", value: "1600x900" },
      { text: "1920x1080", value: "1920x1080" },
      { text: "2560x1440", value: "2560x1440" },
      { text: "3840x2160", value: "3840x2160" },
      { text: "空", value: "" },
    ],
  },
  {
    text: "Ultrawide",
    value: "Ultrawide",
    resolutions: [
      { text: "2560x1080", value: "2560x1080" },
      { text: "3840x1440", value: "3840x1440" },
      { text: "3840x1600", value: "3840x1600" },
      { text: "空", value: "" },
    ],
  },
  {
    text: "空",
    value: "",
    resolutions: [{ text: "空", value: "" }],
  },
]);

const resolutions = ref<
  {
    text: string;
    value: string;
  }[]
>([{ text: "空", value: "" }]);

watch(selectvalue.value, () => {
  let arr = ratio.value.filter((item) => {
    return item.value == selectvalue.value.ratio;
  });
  resolutions.value = arr[0].resolutions;
});

//#endregion

const mousename = ref("");
const emit = defineEmits<{
  setwallpaper: [src: string];
  download: [src: string];
}>();

const setwallpaper = function (src: string) {
  emit("setwallpaper", src);
};

const download = function (src: string) {
  emit("download", src);
};
</script>

<template>
  <div class="container" data-tauri-drag-region>
    <waterfall :col="4" :gap="5" :images="wallpaperswaterfall" @scrollload="getwallpapers(false)">
      <template v-slot="{ image }">
        <div style="position: relative; width: 100%; height: 100%" @mouseover="mousename = image.src">
          <img :src="image.thumbs" style="width: 100%; height: 100%" />
          <div v-if="image.src == mousename" style="
              position: absolute;
              left: 0px;
              top: 0px;
              z-index: 100;
              display: flex;
              justify-content: space-evenly;
              align-items: center;
              width: 100%;
              height: 100%;
            ">
            <v-btn icon size="small" @click="setwallpaper(image.src)">
              <v-icon>mdi-image</v-icon>
            </v-btn>
            <v-btn icon size="small" @click="download(image.src)">
              <v-icon>mdi-download-outline</v-icon>
            </v-btn>
          </div>
        </div>
      </template>
    </waterfall>
    <RightBar>
      <div class="select">
        <buttonbox :boxs="categories" v-model="selectvalue.categories"></buttonbox>
        <buttonbox :boxs="sorting" v-model="selectvalue.sorting"></buttonbox>
        <buttonbox :boxs="order" v-model="selectvalue.order"></buttonbox>
        <buttonbox :boxs="ratio" v-model="selectvalue.ratio"></buttonbox>
        <buttonbox :boxs="resolutions" v-model="selectvalue.resolutions"></buttonbox>
        <div style="width: 100%; height: 40px; display: flex; flex-direction: row">
          <div style="width: 300px; margin-right: 20px">
            <v-text-field v-model="selectvalue.keyword" density="compact" hide-details="auto"
              placeholder="关键字"></v-text-field>
          </div>
          <div style="width: 200px">
            <v-number-input v-model="selectvalue.page" :min="1" :max="999999" control-variant="split" density="compact"
              hide-details="auto" placeholder="页"></v-number-input>
          </div>
        </div>
        <v-btn @click="getwallpapers(true)">查询</v-btn>
      </div>
    </RightBar>
  </div>
</template>

<style scoped>
.container {
  width: 100%;
  height: 100%;
  position: relative;
}

.select {
  box-sizing: border-box;
  padding: 20px;
  width: 100%;
  height: 60%;
  position: absolute;
  z-index: 100;
  left: 0;
  top: 0;
  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  background: wheat;
}
</style>
