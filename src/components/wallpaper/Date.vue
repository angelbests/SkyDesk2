<script setup lang="ts">
import { ref } from "vue";
import { get_time, TimeWallpaper } from "../../functions/date";
const time = ref<TimeWallpaper>({ year: "0000", month: "00", day: "00", "week": "星期一", hour: "00", minute: "00", second: "00" });
setInterval(() => {
    time.value = get_time()
}, 1000);
</script>
<template>
    <div class="date">
        <div class="date_time">
            {{ time.hour }}
            <div class="date_dot">:</div>
            {{ time.minute }}
        </div>
        <div class="date_date">
            {{ time.year }} - {{ time.month }} - {{ time.day }}
        </div>
        <div class="date_week">{{ time.week }}</div>
    </div>
</template>
<style>
/* date 日期 */
.date {
    position: absolute;
    z-index: 500;
    width: 400px;
    height: 200px;
    border-radius: 50px;
    text-align: center;
}

.date_time {
    position: absolute;
    width: 100%;
    height: 60px;
    text-align: center;
    font-size: 90px;
    font-weight: bolder;
    text-align: center;
    display: flex;
    justify-content: center;
    align-items: start;
}

.date_date {
    width: 100%;
    height: 50px;
    text-align: center;
    font-size: 20px;
    left: 0px;
    top: 170px;
    position: absolute;
    text-align: center;
}

.date_week {
    width: 100%;
    height: 50px;
    text-align: center;
    font-size: 18px;
    left: 0px;
    top: 130px;
    position: absolute;
    text-align: center;
}

.date_dot {
    display: flex;
    position: relative;
    top: -10px;
    animation: 1s normal 0s infinite ease dot;
    align-items: center;
    width: 40px;
    justify-content: center;
}

@keyframes dot {
    0% {
        opacity: 0;
    }

    50% {
        opacity: 0;
    }

    100% {
        opacity: 1;
    }
}
</style>