<script setup lang="ts">
import { onMounted, ref, toRefs } from 'vue';
import GridContainer from '../../components/GridContainer.vue';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { LogicalSize } from '@tauri-apps/api/dpi';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { openUrl, openPath } from '@tauri-apps/plugin-opener';
import { exists } from '@tauri-apps/plugin-fs';
import { isRegistered, register } from '@tauri-apps/plugin-global-shortcut';
import { shortcutStore } from '../../stores/shortcut';
import { exec } from "../../functions/open";
import { ShortCut } from '../../types/storeType';
import { homeDir } from '@tauri-apps/api/path';
import { createWindow } from '../../functions/window';
import { uuid } from '../../functions';
import { get_pe_ico } from '../../functions/peIcon';
type searchResult = {
    name: string,
    path: string,
    kind: string,
    ext: string,
    dir: string,
    icon: string
}
const shortcutstore = shortcutStore()
const { shortcuts } = toRefs(shortcutstore)
const tab = ref(0);
const inputvalue = ref("");
const searchresult = ref<searchResult[]>([]);
const searchshortcut = ref<ShortCut[]>([]);
const focusindex = ref(-1);
let timer: string | number | NodeJS.Timeout | undefined;
let isPressed = false;

// 监听缓存shortcut变化
window.addEventListener("storage", (e) => {
    if (e.key == "shortcut") {
        shortcutstore.$hydrate();
    }
});

type SettingBottomItem = {
    path: string
    cmd: string
    label: string
};
const settingbottom = ref<SettingBottomItem[]>([]);

onMounted(async () => {
    // 注册快捷组合键
    let res = await isRegistered("Alt+Space")
    console.log("未注册");
    if (!res) {
        register("Alt+Space", async (e) => {
            if (e.state !== "Pressed") return;
            let show = await getCurrentWebviewWindow().isVisible();
            if (show) {
                getCurrentWebviewWindow().hide()
                let dom = document.getElementById("container")
                if (dom) dom.style.height = '50px'
                getCurrentWebviewWindow().setSize(new LogicalSize(800, 50));
            } else {
                getCurrentWebviewWindow().show()
                getCurrentWebviewWindow().center()
                getCurrentWebviewWindow().setFocus();
            }
        })
    }
    let path = await homeDir()
    console.log(path)
    settingbottom.value = [
        {
            path: "/icons/Folder Documents.png",
            cmd: path + "\\Documents\\",
            label: "文档"
        },
        {
            path: "/icons/Folder Downloads.png",
            cmd: path + "\\Downloads\\",
            label: "下载"
        },
        {
            path: "/icons/Folder Music.png",
            cmd: path + "\\Music\\",
            label: "音乐"
        },
        {
            path: "/icons/Folder Pictures.png",
            cmd: path + "\\Pictures\\",
            label: "图片"
        },
        {
            path: "/icons/Folder Videos.png",
            cmd: path + "\\Videos\\",
            label: "视频"
        },
        {
            path: "/icons/Folder Desktop.png",
            cmd: path + "\\Desktop\\",
            label: "桌面"
        },
        {
            path: "/icons/Folder User.png",
            cmd: path,
            label: "用户"
        },
        {
            path: "/icons/Folder Blue.png",
            cmd: '::{20D04FE0-3AEA-1069-A2D8-08002B30309D}',
            label: "资源管理器"
        },
        {
            path: "/icons/setting.png",
            cmd: "ms-settings:",
            label: "设置"
        },
        {
            path: "/icons/Control Panel.png",
            cmd: "control",
            label: "控制面板"
        },
        {
            path: "/icons/cmd.png",
            cmd: "wt",
            label: "终端"
        },
        {
            path: "/icons/calc.png",
            cmd: "calc",
            label: "计算器"
        },
        // {
        //     path: "/icons/screenshot.png",
        //     cmd: "start ms-screenclip:",
        //     label: "截图"
        // }
    ]
})

