<script setup lang="ts">
import { ref } from "vue";
import "qweather-icons/font/qweather-icons.css";
import { weatherNowWallpaper, weatherD7Wallpaper, WeatherType, WeatherD7Type } from "../../functions/weather";
import { weatherIcon, windIcon } from "../../functions/weatnerIcon";
const w = ref<WeatherType>({ temp: "", feelsLike: "", icon: "", text: "", windDir: "", windScale: "", windSpeed: "", humidity: "", precip: "", pressure: "", vis: "", cloud: "", dew: "", city: "", });
weatherNowWallpaper((e) => {
    w.value = e
    icon1.value = getWeatherIcon(Number(e.icon))
    wind.value = getWindIcon(Number(e.windScale))
    console.log(e)
}).then((e) => {
    w.value = e
    icon1.value = getWeatherIcon(Number(e.icon))
    wind.value = getWindIcon(Number(e.windScale))
    console.log(e)
})
const ws = ref<{
    city: string,
    daily: WeatherD7Type[]
}>({
    city: "",
    daily: []
})
weatherD7Wallpaper((e) => {
    ws.value = e
    icon2.value = getWeatherIcon(Number(e.daily[0].iconDay))
    console.log(e)
}).then((e) => {
    ws.value = e
    icon2.value = getWeatherIcon(Number(e.daily[0].iconDay))
    console.log(e)
})
const icon1 = ref("");
const icon2 = ref("");
const getWeatherIcon = function (id: number) {
    let res = weatherIcon.filter(e => {
        return e.code == id
    })
    return `/svg/${res[0].icon}.svg`
}
const wind = ref("")
const getWindIcon = function (id: number) {
    let res = windIcon.filter(e => {
        return e.code == id
    })
    return `/svg/${res[0].icon}.svg`
}

// import { readDir } from "@tauri-apps/plugin-fs";
// const readfile = async function () {
//     let res = await readDir("D:\\SkyDesk2\\public\\svg")
//     let arr = []
//     res.filter(e => {
//         arr.push(e.name)
//     })
// }
// readfile()
</script>

<template>
    <!-- weather -->
    <div class="weather" v-if="ws.daily.length > 0">
        <div class="weather_city ">
            {{ w.city }}
        </div>
        <img class="weather_w" :src="icon1">
        <img class="weather_wind" :src="wind">
        <div class="weather_temp">
            {{ w.temp }}
            <div class="weather_temp_c">°C
            </div>
        </div>
        <div class="weather_temp_rang">
            {{ ws.daily[0].tempMin }}
            <div class="weather_temp_rang_c">°C</div>
            <div style="margin: 0px 8px;">
                -
            </div>
            {{ ws.daily[0].tempMax }}
            <div class="weather_temp_rang_c">°C</div>
        </div>
        <div class="weather_text">
            {{ w.text }}
        </div>
        <div class="weather_winddir">
            {{ w.windDir }}
        </div>
        <div class="weather_rh">
            {{ w.humidity }}% RH
        </div>
    </div>

</template>
<style>
.weather {
    width: 200px;
    height: 200px;
    background: rgba(253, 253, 253, 0.1);
    border-radius: 50px;
    position: absolute;
    z-index: 400;
    color: rgba(250, 250, 250, 1);
    box-shadow: 0px 0px 10px black;
}

.weather_city {
    width: 200px;
    height: 30px;
    position: absolute;
    left: 0px;
    top: 7px;
    font-size: 15px;
    font-weight: bold;
    display: flex;
    justify-content: center;
    align-items: center;
    text-shadow: 0px 0px 10px black;
}

.weather_w {
    width: 100px;
    height: 100px;
    position: absolute;
    left: 0px;
    top: 20px;
    filter: drop-shadow(0px 0px 5px black);
}

.weather_wind {
    width: 100px;
    height: 100px;
    position: absolute;
    left: 0px;
    top: 100px;
    filter: drop-shadow(0px 0px 10px gray);
}

.weather_temp {
    width: 100px;
    height: 70px;
    position: absolute;
    left: 100px;
    top: 30px;
    font-size: 30px;
    font-weight: bold;
    display: flex;
    justify-content: center;
    align-items: center;
    text-shadow: 0px 0px 10px black;
}

.weather_temp_c {
    margin-left: 5px;
    height: 30px;
    font-size: 15px;
    display: flex;
    align-items: start;
    text-shadow: 0px 0px 10px black;
}

.weather_temp_rang {
    width: 100px;
    height: 30px;
    position: absolute;
    left: 100px;
    top: 85px;
    font-size: 15px;
    font-weight: bold;
    display: flex;
    justify-content: center;
    align-items: center;
    text-shadow: 0px 0px 10px black;
}

.weather_temp_rang_c {
    margin-left: 1px;
    height: 20px;
    font-size: 8px;
    text-shadow: 0px 0px 10px black;
}

.weather_text {
    width: 100px;
    height: 30px;
    position: absolute;
    left: 100px;
    top: 110px;
    font-size: 15px;
    font-weight: bold;
    display: flex;
    justify-content: center;
    align-items: center;
    text-shadow: 0px 0px 10px black;
}

.weather_winddir {
    width: 100px;
    height: 30px;
    position: absolute;
    left: 100px;
    top: 135px;
    font-size: 15px;
    font-weight: bold;
    display: flex;
    justify-content: center;
    align-items: center;
    text-shadow: 0px 0px 10px black;
}

.weather_rh {
    width: 100px;
    height: 30px;
    position: absolute;
    left: 100px;
    top: 160px;
    font-size: 15px;
    font-weight: bold;
    display: flex;
    justify-content: center;
    align-items: center;
    text-shadow: 0px 0px 10px black;
}
</style>
