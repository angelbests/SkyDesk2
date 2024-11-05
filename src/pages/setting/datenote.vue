<script setup lang="ts" >
import { ref } from 'vue';
import vMonth from '../../components/Month.vue';
const date = ref<{
    year:number,
    month:number
}>({
    year:new Date().getFullYear(),
    month:new Date().getMonth()+1
})

const wheelyear = function(e:WheelEvent){
    if(e.deltaY == -100){
        if(date.value.year == 1900){
            date.value.year =2100;
        }else{
            date.value.year -= 1;
        }
    }else if(e.deltaY == 100){
        if(date.value.year == 2100){
            date.value.year =1900;
        }else{
            date.value.year += 1;
        }
    }
}

const wheelmonth = function(e:WheelEvent){
    if(e.deltaY == -100){
        if(date.value.month == 1){
            date.value.month = 12;
            date.value.year -=1
        }else{
            date.value.month -= 1;
        }
    }else if(e.deltaY == 100){
        if(date.value.month == 12){
            date.value.month = 1;
            date.value.year +=1
        }else{
            date.value.month += 1;
        }
    }
}
</script>

<template>
    <div class="window">
        <v-card @wheel.self="wheelmonth"
            style="width: 100%;height: 60px;display: flex;align-items: center;box-sizing: border-box;padding: 0 20px;filter:drop-shadow(0px 2px 5px gray)">
            <div style="display: flex;flex-direction: row;justify-content: center;align-items: center;font-size: 20px;height:100%;margin-right: 20px;width: 150px;">
                <div @wheel="wheelyear" style="height: 100%;line-height: 300%;">
                     {{ date.year }} 年
                </div>
                <div @wheel="wheelmonth" style="height: 100%;line-height: 300%;">
                     {{ date.month<10?'0'+date.month:date.month }} 月
                </div>
            </div>
            <v-btn style="margin-right: 20px;">
                <template v-slot:prepend>
                    <v-icon>mdi-clipboard-text-clock</v-icon>
                </template>
                新建日程
            </v-btn>
        </v-card>
        <v-progress-linear color="black" :indeterminate="false"></v-progress-linear>
        <div style="width: 100%;height: calc(100% - 60px);display: flex;overflow: hidden;background: white;">
            <v-month v-model:date="date"></v-month>
        </div>
    </div>
</template>

<style>
.window{
    width: 100%;
    height: 100%;
}
</style>