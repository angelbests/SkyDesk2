<script setup lang="ts" >
import { ref } from 'vue';
import vMonth from '../../components/Month.vue';
const month = ref(11)
const year = ref(2024)
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
        </v-card>
        <v-progress-linear color="black" :indeterminate="false"></v-progress-linear>
        <div style="width: 100%;height: calc(100% - 60px);display: flex;overflow: hidden;background: white;">
            <v-month v-model:month="month" v-model:year="year"></v-month>
        </div>
    </div>
</template>

<style>
.window{
    width: 100%;
    height: 100%;
}
</style>