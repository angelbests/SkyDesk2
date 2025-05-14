<script setup lang="ts">
import { ref } from "vue"
import vMonth from "../../components/Month.vue"
import { systemStore } from "../../stores/system"
import { DayInfo, today as gettoday } from "../../functions/calendar"
const systemstore = systemStore()
window.addEventListener("storage", (e) => {
    if (e.key == "system") {
        systemstore.$hydrate()
    }
})
const date = ref<{
    year: number
    month: number
}>({
    year: new Date().getFullYear(),
    month: new Date().getMonth() + 1,
})
const today = ref<DayInfo | null>(null)
const updatedate = function () {
    today.value = gettoday(new Date().getFullYear(), new Date().getMonth() + 1, new Date().getDate())
}
updatedate()
setInterval(() => {
    updatedate()
}, 60 * 1000);
</script>

<template>
    <div class="calendar" v-if="today">
        <!-- <div style="width: 100%;height: 100%;position: absolute;left: 0;top: 0;border-radius: 50px;
        font-size: 250px;display: flex;justify-content: center;align-items: center;color: rgba(125,125,125,0.3);">
            {{ today.solar_month }}
        </div> -->
        <v-month v-model:date="date" :weekfontsize="14">
            <template v-slot:default="{ day }">
                <div :style="{
                    background:
                        today.solar_year == day.solar_year &&
                            today.solar_month == day.solar_month &&
                            today.solar_day == day.solar_day
                            ? 'rgba(233,233,233,0.3)'
                            : '',
                }" class="day">
                    <div>
                        {{ day.solar_day }}
                    </div>
                    <div style="font-size: 10px;">
                        <span v-if="day.solar_festival">{{ day.solar_festival }}</span>
                        <span v-else-if="day.lunar_festival">{{ day.lunar_festival }}</span>
                        <span v-else-if="day.lunar_term">{{ day.lunar_term }}</span>
                        <span v-else> {{ day.lunar_day == '初一' ? day.lunar_month : day.lunar_day }}</span>
                    </div>
                </div>
            </template>
        </v-month>
    </div>
</template>

<style>
.calendar {
    box-sizing: border-box;
    padding: 20px;
    font-size: 14px;
    position: absolute;
    z-index: 200;
    width: 350px;
    height: 350px;
    border-radius: 50px;
}

.day {
    width: 100%;
    height: 100%;
    text-align: center;
    box-sizing: border-box;
    transition: all 0.1s linear;
    border-radius: 10px;
}

.day:hover {
    padding: 5px;
    background: rgba(222, 222, 222, 0.3);
    border-radius: 10px;
}
</style>
