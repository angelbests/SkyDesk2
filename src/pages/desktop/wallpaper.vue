<script setup lang="ts" >
import { onMounted, ref, toRefs, watch } from 'vue';
import { useRoute } from 'vue-router';
import { convertFileSrc } from '@tauri-apps/api/core';
import { weatherStore } from '../../stores/window';
import { getWeather } from '../../api/weather';
import { listen } from "@tauri-apps/api/event";
import 'qweather-icons/font/qweather-icons.css'
const weatherstore = weatherStore()
const { city,apikey,citycode } = toRefs(weatherstore)
const route = useRoute()
const show = ref(false)
const path = ref()
const type = ref()
const net = ref({
    speed_r:0,
    speed_s:0
})
onMounted(async ()=>{
    window.addEventListener("storage",(e)=>{
        if(e.key == "weather"){
            weatherstore.$hydrate()
        }
    })
    path.value = route.query.path
    type.value = route.query.type
    setTimeout(async ()=>{
        show.value = true
    },500)

    setInterval(async () => {
        let res = await getWeather(citycode.value)
        console.log(res)
        if(res.code == 200){
            w.value= res.now
        }
    }, 60*60*60);
    listen("netspeed",(e)=>{
        let res = JSON.parse(e.payload as string)
        net.value.speed_r = res.speed_r;
        net.value.speed_s = res.speed_s
    })
})
const w = ref<{
    "temp": string, // 温度
    "feelsLike":string, // 体感温度
    "icon": string, // 天气图标
    "text": string, // 文本描述
    "windDir": string, // 风向
    "windScale": string, // 风等级
    "windSpeed": string, // 风速
    "humidity": string, // 相对湿度
    "precip": string, // 过去1小时降水量，默认单位：毫米
    "pressure": string, // 大气压强
    "vis": string, // 能见度
    "cloud": string, // 云量
    "dew":string // 露点温度
}>({
    "temp": "", // 温度
    "feelsLike":"", // 体感温度
    "icon": "", // 天气图标
    "text": "", // 文本描述
    "windDir": "", // 风向
    "windScale": "", // 风等级
    "windSpeed": "", // 风速
    "humidity": "", // 相对湿度
    "precip": "", // 过去1小时降水量，默认单位：毫米
    "pressure": "", // 大气压强
    "vis": "", // 能见度
    "cloud": "", // 云量
    "dew":"" // 露点温度
})
watch(city,async ()=>{
    if(apikey.value && city){
        let res = await getWeather(citycode.value)
        console.log(res)
        if(res.code == 200){
            w.value= res.now
        }
    }
},{
    immediate:true
})
</script>

<template>
    <div class="window" >
        <img v-if="type=='image'" :src="convertFileSrc(path)" class="image"/>
        <video muted v-else class="video" id="video" ref="video" :src="convertFileSrc(path)" autoplay="true" loop="true"></video>
        <div style="display: flex;flex-direction: column;position: absolute;right:40px;top:40px;z-index: 100;width: 200px;height: 300px;">
            <div style="width: 200px;height: 30px;text-align: center;">
                {{ city }} 
            </div>
            <div style="width: 200px;display: flex;height: 60px;">
                <div style="width: 100px;height: 50px;line-height: 50px;text-align: center;">
                    {{ w.text }} {{ w.temp }}°
                </div>
                <div style="width: 100px;height: 50px;text-align: center;line-height: 50px;">
                    <i style="font-size: 30px;" :class="['qi-'+(w.icon?w.icon:100),'weather-icon']"></i>
                </div>
            </div>
            <div style="width: 200px;height: 60px;display: flex;">
                <div style="width: 100px;height: 50px;text-align: center;">
                    {{ w.windDir }} 
                </div>
                <div style="width: 100px;height: 50px;text-align: center;">
                    {{ w.windScale }}级
                </div>
            </div>
        </div>
        <div class="netspeed">
            <div data-tauri-drag-region style="width: 100px;display: flex;align-items: center;">
                <v-icon data-tauri-drag-region>mdi-arrow-down-thin</v-icon>{{ Math.trunc(net.speed_r/1024)<1024?Math.trunc(net.speed_r/1024)+'KB/s':Math.trunc(net.speed_r/1024/1024*10)/10+'MB/s' }}
            </div>
            <div style="width: 100px;display: flex;align-items: center;">
                <v-icon data-tauri-drag-region>mdi-arrow-up-thin</v-icon>{{ Math.trunc(net.speed_s/1024)<1024?Math.trunc(net.speed_s/1024)+'KB/s':Math.trunc(net.speed_s/1024/1024*10)/10+'MB/s' }}
            </div>
        </div>
    </div>
</template>

<style scoped>
.window{
    width: 100vw;
    height: 100vh;
    position: relative;
    font-size: 20px;
}
.image{
    width: 100vw;
    height: 100vh;
    object-fit: cover;
}
.video{
    width: 100vw;
    height: 100vh;
    object-fit: cover;
}
.netspeed {
    position: absolute;
    right: 100px;
    top: 200px;
    z-index: 200;
    width: 80px;
    height: 45px;
    font-size: 13px;
    display: flex;
    justify-content: center;
    flex-direction: column;
    align-items: flex-start;
}
</style>