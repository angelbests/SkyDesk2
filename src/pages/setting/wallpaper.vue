<script setup lang="ts">
import { ref, onMounted, computed, watch, toRefs } from "vue";
import { wallpaperStore, weatherStore, windowStore } from "../../stores/window";
import { setWindowToMonitor } from "../../functions/monitor";
import { scanFiles, uuid } from "../../functions";
import { createWindow } from "../../functions/window";
import { open } from '@tauri-apps/plugin-dialog';
import { appDataDir, basename, resolve, pictureDir } from '@tauri-apps/api/path'
import { convertFileSrc } from "@tauri-apps/api/core";
import { copyFile, mkdir, exists, remove } from "@tauri-apps/plugin-fs";
import { LogicalSize, Monitor } from "@tauri-apps/api/window";
import { getAllWebviewWindows } from "@tauri-apps/api/webviewWindow";
import { downloadload } from "../../api/download";
import wallpaperfall from "../../components/wallpaperfall.vue";
import  { getpoi } from "./../../api/weather"

const wallpaperWidth = ref(0);
const wallpaperref = ref<HTMLDivElement>();
const wallpapers = wallpaperStore()
const waterfallshow = ref(false)

onMounted(() => {
    updateElementHeight()
    window.addEventListener('resize', updateElementHeight);

})

const updateElementHeight = function () {
    if (wallpaperref.value) {
        wallpaperWidth.value = wallpaperref.value.offsetWidth
    }
}

const wallpaperListHeight = computed(() => {
    return Math.ceil((wallpapers.wallpaperList.length / Math.trunc(wallpaperWidth.value / 420))) * 320 + 'px';
})

const windowstore = windowStore()
const textwallpaper = async function (item: any, monitor: Monitor) {
    let index = wallpapers.config.findIndex(item => {
        return monitor.name == item.monitor.name
    })
    if (index >= 0) {
        (await getAllWebviewWindows()).filter(async item => {
            if (item.label == wallpapers.config[index].label) {
                await item.close()
                wallpapers.config.splice(index, 1)
            }
        })
    }
    let label = "wallpaper-" + uuid()
    let url = ""
    if (item.type == "html") {
        url = "http://127.0.0.1:12345/" + item.path.replace(await appDataDir() + "\\wallpapers\\html\\", "")
    } else {
        url = "/#/pages/desktop/wallpaper?type=" + item.type + "&path=" + item.path
    }
    let w = await createWindow(label, {
        x: 999999999,
        y: 999999999,
        width: 1000,
        height: 1000,
        decorations: false,
        transparent: true,
        fullscreen: false,
        dragDropEnabled: true,
        shadow: false,
        alwaysOnBottom: true,
        skipTaskbar: true,
        url: url,
    },{
        x:monitor.position.x,
        y:monitor.position.y,
        w:monitor.size.width,
        h:monitor.size.height,
        z:0,
        status:true
    })
    w?.setSize(new LogicalSize(0, 0));
    wallpapers.config.push({
        label: label,
        monitor: monitor,
        type: item.type,
        url: url,
    })
    await setWindowToMonitor(
        label,
        monitor.position.x as number ,
        monitor.position.y as number -5,
        monitor.size.width as number,
        monitor.size.height as number + 10
    )
}

const addWallPaperData = ref<{
    "type": "image" | "video" | "html"
    "title": string
    "preview": string // 预览图
    "filename": string // 文件名称
    "path": string
}>({
    "type": "image",
    "title": "",
    "preview": "",
    "filename": "",
    "path": ""
})

const addWallpaperShow = ref(false)

const getpreview = async function () {
    if (addWallPaperData.value.type == "image") return
    let res = await open({
        "filters": [
            {
                "extensions": ['jpg', 'png', 'jpeg', 'gif'],
                name: "Image"
            }
        ]
    })
    if (res) {
        addWallPaperData.value.preview = res
    }
}

const getpath = async function () {
    let extensions: string[] = []
    let name = ""
    if (addWallPaperData.value.type == "image") {
        extensions = ['jpg', 'png', 'jpeg', 'gif']
        name = "图片"
    } else if (addWallPaperData.value.type == "video") {
        extensions = ['mp4', 'mkv']
        name = "视频"
    } else {
        extensions = ['html']
        name = "网页"
    }
    let res = await open({
        "filters": [
            {
                "extensions": extensions,
                name: name
            }
        ]
    })
    if (res) {
        addWallPaperData.value.path = res
        addWallPaperData.value.filename = await basename(res)
    }
    if (addWallPaperData.value.type == "image" && res) {
        addWallPaperData.value.preview = res
    }
}

const typechange = function (value: any) {
    addWallPaperData.value = {
        "type": value,
        "title": "",
        "preview": "",
        "filename": "",
        "path": ""
    }
}

