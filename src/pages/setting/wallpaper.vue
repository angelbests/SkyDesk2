<script setup lang="ts">
import { ref, watch, toRefs, onMounted, onActivated } from 'vue'
import { weatherStore } from '../../stores/weather'
import { systemStore } from '../../stores/system'
import { wallpaperStore } from '../../stores/wallpaper'
import { setWindowToMonitor } from '../../functions/monitor'
import { scanFiles, uuid } from '../../functions'
import { createWindow } from '../../functions/window'
import { open } from '@tauri-apps/plugin-dialog'
import { appDataDir, basename, resolve, pictureDir, resourceDir } from '@tauri-apps/api/path'
import { convertFileSrc, invoke } from '@tauri-apps/api/core'
import { copyFile, mkdir, exists, remove } from '@tauri-apps/plugin-fs'
import { availableMonitors, LogicalSize, Monitor, ProgressBarStatus } from '@tauri-apps/api/window'
import { getAllWebviewWindows, getCurrentWebviewWindow, WebviewWindow } from '@tauri-apps/api/webviewWindow'
import wallpaperfall from '../../components/WallpaperFall.vue'
import GridContainer from '../../components/GridContainer.vue'
import { getpoi } from './../../api/weather'
const wallpapers = wallpaperStore()
const waterfallshow = ref(false)
const monitors = ref<Monitor[]>([])
const systemstore = systemStore()
const video = ref()
const image = ref()
const html = ref()

onActivated(async () => {
  monitors.value = await availableMonitors()
  console.log('屏幕：', monitors.value)
})
onMounted(async () => {
  window.addEventListener('storage', (e) => {
    if (e.key == 'system') {
      systemstore.$hydrate()
    }
    if (e.key == 'wallpaper') {
      wallpapers.$hydrate()
    }
  })
  video.value = convertFileSrc((await resourceDir()) + '\\resources\\video.png')
  image.value = convertFileSrc((await resourceDir()) + '\\resources\\image.png')
  html.value = convertFileSrc((await resourceDir()) + '\\resources\\html.png')
  monitors.value = await availableMonitors()

  if (wallpapers.wallpaperConfig.length == 0) {
    for (let i = 0; i < monitors.value.length; i++) {
      wallpapers.wallpaperConfig.push({
        label: '',
        monitor: monitors.value[i].name as string,
        config: {
          action: false,
          sakura: false,
          audio: 0,
          date: false,
          datex: 0,
          datey: 0,
          datecolor: 'white',
          dateshadow: false,
          weather: false,
          weatherx: 0,
          weathery: 0,
          weatherd7: 1,
          weathershadow: false,
          music: false,
          musictype: 1,
          musicx: 0,
          musicy: 0,
          musicapp: 'QQMusic.exe',
          musicshadow: false,
          festivalcountdown: false,
          festivalcountdownx: 0,
          festivalcountdowny: 0,
          festivals: [],
          calendar: false,
          calendarx: 0,
          calendary: 0,
          calendarcolor: 'white',
          calendarshadow: false,
        },
      })
    }
  }
})

// 设置壁纸
const setmonitorwallpaper = async function (item: any, monitor: Monitor) {
  // 配置url
  let label = 'wallpaper-' + uuid()
  let url = ''
  if (item.type == 'html') {
    url = 'http://127.0.0.1:12345/' + item.path.replace((await appDataDir()) + '\\wallpapers\\html\\', '')
  } else {
    url = '/#/pages/desktop/wallpaper?type=' + item.type + '&path=' + item.path
  }
  // 创建窗口到壁纸层
  let w = await createWindow(
    label,
    {
      x: monitor.position.x,
      y: monitor.position.y,
      width: monitor.size.width,
      height: monitor.size.height,
      decorations: true,
      transparent: true,
      fullscreen: true,
      dragDropEnabled: true,
      shadow: false,
      alwaysOnBottom: true,
      skipTaskbar: true,
      url: url,
      title: label,
    },
    {
      x: monitor.position.x,
      y: monitor.position.y,
      w: monitor.size.width,
      h: monitor.size.height,
      z: 0,
      status: true,
      monitor: monitor,
    },
  )

  w?.setSize(new LogicalSize(100, 100))
  await setWindowToMonitor(label, monitor.position.x as number, monitor.position.y as number, monitor.size.width as number, monitor.size.height as number)
  let i = wallpapers.wallpaperConfig.findIndex((item) => item.monitor == monitor.name)
  if (i >= 0) {
    let all = await getAllWebviewWindows()
    all.filter((item) => {
      if (item.label == wallpapers.wallpaperConfig[i].label) {
        item.close()
      }
    })
  }
  wallpapers.wallpaperConfig[i].label = label
}

