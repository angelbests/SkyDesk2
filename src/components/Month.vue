<template>
    <div style="
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
        box-sizing: border-box;
        padding: 10px;
      ">
        <div style="
          display: grid;
          width: 100%;
          grid-template-columns: repeat(7, 1fr);
          grid-template-rows: repeat(1, 1fr);
        ">

        </div>
        <div style="width: 100%; height: 100%; position: relative">
            <div class="month">
                <div class="week">一</div>
                <div class="week">二</div>
                <div class="week">三</div>
                <div class="week">四</div>
                <div class="week">五</div>
                <div class="week">六</div>
                <div class="week">日</div>
                <div class="day" v-for="day in days">
                    <slot :day="day"></slot>
                </div>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { onMounted, ref, watch } from "vue";
import { DayInfo, today } from "../functions/calendar";
const days = ref<DayInfo[]>([]);
const date = defineModel<{ year: number, month: number }>("date", {
    default: () => ({
        year: new Date().getFullYear(),
        month: new Date().getMonth(),
    }),
});

onMounted(async () => {
    days.value = await monthdays(date.value.year, date.value.month);
});

watch(date.value, async () => {
    days.value = await monthdays(date.value.year, date.value.month);
});

const monthdays = async function (year: number, month: number) {
    let days: DayInfo[] = [];
    let now = new Date();
    now.setFullYear(year);
    now.setMonth(month - 1);
    now.setDate(1);
    let week = getWeekday(year, month - 1, 1);
    now.setDate(now.getDate() - week);
    for (let i = 0; i < 42; i++) {
        now.setDate(now.getDate() + 1);
        let day = today(now.getFullYear(), now.getMonth() + 1, now.getDate())
        days.push({
            ...day
        })
    }
    return days;
};
const getWeekday = function (year: number, month: number, day: number) {
    let week = new Date(year, month, day).getDay();
    if (week == 0) {
        week = 7;
    }
    return week;
};
</script>

<style>
.month {
    width: 100%;
    height: 100%;
    display: grid;
    grid-template-rows: repeat(7, 1fr);
    grid-template-columns: repeat(7, 1fr);
    position: relative;
    text-align: center;
    align-items: center;
}

.week {
    font-size: 17px;
}
</style>