const searchstatus = ref(false)
// windows search 查询
const search = async function (e: any) {
    focusindex.value = -1
    searchresult.value = []
    searchshortcut.value = []
    if (e.target.value.trim() == "") {
        clearTimeout(timer);
        searchstatus.value = false
        return
    }
    if (timer) {
        clearTimeout(timer);
    }
    timer = setTimeout(async () => {
        console.log(e.target.value)
        searchstatus.value = true
        let value = e.target.value.trim().replace("'", "");
        let show = await getCurrentWebviewWindow().isVisible();
        if (show) {
            let shortcut: ShortCut[] = []
            shortcutstore.shortcuts.filter(e => {
                shortcut.push(...e.shortcut)
            })
            shortcut.push(...shortcutstore.shortcutsTemp)
            const map = new Map();
            shortcut = shortcut.filter((v) => !map.has(v.lnkPath) && map.set(v.lnkPath, v));
            let shortcutres = shortcut.filter(item => {
                console.log(item)
                if (item.name) {
                    let path = item.name.toLocaleLowerCase();
                    let str = value.toLocaleLowerCase()
                    return path.indexOf(str) >= 0
                }
            });
            searchshortcut.value = [...shortcutres]
            let searchinvoke: {
                str: string,
                result: searchResult[]
            } = await invoke('search_query', { str: value });
            if (searchinvoke.str != (inputvalue.value.trim().replace("'", ""))) return
            let res = searchinvoke.result;
            for (let i = 0; i < res.length; i++) {
                let path = await get_pe_ico(res[i].path, 'ico')
                console.log(path)
                res[i].icon = convertFileSrc(path)
            }
            res.forEach(e => {
                e.dir = e.path.replace(e.name, "");
            });
            let program = res.filter(e => {
                return e.kind == "程序"
            })
            let dir = res.filter(e => {
                return e.kind == "文件夹"
            })
            let image = res.filter(e => {
                return e.kind == "图片"
            })
            let video = res.filter(e => {
                return e.kind == "视频"
            })
            let document = res.filter(e => {
                return e.kind == "文档"
            })
            let url = res.filter(e => {
                return e.kind == "链接"
            })
            let music = res.filter(e => {
                return e.kind == "音乐"
            })
            searchresult.value = [...program, ...url, ...document, ...image, ...video, ...music, ...dir,]
            searchstatus.value = false
        }
    }, 500);

}

// 添加窗口失焦事件
getCurrentWebviewWindow().listen("tauri://blur", () => {
    getCurrentWebviewWindow().hide()
    focusindex.value = -1
    searchresult.value = []
    searchshortcut.value = []
    inputvalue.value = ""
    let dom = document.getElementById("container")
    if (dom) dom.style.height = '50px'
    getCurrentWebviewWindow().setSize(new LogicalSize(800, 50));
})

// 添加窗口聚焦事件
getCurrentWebviewWindow().listen("tauri://focus", () => {
    document.getElementById("input")?.focus()
    getCurrentWebviewWindow().setSize(new LogicalSize(800, 500));
    let dom = document.getElementById("container")
    if (dom) dom.style.height = '500px'
})

// 鼠标点击打开程序或文件或路径
const openfile = async function (item: searchResult) {
    getCurrentWebviewWindow().hide();
    let bool = await exists(item.path)
    if (bool) {
        openPath(item.path)
    } else {
        console.log("文件不存在")
    }
}

// 添加enter支持
const searchenter = async function (e: any) {
    getCurrentWebviewWindow().hide()
    if (focusindex.value >= 0) {
        let bool = await exists(searchresult.value[focusindex.value].path)
        if (bool) {
            openPath(searchresult.value[focusindex.value].path)
        } else {
            console.log("文件不存在")
        }
    } else {
        openUrl("https://cn.bing.com/search?q=" + e.target.value.trim())
    }
}

