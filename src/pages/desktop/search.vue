<script setup lang="ts">
import { onMounted, ref, toRefs } from 'vue';
import RightBar from '../../components/RightBar.vue';
import GridContainer from '../../components/GridContainer.vue';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { LogicalSize } from '@tauri-apps/api/dpi';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { openUrl, openPath } from '@tauri-apps/plugin-opener';
import { exists } from '@tauri-apps/plugin-fs';
import { isRegistered, register } from '@tauri-apps/plugin-global-shortcut';
import { shortcutStore } from '../../stores/shortcut';
import { exec } from "../../functions/open";

type searchResult = {
    name: string,
    path: string,
    kind: string,
    ext: string
}
const shortcutstore = shortcutStore()
const { shortcuts } = toRefs(shortcutstore)
const tab = ref(0);
const inputvalue = ref("");
onMounted(async () => {
    let res = await isRegistered("Ctrl+Alt+Space")
    console.log("未注册");
    if (!res) {
        register("Ctrl+Alt+Space", async (e) => {
            if (e.state !== "Pressed") return;
            let show = await getCurrentWebviewWindow().isVisible();
            if (show) {
                getCurrentWebviewWindow().hide()
                let dom = document.getElementById("container")
                if (dom) dom.style.height = '50px'
                getCurrentWebviewWindow().setSize(new LogicalSize(600, 50));
            } else {
                searchresult.value = []
                inputvalue.value = ""
                getCurrentWebviewWindow().show()
                getCurrentWebviewWindow().center()
                getCurrentWebviewWindow().setFocus();

            }
        })
    }
})

window.addEventListener("storage", (e) => {
    if (e.key == "shortcut") {
        shortcutstore.$hydrate();
    }
});

const searchresult = ref<searchResult[]>([]);
let timer: string | number | NodeJS.Timeout | undefined;
const search = async function (e: any) {
    if (e.target.value == "") {
        clearTimeout(timer);
        focusindex.value = -1
        searchresult.value = []
        return
    }
    if (timer) {
        clearTimeout(timer);
    }
    timer = setTimeout(async () => {
        console.log(e.target.value)
        let show = await getCurrentWebviewWindow().isVisible();
        if (show) {
            searchresult.value = await invoke('search_query', { str: e.target.value });
            console.log(searchresult.value)
        }
    }, 500);
}

getCurrentWebviewWindow().listen("tauri://blur", () => {
    getCurrentWebviewWindow().hide()
    let dom = document.getElementById("container")
    if (dom) dom.style.height = '50px'
    focusbool.value = false
    setTimeout(() => {
        getCurrentWebviewWindow().setSize(new LogicalSize(600, 50));
    }, 300)
})
const focusbool = ref(false)
getCurrentWebviewWindow().listen("tauri://focus", () => {
    getCurrentWebviewWindow().setSize(new LogicalSize(600, 500));
    let dom = document.getElementById("container")
    if (dom) dom.style.height = '500px'
    focusbool.value = true
})

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

const searchenter = function (e: any) {
    if (focusindex.value >= 0) {
        openPath(searchresult.value[focusindex.value].path)
    } else {
        getCurrentWebviewWindow().hide()
        openUrl("https://cn.bing.com/search?q=" + e.target.value)
    }

    inputvalue.value = ""
    searchresult.value = []
}

const focusindex = ref(-1);
let isPressed = false;
window.addEventListener("keydown", (e) => {
    console.log(e)
    if (isPressed) return
    let dom = document.getElementById("search-result")
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
            <div v-for="(item, index) in searchresult" :id="'search-' + index" :key="item.path" class="search-item"
                @click="openfile(item)" :style="{ background: focusindex == index ? '#e6e9f0' : '' }">
                <v-chip v-if="item.kind" class="search-item-kind" color="primary" variant="flat">{{ item.kind
                    }}</v-chip>
                <div class="search-item-name">{{ item.name }}</div>
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
        <RightBar style="width: 100vw;height: 100vh;display: flex;justify-content: center;align-items: center;"
            border-radius=" 25px" background="rgba(123,123,123,0.3)">
            <v-btn icon="mdi-close" size="small" @click="getCurrentWebviewWindow().close"></v-btn>
        </RightBar>
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