<script setup lang="ts" >

</script>

<template>
    <div class="window">
        <v-card
            style="width: 100%;height: 60px;display: flex;align-items: center;box-sizing: border-box;padding: 0 20px;filter:drop-shadow(0px 2px 5px gray)">
            <v-btn style="margin-right: 20px;">
                <template v-slot:prepend>
                    <v-icon>mdi-magnify-scan</v-icon>
                </template>
                录屏
            </v-btn>
            <v-btn style="margin-right: 20px;">
                <template v-slot:prepend>
                    <v-icon>mdi-view-grid-plus-outline</v-icon>
                </template>
                添加快捷
            </v-btn>
            <v-btn style="margin-right: 20px;">
                <template v-slot:prepend>
                    <v-icon>mdi-cog-outline</v-icon>
                </template>
                设置快捷
            </v-btn>
            <v-btn style="margin-right: 20px;">
                <template v-slot:prepend>
                    <v-icon>mdi-shape-rectangle-plus</v-icon>
                </template>
                添加桌面合集
            </v-btn>
        </v-card>
        <div style="width: 100%;height: calc(100% - 60px);display: flex;overflow: hidden;background: white;">

        </div>
    </div>
</template>

<style>
.window{
    width: 100%;
    height: 100%;
}
</style>