// 添加键盘上下选择
window.addEventListener("keydown", (e) => {
    if (isPressed) return
    let dom = document.getElementById("search-system")
    if (!dom) return
    if (e.key == 'ArrowDown') {
        isPressed = true;
        if (focusindex.value == -1) {
            focusindex.value = 0
            return
        }
        if (focusindex.value < searchresult.value.length - 1) {
            focusindex.value += 1;
            let domitem = document.getElementById("search-" + focusindex.value)
            if (!domitem) return
            let height = domitem.offsetHeight
            dom.scrollBy({
                "behavior": "smooth",
                "top": height
            })
        } else {
            focusindex.value = 0
            dom.scrollTo({
                "behavior": "smooth",
                "top": 0
            })
        }


    } else if (e.key == 'ArrowUp') {
        if (focusindex.value == -1) {
            focusindex.value = 0
            return
        }
        if (focusindex.value == 0) {
            focusindex.value = searchresult.value.length - 1;
            dom.scrollTo({
                "behavior": "smooth",
                "top": dom.scrollHeight
            })
        } else {
            focusindex.value -= 1;
            let domitem = document.getElementById("search-" + focusindex.value)
            if (!domitem) return
            let height = domitem.offsetHeight
            dom.scrollBy({
                "behavior": "smooth",
                "top": -height
            })
        }

    }
})

window.addEventListener("keyup", () => {
    isPressed = false;
});

const screen = async function () {
    await getCurrentWebviewWindow().hide();
    invoke('screen')
}

const openshortcut = function (item: any) {
    getCurrentWebviewWindow().hide()
    exec(item);
}
const contextmenuShow = ref(false)
const createnote = async function () {
    let label = "note-" + uuid();
    let w = await createWindow(label, {
        x: 200,
        y: 200,
        width: 330,
        height: 330,
        minWidth: 330,
        minHeight: 100,
        shadow: false,
        decorations: false,
        transparent: true,
        skipTaskbar: true,
        url: "/#/pages/desktop/note",
        title: "note"
    });
    w?.center()
};

const rightclick = function (e: MouseEvent, item: searchResult) {
    contextmenuShow.value = false
    console.log(e, item)
    let dom = document.getElementById("contextmenu")
    if (!dom) return;
    dom.style.left = e.clientX + 'px'
    dom.style.top = e.clientY - 20 + 'px'
    console.log(e.offsetX, e.offsetY)
    contextmenuShow.value = false
}

const opendir = function (item: searchResult) {
    console.log(item)
    openPath(item.dir)
}

import { writeText } from "@tauri-apps/plugin-clipboard-manager"
const copy = async function (item: searchResult) {
    let path = item.path.replace("file:", "")
    if (item.kind == '文件夹') {
        await writeText(path)
    } else {
        await invoke("copyfile", { path })
    }
    getCurrentWebviewWindow().hide()
}
</script>