const addWallpaper = async function () {
    let path = ""
    let dirid = uuid()
    if (addWallPaperData.value.type == "image") {
        path = await resolve(await appDataDir(), "wallpapers\\image", dirid)
        await mkdir(path)
        console.log(path)
        await copyFile(addWallPaperData.value.path, path + "\\" + addWallPaperData.value.filename)
        addWallPaperData.value.path = path + "\\" + addWallPaperData.value.filename
        await copyFile(addWallPaperData.value.preview, path + "\\preview.png")
        addWallPaperData.value.preview = path + "\\preview.png"
    } else if (addWallPaperData.value.type == "video") {
        path = await resolve(await appDataDir(), "wallpapers\\video", dirid)
        await mkdir(path)
        await copyFile(addWallPaperData.value.path, path + "\\" + addWallPaperData.value.filename)
        addWallPaperData.value.path = path + "\\" + addWallPaperData.value.filename
        await copyFile(addWallPaperData.value.preview, path + "\\preview.png")
        addWallPaperData.value.preview = path + "\\preview.png"
    } else {
        path = await resolve(await appDataDir(), "wallpapers\\html", dirid)
        await mkdir(path)
        await copyFile(addWallPaperData.value.preview, path + "\\preview.png")
        addWallPaperData.value.preview = path + "\\preview.png"
        let htmldir = addWallPaperData.value.path.replace("\\" + addWallPaperData.value.filename, "");
        let files = await scanFiles(htmldir)
        for (let i = 0; i < files.length; i++) {
            let name = await basename(files[i])
            await mkdir((files[i].replace(htmldir, path)).replace(name, ""), { "recursive": true })
            await copyFile(files[i], files[i].replace(htmldir, path))
        }
        addWallPaperData.value.path = path + "\\" + addWallPaperData.value.filename
    }
    wallpapers.wallpaperList.push({
        ...addWallPaperData.value
    })
    addWallPaperData.value = {
        "type": "image",
        "title": "",
        "preview": "",
        "filename": "",
        "path": ""
    }
    addWallpaperShow.value = false
}

const closewallpaper = async function () {
    let all = await getAllWebviewWindows()
    wallpapers.config = []
    all.filter(item => {
        if (item.label.indexOf("wallpaper-") >= 0) {
            item.close()
        }
    })

    windowstore.windows = windowstore.windows.filter(item => {
        return item.label.indexOf('wallpaper') < 0
    })
}

const setwallpaper =async function(src:string){
    console.log(src)
    let path =await downloadwallpaper(src)
    if(path){
        addWallPaperData.value = {
            "type": "image",
            "title": "",
            "preview": path,
            "filename": await basename(path),
            "path": path
        }
        addWallpaper()
    }
}

const downloadwallpaper =async (src:string):Promise<string>=>{
  let name = await basename(src);
  let path = await pictureDir() + "\\skydesk2\\" + name;
  if(!await exists(path)){
    let res = await downloadload(path,src);
    if(res){
      return path
    }
  }
  return "";
}

const weatherstore = weatherStore()
const weathershow = ref(false)
let { city,query,pois,apikey,citycode }  = toRefs(weatherstore)
let timer:any;
watch(query,()=>{
    if(query.value){
        timer =setTimeout(async() => {
            clearTimeout(timer)
            let res:any[] = await getpoi(query.value)
            if(!res) return;
            pois.value = []
            res.forEach((element:any) => {
                pois.value.push({
                    title:element.country + " " + element.adm1 + " " + element.adm2 + " " +element.name,
                    value:element.id,
                    city:element.name
                })
            });
        }, 300);
    }else{
        pois.value = []
    }
})

const selectcity =async function(e:string){
    citycode.value = e
    let index = pois.value.findIndex(item=>{
        return item.value == e
    })
    city.value = pois.value[index].city
}

const delwallpaper = async function(index:number){
    console.log(wallpapers.wallpaperList[index])
    let name = await basename(wallpapers.wallpaperList[index].preview)
    let path = wallpapers.wallpaperList[index].path.replace(name,"")
    if(await exists(path)){
        await remove(path,{recursive:true})
    }
    wallpapers.wallpaperList.splice(index, 1)
}

</script>

