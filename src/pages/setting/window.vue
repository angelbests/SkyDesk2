<script setup lang="ts" >
import { onMounted, ref } from 'vue';
import { windowStore } from '../../stores/window';
const windowsstore = windowStore()
const windowsref = ref<HTMLDivElement>()
const windowsListHeight = ref()
onMounted(()=>{
    updateElementHeight()
    window.addEventListener("resize",updateElementHeight)
    window.addEventListener("storage",(e)=>{
        if(e.key == "window"){
            windowsstore.$hydrate();
        }
    })
})

const updateElementHeight = function () {
    if (windowsref.value) {
        windowsListHeight.value = Math.ceil((windowsstore.windows.length / Math.trunc(windowsref.value.offsetWidth / 420))) * 320 + 'px';
    }
}

</script>

<template>
    <div class="window">
        <v-card
            style="width: 100%;height: 60px;display: flex;align-items: center;box-sizing: border-box;padding: 0 20px;filter:drop-shadow(0px 2px 5px gray)">
            <v-btn style="margin-right: 20px;">
                <template v-slot:prepend>
                    <v-icon>mdi-magnify-scan</v-icon>
                </template>
                按钮
            </v-btn>
        </v-card>
        <v-progress-linear color="black" :indeterminate="false"></v-progress-linear>
        <div style="width: 100%;height: calc(100% - 60px);display: flex;overflow: hidden;background: white;">
            <div class="windows" id="windows" ref="windowsref">
                <div class="windows-list" :style="{height:windowsListHeight}">
                    <v-card prepend-icon="" width="400" height="305" variant="elevated" elevation="10"
                        v-for="item in windowsstore.windows">
                        <v-card-text style="position: relative;">
                            <v-img  cover :height="220"></v-img>
                            <div style="width: 100%;position: absolute;left: 15px;top: 240px;z-index: 50;color: gray;">
                                {{ item.label }}
                            </div>
                        </v-card-text>
                        <v-card-actions>
                            <v-btn>按钮</v-btn>
                        </v-card-actions>
                    </v-card>
                </div>
            </div>
        </div>
    </div>
</template>

<style>
.window{
    width: 100%;
    height: 100%;
}
.windows {
    width: 100%;
    background: white;
    display: flex;
    justify-content: center;
    overflow-x: hidden;
    overflow-y: scroll;
}

.windows-list {
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