// 设置锁屏
const lockscreen = function (item: any) {
  invoke('setlockscreen', { path: item.path })
}

const addWallPaperData = ref<{
  type: 'image' | 'video' | 'html'
  title: string
  preview: string // 预览图
  filename: string // 文件名称
  path: string
}>({
  type: 'image',
  title: '',
  preview: '',
  filename: '',
  path: '',
})

const addWallpaperShow = ref(false)

// 获取预览图
const getpreview = async function () {
  if (addWallPaperData.value.type == 'image') return
  let res = await open({
    filters: [
      {
        extensions: ['jpg', 'png', 'jpeg', 'gif'],
        name: 'Image',
      },
    ],
  })
  if (res) {
    addWallPaperData.value.preview = res
  }
}

// 获取壁纸路径
const getpath = async function () {
  let extensions: string[] = []
  let name = ''
  if (addWallPaperData.value.type == 'image') {
    extensions = ['jpg', 'png', 'jpeg', 'gif']
    name = '图片'
  } else if (addWallPaperData.value.type == 'video') {
    extensions = ['mp4', 'mkv']
    name = '视频'
  } else {
    extensions = ['html']
    name = '网页'
  }
  let res = await open({
    filters: [
      {
        extensions: extensions,
        name: name,
      },
    ],
  })
  overlay.value = true
  if (res) {
    addWallPaperData.value.path = res
    addWallPaperData.value.filename = await basename(res)
  }
  if (addWallPaperData.value.type == 'image' && res) {
    addWallPaperData.value.preview = res
  } else if (addWallPaperData.value.type == 'video' && res) {
    let p_path = (await appDataDir()) + '\\wallpapers\\temp\\' + uuid() + '.png'
    res = await getvideothumb(res, p_path)
    addWallPaperData.value.preview = res ? res : ''
  }
  overlay.value = false
}

// 类型改变时，清空内容
const typechange = function (value: any) {
  addWallPaperData.value = {
    type: value,
    title: '',
    preview: '',
    filename: '',
    path: '',
  }
}

// 新增壁纸
const overlay = ref(false)
const addWallpaper = async function () {
  if (!addWallPaperData.value.path || !addWallPaperData.value.preview) {
    return
  }
  overlay.value = true
  let path = ''
  let dirid = uuid()
  let name = await basename(addWallPaperData.value.preview)
  let p_path = (await appDataDir()) + '\\wallpapers\\temp\\p_' + name
  p_path = await invoke('zipimage', {
    imgpath: addWallPaperData.value.preview,
    savepath: p_path,
  })
  addWallPaperData.value.preview = p_path
  if (addWallPaperData.value.type == 'image') {
    path = await resolve(await appDataDir(), 'wallpapers\\image', dirid)
    await mkdir(path)
    let topath = path + '\\' + uuid() + '.png'
    await copyFile(addWallPaperData.value.path, topath)
    addWallPaperData.value.path = topath
    await copyFile(addWallPaperData.value.preview, path + '\\preview.png')
    addWallPaperData.value.preview = path + '\\preview.png'
  } else if (addWallPaperData.value.type == 'video') {
    path = await resolve(await appDataDir(), 'wallpapers\\video', dirid)
    await mkdir(path)
    let topath = path + '\\' + uuid() + '.mp4'
    await copyFile(addWallPaperData.value.path, topath)
    addWallPaperData.value.path = topath
    await copyFile(addWallPaperData.value.preview, path + '\\preview.png')
    addWallPaperData.value.preview = path + '\\preview.png'
  } else {
    path = await resolve(await appDataDir(), 'wallpapers\\html', dirid)
    await mkdir(path)
    await copyFile(addWallPaperData.value.preview, path + '\\preview.png')
    addWallPaperData.value.preview = path + '\\preview.png'
    let htmldir = addWallPaperData.value.path.replace('\\' + addWallPaperData.value.filename, '')
    let files = await scanFiles(htmldir)
    for (let i = 0; i < files.length; i++) {
      let name = await basename(files[i])
      await mkdir(files[i].replace(htmldir, path).replace(name, ''), { recursive: true })
      await copyFile(files[i], files[i].replace(htmldir, path))
    }
    addWallPaperData.value.path = path + '\\' + addWallPaperData.value.filename
  }
  wallpapers.wallpaperList.push({
    ...addWallPaperData.value,
  })
  addWallPaperData.value = {
    type: 'image',
    title: '',
    preview: '',
    filename: '',
    path: '',
  }
  addWallpaperShow.value = false
  overlay.value = false
}

