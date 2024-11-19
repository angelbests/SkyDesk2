<script setup lang="ts" >
import { onMounted, ref, toRefs, watch } from 'vue';
import { useRoute } from 'vue-router';
import { convertFileSrc } from '@tauri-apps/api/core';
import { weatherStore } from '../../stores/window';
import { getWeather } from '../../api/weather';
import 'qweather-icons/font/qweather-icons.css'
const weatherstore = weatherStore()
const { city,apikey,citycode } = toRefs(weatherstore)
const route = useRoute()
const show = ref(false)
const path = ref()
const type = ref()
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
            <div style="width: 200px;height: 30px;font-size: 20px;text-align: center;">
                {{ city }} 
            </div>
            <div style="width: 200px;display: flex;height: 60px;">
                <div style="width: 100px;height: 50px;font-size: 20px;line-height: 50px;text-align: center;">
                    {{ w.text }} {{ w.temp }}°
                </div>
                <div style="width: 100px;height: 50px;text-align: center;line-height: 50px;">
                    <i style="font-size: 30px;" :class="['qi-'+(w.icon?w.icon:100),'weather-icon']"></i>
                </div>
            </div>
            <div style="width: 200px;height: 60px;font-size: 20px;display: flex;">
                <div style="width: 100px;height: 50px;text-align: center;">
                    {{ w.windDir }} 
                </div>
                <div style="width: 100px;height: 50px;text-align: center;">
                    {{ w.windScale }}级
                </div>
            </div>
        </div>
    </div>
</template>

<style>
.window{
    width: 100vw;
    height: 100vh;
    background-color: black;
    position: relative;
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
</style>