<template>
    <div class="container" id="container">
        <div id="contextmenu" style="position: absolute;z-index: 100;width: 100px;height: 100px;width: 120px;height:90px;
        padding: 10px;display: flex;flex-direction: column;align-items: center;justify-content: space-evenly;"
            v-show="contextmenuShow">
            <v-btn size="small" width="100px">打开文件夹</v-btn>
            <v-btn size='small' width="100px">复制路径</v-btn>
        </div>
        <div class="search-bar">
            <v-icon style="font-size: 26px;margin-left: 25px;color:rgba(123,123,123,0.8);">mdi-magnify</v-icon>
            <input id="input" autocomplete="off" @input="search" v-model="inputvalue" type="text"
                @keyup.enter="searchenter" placeholder="请输入搜索内容" class="search-input" />
        </div>

        <div id="search-result" class="search-result" v-if="inputvalue">
            <div class="search-shortcut" :style="{ height: searchshortcut.length == 0 ? '0px' : '80px' }">
                <div tabindex="0" @keyup.enter="openshortcut(item)" v-for="item in searchshortcut"
                    class="shortcut-container">
                    <div class="icon-div" @click="openshortcut(item)">
                        <img class="icon"
                            :src="item.icoPath == '' ? '/icons/program.png' : convertFileSrc(item.icoPath)" />
                    </div>
                    <div
                        style="font-size: 10px;width: 60px;height: 30px;filter: drop-shadow(0px 5px 5px gray);text-wrap: balance;text-align: center; text-overflow: clip;overflow: hidden;">
                        {{ item.name }}
                    </div>
                </div>
            </div>
            <div class="search-system" id="search-system"
                :style="{ height: searchshortcut.length == 0 ? '440px' : '360px' }">

                <div v-if="searchresult.length > 0" tabindex="0" v-for="(item, index) in searchresult"
                    :id="'search-' + index" :key="item.path" class="search-item"
                    :style="{ background: focusindex == index ? '#e6e9f0' : '' }" @keyup.enter="openfile(item)"
                    @contextmenu="rightclick($event, item)">
                    <v-chip @click="openfile(item)" v-if="item.kind" class="search-item-kind">
                        <template v-slot:default>
                            <img class="search-item-kind-img" :src="item.icon">
                            {{ item.kind }}
                        </template>
                    </v-chip>
                    <div @click="openfile(item)" class="search-item-name">{{ item.name }}</div>

                    <v-btn class="search-item-btn" size="small" @click="opendir(item)">
                        <template v-slot:prepend>
                            <img style="width: 20px;" src="/icons/Folder Live - Back.png">
                        </template>
                        文件夹
                    </v-btn>
                    <v-btn class="search-item-btn" size="small" @click="copy(item)">
                        <template v-slot:prepend>
                            <img style="width: 20px;" src="/icons/copy.png">
                        </template>
                        复制
                    </v-btn>
                    <!-- <v-btn style="margin-left: 10px;" size="x-small">文件夹</v-btn> -->
                </div>
                <!-- 搜索提示与错误 -->
                <div v-else style="display: flex;justify-content: center;align-items: center;height: 100%;">
                    <div v-if="searchstatus"
                        style="display:flex;flex-direction: column;justify-content: center;align-items: center;">
                        <v-progress-circular :size="50" color="primary" indeterminate></v-progress-circular>
                    </div>
                    <div v-else style="display: flex;justify-content: center;align-items: center;">
                        <img src="/icons/search.png" draggable="false">
                    </div>
                </div>
            </div>
        </div>
        <div class="search-result" v-else>
            <v-tabs id="shortcuttab" density="compact" v-model="tab" center-active style="
            height: 36px;
            width: 100%;
            background: white;
            ;
          " hide-slider show-arrows>
                <v-tab v-for="item in shortcuts" :value="item.index">
                    <div style="display: flex; justify-content: center">
                        <div style="width: calc(100% - 20px); margin-right: 10px">
                            {{ item.title }}
                        </div>
                    </div>
                </v-tab>
            </v-tabs>
            <v-tabs-window v-model="tab" style="width: 100%; height: calc(100% - 86px);">
                <v-tabs-window-item v-for="item1 in shortcuts" style="width: 100%;height: 100%;">
                    <GridContainer v-model="item1.shortcut" :animation="150" :gridwidth="90" :gridheight="90"
                        :group="{ name: 'shortcut', pull: 'clone' }">
                        <template v-slot="{ item }">
                            <div tabindex="0" @keyup.enter="openshortcut(item)"
                                :style="{ height: '80px', backgroundSize: 'cover', }" class="shortcut-container">
                                <div class="icon-div" @click="openshortcut(item)">
                                    <img class="icon"
                                        :src="item.icoPath == '' ? '/icons/program.png' : convertFileSrc(item.icoPath)" />
                                </div>
                                <div
                                    style="font-size: 10px;width: 60px;height: 30px;filter: drop-shadow(0px 5px 5px gray);text-wrap: balance;text-align: center; text-overflow: clip;overflow: hidden;">
                                    {{ item.name }}
                                </div>
                            </div>
                        </template>
                    </GridContainer>
                </v-tabs-window-item>
            </v-tabs-window>
            <div class="settingbottom">
                <div tabindex="0" v-for="item in settingbottom" class="settingbottom-item" @click="openPath(item.cmd)"
                    @keyup.enter="openPath(item.cmd)">
                    <img :src="item.path" class="settingbottom-icon" />
                    <v-tooltip activator="parent" location="top">
                        {{ item.label }}
                    </v-tooltip>
                </div>
                <div tabindex="0" class="settingbottom-item" @click="screen" @keyup.enter="screen">
                    <img src="/icons/screenshot.png" class="settingbottom-icon" />
                    <v-tooltip activator="parent" location="top">
                        截图
                    </v-tooltip>
                </div>
                <div tabindex="0" class="settingbottom-item" @click="createnote" @keyup.enter="createnote">
                    <img src="/icons/note.png" class="settingbottom-icon" />
                    <v-tooltip activator="parent" location="top">
                        便签
                    </v-tooltip>
                </div>
            </div>
        </div>
    </div>