// 关闭所有壁纸
const closewallpaper = async function () {
  let all = await getAllWebviewWindows()
  all.filter((item) => {
    if (item.label.indexOf('wallpaper-') >= 0) {
      item.close()
    }
  })
}

// 设置图片为壁纸
const setwallpaper = async function (src: string) {
  console.log(src)
  let path = await downloadwallpaper(src)
  if (path) {
    addWallPaperData.value = {
      type: 'image',
      title: '',
      preview: path,
      filename: await basename(path),
      path: path,
    }
    addWallpaper()
  }
}

// 下载壁纸
import { download } from '@tauri-apps/plugin-upload'
import { getvideothumb } from '../../functions/videothumb'
const downloadwallpaper = async function (src: string) {
  let name = await basename(src)
  let path = (await pictureDir()) + '\\skydesk2\\' + name
  if (!(await exists(path))) {
    await download(src, path, async ({ progress, total, progressTotal, transferSpeed }) => {
      await getCurrentWebviewWindow().setProgressBar({
        progress: total == progressTotal ? 0 : Math.trunc((progressTotal / total) * 100),
        status: total == progressTotal ? ProgressBarStatus.Normal : ProgressBarStatus.None,
      })

      console.log(progress, total, progressTotal, transferSpeed)
      console.log(`name:${name},percentage :${Math.trunc((progressTotal / total) * 100)}%,speed:${transferSpeed / 1024}KB/s`)
    })
  }
  return path
}
getCurrentWebviewWindow().setProgressBar({
  progress: 0,
  status: ProgressBarStatus.None,
})
// 天气
const weatherstore = weatherStore()
const weathershow = ref(false)
let { city, query, pois, apikey, citycode } = toRefs(weatherstore)
let timer: any
watch(query, () => {
  if (query.value) {
    timer = setTimeout(async () => {
      clearTimeout(timer)
      let res: any[] = await getpoi(query.value)
      if (!res) return
      pois.value = []
      res.forEach((element: any) => {
        pois.value.push({
          title: element.country + ' ' + element.adm1 + ' ' + element.adm2 + ' ' + element.name,
          value: element.id,
          city: element.name,
        })
      })
    }, 300)
  } else {
    pois.value = []
  }
})

const selectcity = async function (e: string) {
  citycode.value = e
  let index = pois.value.findIndex((item) => {
    return item.value == e
  })
  city.value = pois.value[index].city
}

const delwallpaper = async function (index: number) {
  console.log(wallpapers.wallpaperList[index])
  let name = await basename(wallpapers.wallpaperList[index].preview)
  let path = wallpapers.wallpaperList[index].preview.replace(name, '')
  console.log(name, path)
  if (await exists(path)) {
    await remove(path, { recursive: true })
  }
  wallpapers.wallpaperList.splice(index, 1)
}

const wallpapersetting = function () {
  new WebviewWindow('wallpapersetting', {
    width: 400,
    height: 600,
    decorations: false,
    transparent: true,
    dragDropEnabled: false,
    shadow: false,
    alwaysOnTop: true,
    maximizable: false,
    resizable: false,
    skipTaskbar: true,
    center: true,
    url: '/#/pages/desktop/wallpapersetting',
  })
}
</script>

