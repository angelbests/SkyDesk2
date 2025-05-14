<script setup lang="ts">
import { computed, ref } from "vue"
import vMonth from "../../components/Month.vue"
import { systemStore } from "../../stores/system"
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
const today = ref<{
  year: number
  month: number
  day: number
}>({
  year: new Date().getFullYear(),
  month: new Date().getMonth() + 1,
  day: new Date().getDate()
})

const showmonth = computed(() => {
  return date.value.month < 10 ? "0" + date.value.month : date.value.month
})


const wheelyear = function (e: WheelEvent) {
  if (e.deltaY == -100) {
    if (date.value.year == 1900) {
      date.value.year = 2100
    } else {
      date.value.year -= 1
    }
  } else if (e.deltaY == 100) {
    if (date.value.year == 2100) {
      date.value.year = 1900
    } else {
      date.value.year += 1
    }
  }
}

const wheelmonth = function (e: WheelEvent) {
  if (e.deltaY == -100) {
    if (date.value.month == 1) {
      date.value.month = 12
      date.value.year -= 1
    } else {
      date.value.month -= 1
    }
  } else if (e.deltaY == 100) {
    if (date.value.month == 12) {
      date.value.month = 1
      date.value.year += 1
    } else {
      date.value.month += 1
    }
  }
}

// const repeat = [
//   { title: "不重复", value: 0 },
//   { title: "每天", value: 1 },
//   { title: "每周", value: 2 },
//   { title: "每月", value: 3 },
//   { title: "每年", value: 4 },
// ]
// const reminder = [
//   { title: "不提醒", value: 0 },
//   { title: "准时", value: 1 },
//   { title: "15 分钟前", value: 2 },
//   { title: "30 分钟前", value: 3 },
//   { title: "1 小时前", value: 4 },
//   { title: "2 小时前", value: 5 },
//   { title: "12 小时前", value: 6 },
//   { title: "1 天前", value: 6 },
//   { title: "2 天前", value: 7 },
//   { title: "1 周前", value: 8 },
// ]
// const weekday = [
//   { title: "周一", value: 1 },
//   { title: "周二", value: 2 },
//   { title: "周三", value: 3 },
//   { title: "周四", value: 4 },
//   { title: "周五", value: 5 },
//   { title: "周六", value: 6 },
//   { title: "周天", value: 7 },
// ]

</script>

<template>
  <div class="window">
    <v-card @wheel.self="wheelmonth" :style="{
      background: systemstore.btnbarbackground,
      backgroundSize: 'cover',
    }" class="btnbar">
      <div style="
          display: flex;
          flex-direction: row;
          justify-content: center;
          align-items: center;
          font-size: 20px;
          height: 100%;
          margin-right: 20px;
          width: 150px;
        ">
        <div @wheel="wheelyear" style="height: 100%; line-height: 300%">
          {{ date.year }} 年
        </div>
        <div @wheel="wheelmonth" style="height: 100%; line-height: 300%">
          {{ showmonth }} 月
        </div>
      </div>
      <!-- <v-btn style="margin-right: 20px">
        <template v-slot:prepend>
          <v-icon>mdi-calendar-range</v-icon>
        </template>
节日计时
</v-btn> -->
    </v-card>
    <v-progress-linear color="black" :indeterminate="false"></v-progress-linear>
    <div @wheel="wheelmonth" style="width: 100%; height: calc(100% - 64px); display: flex; overflow: hidden">
      <v-month v-model:date="date">
        <template v-slot:default="{ day }">
          <div :style="{
            background:
              today.year == day.solar_year &&
                today.month == day.solar_month &&
                today.day == day.solar_day
                ? 'rgba(133,133,133,0.5)'
                : '',
          }" class="day">
            <div style="font-size: 17px;">
              {{ day.solar_day }}
            </div>
            <div>
              <span v-if="day.solar_festival">{{ day.solar_festival }}</span>
              <span v-else-if="day.lunar_festival">{{ day.lunar_festival }}</span>
              <span v-else-if="day.lunar_term">{{ day.lunar_term }}</span>
              <span v-else> {{ day.lunar_day == '初一' ? day.lunar_month : day.lunar_day }}</span>
            </div>
          </div>
        </template>
      </v-month>
    </div>
  </div>
</template>

<style>
.window {
  width: 100%;
  height: 100%;
}

.btnbar {
  width: 100%;
  height: 60px;
  display: flex;
  align-items: center;
  box-sizing: border-box;
  padding: 0 20px;
  filter: drop-shadow(0px 2px 5px gray);
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