<template>
    <div class="window">
        <v-dialog max-width="500" v-model="weathershow">
            <v-list>
                <v-list-item>
                    <v-text-field v-model="apikey" density="compact" hide-details="auto" placeholder="填入和风天气key" label="和风天气apikey"</v-text-field>
                </v-list-item>
                <v-list-item>
                    <v-text-field v-model="query" density="compact" hide-details="auto" placeholder="查询城市名称" label="查询城市名称"></v-text-field>
                </v-list-item>
                <v-list-item>
                    <v-select :items="pois" density="compact" hide-details="auto" placeholder="选择城市" label="选择城市" @update:model-value="selectcity"></v-select> 
                </v-list-item>
            </v-list>
            <div
                style="background: white;box-sizing: border-box;padding: 10px;display: flex;justify-content: flex-end;">
                <v-btn style="margin-right: 10px;" @click="weathershow = false">取消</v-btn>
                <v-btn style="margin-right: 10px;" @click="weathershow = false">确认</v-btn>
            </div>
        </v-dialog>
        <v-dialog v-model="waterfallshow" persistent>
            <div style="width: 100%;height: 100vh;background: wheat;border-radius: 10px;">
                <v-btn style="height: 30px;width: 100%;" @click="waterfallshow = false">
                    <v-icon>mdi-close</v-icon>
                </v-btn>
                <div style="overflow: hidden;overflow-y: scroll;width: 100%;height: calc(100vh - 50px);box-sizing: border-box;padding: 10px;">
                    <wallpaperfall @setwallpaper="setwallpaper" @download="downloadwallpaper"></wallpaperfall>
                </div>
            </div>
        </v-dialog>
        <v-dialog max-width="500" v-model="addWallpaperShow">
            <v-list>
                <v-list-item>
                    <v-radio-group @update:model-value="typechange" v-model="addWallPaperData.type" inline
                        density="compact" hide-details="auto">
                        <v-radio style="width: 100px;" label="图片" value="image"></v-radio>
                        <v-radio style="width: 100px;" label="视频" value="video"></v-radio>
                        <v-radio style="width: 100px;" label="网页" value="html"></v-radio>
                    </v-radio-group>
                </v-list-item>
                <v-list-item>
                    <v-text-field v-model="addWallPaperData.title" density="compact" hide-details="auto"
                        label="标题"></v-text-field>
                </v-list-item>
                <v-list-item>
                    <v-text-field v-model="addWallPaperData.path" @click="getpath" density="compact" hide-details="auto"
                        :readonly="true"
                        :label="addWallPaperData.type == 'video' ? '视频路径' : addWallPaperData.type == 'image' ? '图片路径' : '网页路径'"></v-text-field>
                </v-list-item>
                <v-list-item>
                    <v-text-field v-model="addWallPaperData.preview" @click="getpreview" density="compact"
                        hide-details="auto" :readonly="true" label="预览图"></v-text-field>
                </v-list-item>
                <v-list-item>
                    <v-text-field v-model="addWallPaperData.filename" density="compact" hide-details="auto"
                        label="文件名称"></v-text-field>
                </v-list-item>
            </v-list>
            <div
                style="background: white;box-sizing: border-box;padding: 10px;display: flex;justify-content: flex-end;">
                <v-btn style="margin-right: 10px;" @click="addWallpaperShow = false">取消</v-btn>
                <v-btn style="margin-right: 10px;" @click="addWallpaper">确认</v-btn>
            </div>
        </v-dialog>
        <v-card
            style="width: 100%;height: 60px;display: flex;align-items: center;box-sizing: border-box;padding: 0 20px;filter:drop-shadow(0px 2px 5px gray)">
            <v-btn style="margin-right: 20px;" @click="addWallpaperShow = true">
                <template v-slot:prepend>
                    <v-icon>mdi-wallpaper</v-icon>
                </template>
                添加壁纸
            </v-btn>
            <v-btn style="margin-right: 20px;" @click="closewallpaper">
                <template v-slot:prepend>
                    <v-icon>mdi-close</v-icon>
                </template>
                关闭壁纸
            </v-btn>
            <v-btn style="margin-right: 20px;" @click="waterfallshow = true">
                <template v-slot:prepend>
                    <v-icon>mdi-image</v-icon>
                </template>
                在线壁纸
            </v-btn>
            <v-btn style="margin-right: 20px;" @click="weathershow = true">
                <template v-slot:prepend>
                    <v-icon>mdi-image</v-icon>
                </template>
                设置天气
            </v-btn>
        </v-card>
        <v-progress-linear color="black" :indeterminate="false"></v-progress-linear>
        <div style="width: 100%;height: calc(100% - 60px);display: flex;overflow: hidden;background: white;">
            <div class="wallpaper" id="wallpaper" ref="wallpaperref">
                <div class="wallpaper-list" :style="{ height: wallpaperListHeight, minHeight: '100%' }">
                    <v-card prepend-icon="" width="400" height="305" variant="elevated" elevation="10"
                        v-for="(item, index) in wallpapers.wallpaperList">
                        <v-card-text style="position: relative;">
                            <v-img :src="convertFileSrc(item.preview)" cover :height="220"></v-img>
                            <div style="width: 100%;position: absolute;left: 15px;top: 240px;z-index: 50;color: gray;">
                                {{ item.title }}
                            </div>
                        </v-card-text>
                        <v-card-actions>
                            <v-btn :disabled="true">{{ item.type == 'video' ? '视频' : item.type == 'image' ? '图片' : '网页' }}</v-btn>
                            <v-btn v-for="(monitor, i) in windowstore.monitors" @click="textwallpaper(item, monitor)">{{
                                "屏幕"+(i+1) }}</v-btn>
                            <v-btn @click="delwallpaper(index)">删除</v-btn>
                        </v-card-actions>
                    </v-card>
                </div>
            </div>
        </div>

    </div>

</template>

<style>
.window {
    width: 100%;
    height: 100%;
}

.wallpaper {
    width: 100%;
    background: white;
    display: flex;
    justify-content: center;
    overflow-x: hidden;
    overflow-y: scroll;
}

.wallpaper-list {
    width: 100%;
    height: auto;
    display: grid;
    grid-template-columns: repeat(auto-fill, 420px);
    grid-template-rows: repeat(auto-fit, 320px);
    grid-auto-flow: row;
    justify-items: center;
    align-items: center;
    justify-content: center;

}

</style>