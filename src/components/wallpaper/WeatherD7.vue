<script setup lang="ts">
import { ref } from "vue";
import "qweather-icons/font/qweather-icons.css";
import { weatherNowWallpaper, weatherD7Wallpaper, WeatherType, WeatherD7Type } from "../../functions/weather";
import { weatherIcon, windIcon } from "../../functions/weatnerIcon";
const w = ref<WeatherType>({ temp: "", feelsLike: "", icon: "", text: "", windDir: "", windScale: "", windSpeed: "", humidity: "", precip: "", pressure: "", vis: "", cloud: "", dew: "", city: "", });
const weather = ref("");
const wind = ref("")
weatherNowWallpaper((e) => {
    w.value = e
    weather.value = getWeatherIcon(Number(e.icon))
    wind.value = getWindIcon(Number(e.windScale))
}).then((e) => {
    w.value = e
    weather.value = getWeatherIcon(Number(e.icon))
    wind.value = getWindIcon(Number(e.windScale))
})
type WeatherD7TypeIcon = WeatherD7Type & {
    iconsrc: string
    week: String | number
}
const ws = ref<{
    city: string,
    daily: WeatherD7TypeIcon[]
}>({
    city: "",
    daily: []
})

weatherD7Wallpaper((e) => {
    let now = new Date()
    ws.value.daily.length = 0;
    for (let i = 0; i < e.daily.length; i++) {
        let iconsrc = getWeatherIcon(Number(e.daily[i].iconDay))
        let week: number | string = now.getDay()
        switch (week) {
            case 0: week = '一'; break;
            case 1: week = '二'; break;
            case 2: week = '三'; break;
            case 3: week = '四'; break;
            case 4: week = '五'; break;
            case 5: week = '六'; break;
            case 6: week = '七'; break;
        }
        if (i == 1) week = '明天'
        now.setDate(now.getDate() + 1)
        ws.value.daily.push({
            ...e.daily[i],
            iconsrc,
            week
        })
    }
}).then((e) => {
    ws.value.daily.length = 0;
    let now = new Date()
    for (let i = 0; i < e.daily.length; i++) {
        let iconsrc = getWeatherIcon(Number(e.daily[i].iconDay))
        let week: number | string = now.getDay()
        switch (week) {
            case 1: week = '周一'; break;
            case 2: week = '周二'; break;
            case 3: week = '周三'; break;
            case 4: week = '周四'; break;
            case 5: week = '周五'; break;
            case 6: week = '周六'; break;
            case 0: week = '周日'; break;
        }
        if (i == 1) week = '明天'
        now.setDate(now.getDate() + 1)
        ws.value.daily.push({
            ...e.daily[i],
            iconsrc,
            week
        })
    }
})

const getWeatherIcon = function (id: number) {
    let res = weatherIcon.filter(e => {
        return e.code == id
    })
    return `/svg/${res[0].icon}.svg`
}

const getWindIcon = function (id: number) {
    let res = windIcon.filter(e => {
        return e.code == id
    })
    return `/svg/${res[0].icon}.svg`
}
</script>
<template>
    <div class="weatherD7">
        <div class="weatherD7_today">
            <div class="weatherD7_today_temp">
                <div>
                    {{ w.temp }}<span class="weatherD7_today_temp_c">℃</span>
                </div>
            </div> <img :src="weather" class="weatherD7_today_icon">
            <img :src="wind" class="weatherD7_today_wind_icon">
            <div>
                <div>
                    {{ w.city }}
                </div>
                <div>
                    {{ w.text }}
                </div>
                <div v-if="ws.daily.length > 0">
                    {{ ws.daily[0].tempMin }}° - {{ ws.daily[0].tempMax }}°
                </div>
            </div>
        </div>
        <div class="weatherD7_future">
            <div v-for="(item, index) in ws.daily" v-show="index != 0">
                <div>{{ item.week }}</div>
                <img :src="item.iconsrc">
                <div>{{ item.textDay }}</div>
                <div>
                    {{ item.tempMin }}°-{{ item.tempMax }}°
                </div>
            </div>
        </div>
    </div>
</template>
<style>
.weatherD7 {
    position: absolute;
    z-index: 400;
    border-radius: 50px;
    color: rgba(250, 250, 250, 1);
    width: 400px;
    height: 240px;
    display: flex;
    flex-direction: column;
    justify-content: space-evenly;
    align-items: center;
}

.weatherD7_today {
    width: 100%;
    height: 100px;
    display: flex;
    justify-content: space-evenly;
    align-items: center;
    overflow: hidden;
}

.weatherD7_today_temp {
    font-size: 50px;
}

.weatherD7_today_temp_c {
    font-size: 18px;
    position: absolute;
    top: 20px;
}

.weatherD7_today_icon {
    width: 100px;
}

.weatherD7_today_wind_icon {
    width: 100px;
}

.weatherD7_future {
    display: flex;
    justify-content: space-evenly;
    align-items: center;
    font-size: 14px;
    width: 100%;
    height: calc(100% - 100px);
    text-align: center;
}
</style>