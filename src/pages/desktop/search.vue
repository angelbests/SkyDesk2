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
type searchResult = {
    name: string,
    path: string,
    kind: string,
    ext: string,
    dir?: string
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
onMounted(async () => {
    // 注册快捷组合键
    let res = await isRegistered("Alt+Space")
    console.log("未注册");
    if (!res) {
        register("Alt+Space", async (e) => {
            inputvalue.value = ""
            searchresult.value = []
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
})

// 监听缓存shortcut变化
window.addEventListener("storage", (e) => {
    if (e.key == "shortcut") {
        shortcutstore.$hydrate();
    }
});

// windows search 查询
const search = async function (e: any) {
    if (e.target.value == "") {
        clearTimeout(timer);
        focusindex.value = -1
        searchresult.value = []
        searchshortcut.value = []
        return
    }
    if (timer) {
        clearTimeout(timer);
    }
    timer = setTimeout(async () => {
        console.log(e.target.value)
        let show = await getCurrentWebviewWindow().isVisible();
        if (show) {
            let res: searchResult[] = await invoke('search_query', { str: e.target.value.trim() });
            res = res.filter((e) => {
                return e.path.indexOf("$Recycle.Bin") < 0
            })
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
            let shortcutres = shortcutstore.shortcutsTemp.filter(item => {
                let path = item.name.toLocaleLowerCase();
                let str = e.target.value.trim().toLocaleLowerCase()
                return path.indexOf(str) >= 0
            });
            searchshortcut.value = [...shortcutres]
        }
    }, 500);
}

// 添加窗口失焦事件
getCurrentWebviewWindow().listen("tauri://blur", () => {
    getCurrentWebviewWindow().hide()
    let dom = document.getElementById("container")
    if (dom) dom.style.height = '50px'
    focusbool.value = false
    setTimeout(() => {
        getCurrentWebviewWindow().setSize(new LogicalSize(800, 50));
    }, 300)
})
const focusbool = ref(false)

// 添加窗口聚焦事件
getCurrentWebviewWindow().listen("tauri://focus", () => {
    getCurrentWebviewWindow().setSize(new LogicalSize(800, 500));
    let dom = document.getElementById("container")
    if (dom) dom.style.height = '500px'
    focusbool.value = true
})

// 鼠标点击打开程序或文件或路径
const openfile = async function (item: searchResult) {
    console.log(item.path)
    let bool = await exists(item.path)
    if (bool) {
        openPath(item.path)
    } else {
        console.log("文件不存在")
    }
    searchresult.value = []
    inputvalue.value = ""
}

// 添加enter支持
const searchenter = async function (e: any) {
    if (focusindex.value >= 0) {
        let bool = await exists(searchresult.value[focusindex.value].path)
        if (bool) {
            openPath(searchresult.value[focusindex.value].path)
        } else {
            console.log("文件不存在")
        }
    } else {
        getCurrentWebviewWindow().hide()
        openUrl("https://cn.bing.com/search?q=" + e.target.value.trim())
    }

    inputvalue.value = ""
    searchresult.value = []
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
</script>

<template>
    <div class="container" id="container">
        <div class="search-bar">
            <v-icon style="font-size: 25px;margin-left: 25px;color:rgba(123,123,123,0.8);">mdi-magnify</v-icon>
            <input @input="search" v-model="inputvalue" type="text" @keyup.enter="searchenter" placeholder="请输入搜索内容"
                class="search-input" />
        </div>
        <div id="search-result" class="search-result" v-if="searchresult.length > 0">
            <div class="search-shortcut" :style="{ height: searchshortcut.length == 0 ? '0px' : '80px' }">
                <div v-for="item in searchshortcut" class="shortcut-container">
                    <div class="icon-div" @click="exec(item)">
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
                <div v-for="(item, index) in searchresult" :id="'search-' + index" :key="item.path" class="search-item"
                    @click="openfile(item)" :style="{ background: focusindex == index ? '#e6e9f0' : '' }">
                    <v-chip v-if="item.kind" class="search-item-kind" color="primary" variant="flat">{{ item.kind
                        }}</v-chip>
                    <div class="search-item-name">{{ item.name }}</div>
                </div>
            </div>
        </div>
        <div class="search-result" v-show="searchresult.length == 0 && focusbool">
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
            <v-tabs-window v-model="tab" style="width: 100%; height: calc(100% - 36px); padding: 10px;">
                <v-tabs-window-item v-for="item1 in shortcuts" style="width: 100%; height: 100%; min-height: 100%">
                    <GridContainer v-model="item1.shortcut" :animation="150" :gridwidth="90" :gridheight="90"
                        :group="{ name: 'shortcut', pull: 'clone' }">
                        <template v-slot="{ item }">
                            <div :style="{ height: '80px', backgroundSize: 'cover', }" class="shortcut-container">
                                <div class="icon-div" @click="exec(item)">
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
        </div>
    </div>
</template>

<style scoped>
::-webkit-scrollbar {
    display: none;
}

.container {
    width: 100vw;
    height: 99vh;
    background-image: linear-gradient(to top, #fad0c4 0%, #ffd1ff 100%);
    border-radius: 25px;
    transition: all 0.2s ease-in-out;
}

.search-bar {
    width: 100%;
    height: 50px;
    display: flex;
    align-items: center;
    background-image: linear-gradient(to top, #a18cd1 0%, #fbc2eb 100%);
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
    transition: all 0.3s ease-in-out;
}

.search-shortcut {
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
    transition: all 0.3s ease-in-out;
    background-image: linear-gradient(120deg, #fdfbfb 0%, #ebedee 100%);
}

.search-system {
    width: 100vw;
    height: 360px;
    display: flex;
    flex-direction: column;
    border-radius: 25px;
    overflow: hidden;
    overflow-y: scroll;
    transition: all 0.3s ease-in-out;
}

.search-item {
    width: 100vw;
    padding: 10px 0px 10px 30px;
    display: flex;
    flex-direction: row;
    align-items: center;
    border-radius: 25px;
    transition: all 0.3s ease-in-out;
}

.search-item-kind {
    margin-right: 10px;
}

.search-item:hover {
    background-image: linear-gradient(to top, #e6e9f0 0%, #eef1f5 100%);
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
    box-shadow: 0px 0px 15px 2px rgba(223, 223, 223, 0.2);
    transition: height 0.1s linear;
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
    transition: all 0.1s linear;
}
</style>