</template>

<style>
div:focus-visible {
    outline: none;
    background-image: linear-gradient(to top, #e6e9f0 0%, #eef1f5 100%);
    border-radius: 3px;
    border: none;
}

::-webkit-scrollbar {
    display: none;
}

.container {
    width: 100vw;
    height: 99vh;
    border-radius: 25px;
    transition: all 0.2s linear;
}

.search-bar {
    width: 100%;
    height: 50px;
    display: flex;
    align-items: center;
    background-image: linear-gradient(to top, #fad0c4 0%, #ffd1ff 100%);
    border-radius: 25px;
    margin-bottom: 10px;
}

.search-input {
    padding: 3px 10px;
    margin: 0;
    width: 100%;
    height: 100%;
    outline: none;
}

.search-result {
    width: 100vw;
    height: 440px;
    display: flex;
    flex-direction: column;
    border-radius: 25px;
    overflow: hidden;
    overflow-y: scroll;
    transition: all 0.2s linear;
    background-image: linear-gradient(to top, #fad0c4 0%, #ffd1ff 100%);
}

.search-shortcut {
    background-image: linear-gradient(to top, #fad0c4 0%, #ffd1ff 100%);
    box-sizing: border-box;
    padding: 0px 30px;
    width: 100%;
    height: 80px;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    flex-wrap: wrap;
    overflow: hidden;
    overflow-y: scroll;
    transition: all 0.2s linear;
}

.search-system {
    width: 100vw;
    height: 360px;
    display: flex;
    flex-direction: column;
    border-bottom-left-radius: 25px;
    border-bottom-right-radius: 25px;
    overflow: hidden;
    overflow-y: scroll;
    transition: all 0.2s linear;
}

.search-item {
    width: 100vw;
    padding: 10px 0px 10px 30px;
    display: flex;
    flex-direction: row;
    align-items: center;
    transition: all 0.2s linear;
}

.search-item:hover {
    background-image: linear-gradient(to top, #e6e9f0 0%, #eef1f5 100%);

}

.search-item-name {
    font-size: 14px;
    width: 450px;
    height: 100%;
    display: flex;
    align-items: center;
    overflow: hidden;
}

.search-item-kind {
    margin-right: 10px;
    font-size: 12px;
    width: 100px;
    align-items: center;
    justify-content: center;
}

.search-item-btn {
    background: transparent;
    margin-right: 10px;
}

.search-item-kind-img {
    width: 23px;
    margin-right: 10px;
}

.icon:hover {
    width: 40px;
    height: 40px;
}

.shortcut-container {
    width: 80px;
    height: 80px;
    margin: 5px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    border-radius: 10px;
    transition: height 0.2s linear;
    background-size: cover;
}

.icon-div {
    width: 40px;
    height: 40px;
    display: flex;
    justify-content: center;
    filter: drop-shadow(0px 5px 5px gray);
}

.icon {
    width: 35px;
    height: 35px;
    border-radius: 5px;
    transition: all 0.2s linear;
}

.settingbottom {
    width: 100%;
    height: 50px;
    background: rgba(123, 123, 123, 0.1);
    display: flex;
    align-items: center;
    justify-content: center;
    box-sizing: border-box;
    padding: 0px 20px;
}

.settingbottom-item {
    width: 55px;
    display: flex;
    justify-content: center;
    align-items: center;
}

.settingbottom-icon {

    width: 40px;
    transition: all 0.2s linear;
    filter: drop-shadow(0px 5px 5px gray);
}

.settingbottom-icon:hover {
    width: 50px;
}

.v-window__container {
    height: 100% !important;
}
</style>