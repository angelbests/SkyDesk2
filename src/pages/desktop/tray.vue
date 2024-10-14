<script setup lang="ts" >
import { LogicalSize } from '@tauri-apps/api/dpi';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { exit } from '@tauri-apps/plugin-process';
getCurrentWebviewWindow().setSize(new LogicalSize(100,30))
</script>

<template>
    <div class="window">
        <v-btn style="width: 100px;height: 30px;" size="small" @click="exit()">退出</v-btn>
    </div>
</template>

<style>
.window{
    width: 100%;
    height: 100%;
}
</style>