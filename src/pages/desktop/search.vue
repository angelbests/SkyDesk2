<script setup lang="ts">
import { onMounted, ref } from 'vue';
import RightBar from '../../components/RightBar.vue';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { LogicalSize } from '@tauri-apps/api/dpi';
import { invoke } from '@tauri-apps/api/core';
import { openUrl, openPath } from '@tauri-apps/plugin-opener';
import { exists } from '@tauri-apps/plugin-fs';
import { isRegistered, register } from '@tauri-apps/plugin-global-shortcut';
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
                setTimeout(() => {
                    getCurrentWebviewWindow().setSize(new LogicalSize(600, 50));
                }, 300)
                show = false
            } else {
                searchresult.value = []
                inputvalue.value = ""
                getCurrentWebviewWindow().show()
                getCurrentWebviewWindow().center()
                getCurrentWebviewWindow().setFocus();
                show = true
            }
        })
    }
})
type searchResult = {
    name: string,
    path: string,
    kind: string,
    ext: string
}
const searchresult = ref<searchResult[]>([]);
let timer: string | number | NodeJS.Timeout | undefined;
const search = async function (e: any) {
    if (e.target.value == "") {
        clearTimeout(timer);
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
    setTimeout(() => {
        getCurrentWebviewWindow().setSize(new LogicalSize(600, 50));
    }, 300)
})

getCurrentWebviewWindow().listen("tauri://focus", () => {
    getCurrentWebviewWindow().setSize(new LogicalSize(600, 500));
    let dom = document.getElementById("container")
    if (dom) dom.style.height = '500px'
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
    getCurrentWebviewWindow().hide()
    openUrl("https://cn.bing.com/search?q=" + e.target.value)
    inputvalue.value = ""
    searchresult.value = []
}
</script>

<template>
    <div class="container" id="container">
        <div class="search-bar">
            <v-icon style="font-size: 25px;margin-left: 25px;color:rgba(123,123,123,0.8);">mdi-magnify</v-icon>
            <input @input="search" v-model="inputvalue" type="text" @keyup.enter="searchenter" placeholder="请输入搜索内容"
                class="search-input" />
        </div>
        <div class="search-result" v-if="searchresult.length > 0">
            <div v-for="item in searchresult" :key="item.path" class="search-item" @click="openfile(item)">
                <v-chip v-if="item.kind" class="search-item-kind" color="primary" variant="flat">{{ item.kind
                    }}</v-chip>
                <div class="search-item-name">{{ item.name }}</div>
            </div>
        </div>
        <RightBar style="width: 100vw;height: 100vh;display: flex;justify-content: center;align-items: center;"
            border-radius=" 25px" background="rgba(123,123,123,0.3)">
            <v-btn icon="mdi-close" size="small" @click="getCurrentWebviewWindow().close"></v-btn>
        </RightBar>
    </div>
</template>

<style scoped>
.container {
    width: 100vw;
    height: 99vh;
    background-image: linear-gradient(to top, #fad0c4 0%, #ffd1ff 100%);
    border-radius: 25px;
    transition: all 0.3s ease-in-out;
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
    border-radius: 25px;
}
</style>