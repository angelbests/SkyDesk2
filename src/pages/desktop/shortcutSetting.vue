<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { useRoute } from 'vue-router';
import { emit } from '@tauri-apps/api/event';
import { LogicalSize } from '@tauri-apps/api/dpi';
import { open } from '@tauri-apps/plugin-dialog';
import { convertFileSrc } from '@tauri-apps/api/core';
const route =useRoute()
const shortcutWindows = ref(JSON.parse(localStorage.getItem(route.query.label as string) || "{}"));
onMounted(async () => {
    getCurrentWebviewWindow().setMaximizable(false)
    getCurrentWebviewWindow().setResizable(false)
    getCurrentWebviewWindow().setSize(new LogicalSize(400,380))
    document.getElementById("toolbar")?.querySelector(".v-toolbar__content")?.setAttribute("data-tauri-drag-region","true")
    document.getElementById("toolbar")?.querySelector(".v-toolbar-title__placeholder")?.setAttribute("data-tauri-drag-region","true")
})

const emitToShortcut = async function(str:string){
    if(str == 'w' || str == 'h'){
        shortcutWindows.value.setting.font = false
    }
    await emit(route.query.label+"-setting",{
        key:str,
        value:shortcutWindows.value.setting[str]
    })
}

const getImage = async function() {
    let res = await open({
            "filters":[
                {"extensions":['png','jpg','jpeg'],name:"Image"}
            ],
            "title":'选择背景图片'
        })
        if(res){
            shortcutWindows.value.setting.background =  `url('${convertFileSrc(res)}')`
            emitToShortcut('background')
        }
}

</script>

<template>
    <v-card class="mx-auto" max-width="400" style="background-color: white;width: 100vw;height: 100vh;">
        <v-toolbar id="toolbar" color="purple" density="compact">
            <v-toolbar-title data-tauri-drag-region>设置快捷容器</v-toolbar-title>
            <v-spacer></v-spacer>
            <v-btn icon="mdi-close" @click="getCurrentWebviewWindow().close()"></v-btn>
        </v-toolbar>
        <v-divider></v-divider>
        <v-list lines="one" select-strategy="classic">
            <v-list-item>
                <template v-slot:append>
                    <v-text-field v-model="shortcutWindows.setting.background" @update:model-value="emitToShortcut('background')" width="200" hide-details="auto" density="compact">
                        <template v-slot:prepend-inner>
                            <v-btn variant="tonal" size="mini" @click="getImage">
                                <template v-slot:append>
                                    <v-icon>mdi-image</v-icon>
                                </template>
                            </v-btn>
                        </template>
                    </v-text-field>
                </template>
                <v-list-item-title>容器背景</v-list-item-title>
            </v-list-item>
            <v-divider></v-divider>
            <v-list-item>
                <template v-slot:append>
                    <v-number-input v-model="shortcutWindows.setting.blur" @update:model-value="emitToShortcut('blur')" style="width: 200px;" hide-details="auto" density="compact" :min="0" :max="20" ></v-number-input>
                </template>
                <v-list-item-title>容器虚化</v-list-item-title>
            </v-list-item>
            <v-divider></v-divider>
            <v-list-item>
                <template v-slot:append>
                    <v-number-input v-model="shortcutWindows.setting.r" @update:model-value="emitToShortcut('r')"  control-variant="stacked" :min="1" :max="100"  width="100" hide-details="auto" density="compact"></v-number-input>
                    <v-number-input v-model="shortcutWindows.setting.c" @update:model-value="emitToShortcut('c')"  control-variant="stacked" :min="1" :max="100"  width="100" hide-details="auto" density="compact"></v-number-input>
                </template>
                <v-list-item-title>容器网格</v-list-item-title>
            </v-list-item>
            <v-divider></v-divider>
            <v-list-item>
                <v-list-item-title>容器宽高</v-list-item-title>
                <template v-slot:append>
                    <v-number-input v-model="shortcutWindows.setting.w" @update:model-value="emitToShortcut('w')"  control-variant="stacked" :min="60" :max="4000"  width="100" hide-details="auto" density="compact"></v-number-input>
                    <v-number-input v-model="shortcutWindows.setting.h" @update:model-value="emitToShortcut('h')"  control-variant="stacked" :min="60" :max="4000"  width="100" hide-details="auto" density="compact"></v-number-input>
                </template>
            </v-list-item>
            <v-divider></v-divider>
            <v-list-item>
                <template v-slot:append>
                    <v-switch v-model="shortcutWindows.setting.font" @update:model-value="emitToShortcut('font')"   hide-details="auto" density="compact"></v-switch>
                </template>
                <v-list-item-title>字体显示</v-list-item-title>
            </v-list-item>
            <v-divider></v-divider>
            <v-list-item>
                <template v-slot:append>
                    <v-switch v-model="shortcutWindows.setting.alwaysOnTop" @update:model-value="emitToShortcut('alwaysOnTop')" hide-details="auto" density="compact"></v-switch>
                </template>
                <v-list-item-title>容器置顶</v-list-item-title>
            </v-list-item>
            <v-divider></v-divider>
        </v-list>
    </v-card>
</template>