<template>
  <div class="window">
    <v-dialog max-width="500" v-model="weathershow">
      <v-list>
        <v-list-item>
          <v-text-field v-model="apikey" density="compact" hide-details="auto" placeholder="填入和风天气key" label="和风天气apikey"></v-text-field>
        </v-list-item>
        <v-list-item>
          <v-text-field v-model="query" density="compact" hide-details="auto" placeholder="查询城市名称" label="查询城市名称"></v-text-field>
        </v-list-item>
        <v-list-item>
          <v-select :items="pois" density="compact" hide-details="auto" placeholder="选择城市" label="选择城市" @update:model-value="selectcity"></v-select>
        </v-list-item>
      </v-list>
      <div style="background: white; box-sizing: border-box; padding: 10px; display: flex; justify-content: flex-end">
        <v-btn style="margin-right: 10px" @click="weathershow = false">取消</v-btn>
        <v-btn style="margin-right: 10px" @click="weathershow = false">确认</v-btn>
      </div>
    </v-dialog>
    <v-dialog v-model="waterfallshow" persistent>
      <div style="width: 100%; height: 100%; background: wheat; border-radius: 10px; overflow: hidden">
        <v-btn style="height: 30px; width: 100%" @click="waterfallshow = false">
          <v-icon>mdi-close</v-icon>
        </v-btn>
        <div style="overflow: hidden; overflow-y: scroll; width: 100%; height: calc(100vh - 50px); box-sizing: border-box; padding: 10px">
          <wallpaperfall @setwallpaper="setwallpaper" @download="downloadwallpaper"></wallpaperfall>
        </div>
      </div>
    </v-dialog>
    <v-dialog max-width="500" v-model="addWallpaperShow">
      <v-overlay :model-value="overlay" :persistent="true" class="align-center justify-center">
        <v-progress-circular color="primary" size="64" indeterminate></v-progress-circular>
      </v-overlay>
      <v-list>
        <v-list-item>
          <v-radio-group @update:model-value="typechange" v-model="addWallPaperData.type" inline density="compact" hide-details="auto">
            <v-radio style="width: 100px" label="图片" value="image"></v-radio>
            <v-radio style="width: 100px" label="视频" value="video"></v-radio>
            <v-radio style="width: 100px" label="网页" value="html"></v-radio>
          </v-radio-group>
        </v-list-item>
        <v-list-item>
          <v-text-field
            v-model="addWallPaperData.path"
            @click="getpath"
            density="compact"
            hide-details="auto"
            :readonly="true"
            :label="addWallPaperData.type == 'video' ? '视频路径' : addWallPaperData.type == 'image' ? '图片路径' : '网页路径'"></v-text-field>
        </v-list-item>
        <v-list-item>
          <v-text-field v-model="addWallPaperData.preview" @click="getpreview" density="compact" hide-details="auto" :readonly="true" label="预览图-必选"></v-text-field>
        </v-list-item>
        <v-list-item>
          <v-text-field v-model="addWallPaperData.filename" density="compact" hide-details="auto" label="文件名称-必选"></v-text-field>
        </v-list-item>
      </v-list>
      <div style="background: white; box-sizing: border-box; padding: 10px; display: flex; justify-content: flex-end">
        <v-btn style="margin-right: 10px" @click="addWallpaperShow = false">取消</v-btn>
        <v-btn style="margin-right: 10px" @click="addWallpaper">确认</v-btn>
      </div>
    </v-dialog>
    <v-card :style="{ background: systemstore.btnbarbackground, backgroundSize: 'cover' }" class="btnbar">
      <v-btn style="margin-right: 20px" @click="addWallpaperShow = true">
        <template v-slot:prepend>
          <v-icon>mdi-image-plus-outline</v-icon>
        </template>
        添加
      </v-btn>
      <v-btn style="margin-right: 20px" @click="closewallpaper">
        <template v-slot:prepend>
          <v-icon>mdi-image-remove-outline</v-icon>
        </template>
        关闭
      </v-btn>
      <v-btn style="margin-right: 20px" @click="waterfallshow = true">
        <template v-slot:prepend>
          <v-icon>mdi-image-search-outline</v-icon>
        </template>
        在线壁纸
      </v-btn>
      <v-btn style="margin-right: 20px" @click="weathershow = true">
        <template v-slot:prepend>
          <v-icon>mdi-image-filter-drama-outline</v-icon>
        </template>
        天气
      </v-btn>
      <v-btn style="margin-right: 20px" @click="wallpapersetting">
        <template v-slot:prepend>
          <v-icon>mdi-tune</v-icon>
        </template>
        设置
      </v-btn>
    </v-card>
    <v-progress-linear color="black" :indeterminate="false"></v-progress-linear>
    <div style="width: 100%; height: calc(100% - 64px); display: flex; overflow: hidden; padding: 10px; box-sizing: border-box">
      <GridContainer style="width: 100%; height: 100%; min-height: 100%" v-model="wallpapers.wallpaperList" :gridheight="240" :gridwidth="320" :padding="10">
        <template v-slot="{ item, index }">
          <v-card style="width: 100%; height: 100%; background: transparent" variant="elevated" elevation="5">
            <v-card-text style="width: 100%; height: 80%; padding: 0px">
              <img :src="convertFileSrc(item.preview)" style="width: 100%; height: 100%; object-fit: cover" />
            </v-card-text>
            <v-card-actions style="height: 20%; padding: 0px 0px 0px 10px">
              <img style="width: 25px; height: 25px; border-radius: 50%" :src="item.type == 'image' ? image : item.type == 'video' ? video : html" />
              <v-btn size="small" border="opacity-50 sm" v-for="(monitor, i) in monitors" @click="setmonitorwallpaper(item, monitor)">{{ '屏幕' + (i + 1) }}</v-btn>
              <v-btn v-if="item.type == 'image'" size="small" border="opacity-50 sm" @click="lockscreen(item)">锁屏</v-btn>
              <v-btn size="small" border="opacity-50 sm" @click="delwallpaper(index)">删除</v-btn>
            </v-card-actions>
          </v-card>
        </template>
      </GridContainer>
    </div>
  </div>
</template>

<style>
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
</style>
