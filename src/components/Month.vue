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
      <div style="border: 1px solid gray; border-bottom: none; text-align: center">
        一
      </div>
      <div style="
          border-right: 1px solid gray;
          border-top: 1px solid gray;
          text-align: center;
        ">
        二
      </div>
      <div style="
          border-right: 1px solid gray;
          border-top: 1px solid gray;
          text-align: center;
        ">
        三
      </div>
      <div style="
          border-right: 1px solid gray;
          border-top: 1px solid gray;
          text-align: center;
        ">
        四
      </div>
      <div style="
          border-right: 1px solid gray;
          border-top: 1px solid gray;
          text-align: center;
        ">
        五
      </div>
      <div style="
          border-right: 1px solid gray;
          border-top: 1px solid gray;
          text-align: center;
        ">
        六
      </div>
      <div style="
          border-right: 1px solid gray;
          border-top: 1px solid gray;
          text-align: center;
        ">
        日
      </div>
    </div>
    <div style="width: 100%; height: 100%; position: relative">
      <div class="month">
        <div class="day" v-for="day in days">
          <slot :day="day"></slot>
        </div>
      </div>
      <div class="monthz">
        <!-- :style="{background:(item%2==0)?'gray':'black',opacity:0.5}" -->
        <div class="dayz" v-for="item in 6">
          <slot name="z" :z="item"></slot>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import calendar from "js-calendar-converter";
import { onMounted, ref, watch } from "vue";
const days = ref([]);
const date = defineModel("date", {
  default: {
    year: new Date().getFullYear(),
    month: new Date().getMonth(),
  },
  type: {
    year: Number,
    month: Number,
  },
});

onMounted(() => {
  days.value = monthdays(date.value.year, date.value.month);
});

watch(date.value, () => {
  days.value = monthdays(date.value.year, date.value.month);
  console.log(days.value);
});

const monthdays = function (year, month) {
  let days = [];
  let now = new Date();
  now.setFullYear(year);
  now.setMonth(month - 1);
  now.setDate(1);
  let week = getWeekday(year, month - 1, 1);
  now.setDate(now.getDate() - week);
  for (let i = 0; i < 42; i++) {
    now.setDate(now.getDate() + 1);
    let fes = getFestival(now.getFullYear(), now.getMonth() + 1, now.getDate());
    days.push(fes);
  }
  return days;
};

const getMonthDays = function (year, month) {
  return new Date(year, month, 0).getDate();
};

const getFestival = function (year, month, day) {
  return calendar.solar2lunar(year, month, day);
};

const getWeekday = function (year, month, day) {
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
  grid-template-rows: repeat(6, 1fr);
  grid-template-columns: repeat(7, 1fr);
  position: relative;
}

.monthz {
  position: absolute;
  left: 0;
  top: 0;
  z-index: 99;
  width: 100%;
  height: 100%;
  display: grid;
  grid-template-rows: repeat(6, 1fr);
  grid-template-columns: 1r;
}

.day:nth-child(7n + 1) {
  border-left: 1px solid gray;
}

.day:nth-child(-n + 7) {
  border-top: 1px solid gray;
}

.day {
  font-size: 13px;
  border-right: 1px solid gray;
  border-bottom: 1px solid gray;
}

.dayz {
  width: 100%;
  height: 100%;
  font-size: 13px;
}
</style>
