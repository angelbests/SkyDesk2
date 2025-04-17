<script setup lang="ts">
import { ref, watch } from "vue"
// import { lunar2solar } from '../../functions/calendar'
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

// 选择项参数 // , { title: '农历每年', value: 5 }
const repeat = [
  { title: "不重复", value: 0 },
  { title: "每天", value: 1 },
  { title: "每周", value: 2 },
  { title: "每月", value: 3 },
  { title: "每年", value: 4 },
]
const reminder = [
  { title: "不提醒", value: 0 },
  { title: "准时", value: 1 },
  { title: "15 分钟前", value: 2 },
  { title: "30 分钟前", value: 3 },
  { title: "1 小时前", value: 4 },
  { title: "2 小时前", value: 5 },
  { title: "12 小时前", value: 6 },
  { title: "1 天前", value: 6 },
  { title: "2 天前", value: 7 },
  { title: "1 周前", value: 8 },
]
const weekday = [
  { title: "周一", value: 1 },
  { title: "周二", value: 2 },
  { title: "周三", value: 3 },
  { title: "周四", value: 4 },
  { title: "周五", value: 5 },
  { title: "周六", value: 6 },
  { title: "周天", value: 7 },
]
const gettimearr = function () {
  let arr = []
  for (let i = 0; i <= 23; i++) {
    let h = i < 10 ? "0" + i : i
    arr.push(h + ":00")
    arr.push(h + ":30")
  }
  return arr
}

const show = ref(false)

const schedules = ref<
  {
    start: {
      date: Date
      time: string
    }
    end: {
      date: Date
      time: string
    }
    week: number[]
    title: string
    text: string
    repeat: number
    clock: number
    address: string
    first: boolean
  }[]
>([])
const schedule = ref<{
  start: {
    date: Date
    time: string
  }
  end: {
    date: Date
    time: string
  }
  week: number[]
  title: string
  text: string
  repeat: number
  clock: number
  address: string
  first: boolean
}>({
  start: {
    date: new Date(),
    time: "00:00",
  },
  end: {
    date: new Date(),
    time: "00:00",
  },
  week: [],
  title: "",
  text: "",
  repeat: 0,
  clock: 0,
  address: "",
  first: false,
})

const submitschedule = function () {
  schedules.value.push(schedule.value)
  schedule.value = {
    start: {
      date: new Date(),
      time: "00:00",
    },
    end: {
      date: new Date(),
      time: "00:00",
    },
    week: [],
    title: "",
    text: "",
    repeat: 0,
    clock: 0,
    address: "",
    first: false,
  }
  show.value = false
}
const minend = ref()
const maxend = ref()
const filterschedule = function (day: { cYear: number; cMonth: number; cDay: number }) {
  // const repeat = [{ title: '不重复', value: 0 }, { title: '每天', value: 1 }, { title: '每周', value: 2 }, { title: '每月', value: 3 }, { title: '每年', value: 4 }, { title: '农历每年', value: 5 }]
  // const reminder = [{ title: '不提醒', value: 0 }, { title: '准时', value: 1 }, { title: '15 分钟前', value: 2 }, { title: '30 分钟前', value: 3 }, { title: '1 小时前', value: 4 }, { title: '2 小时前', value: 5 }, { title: '12 小时前', value: 6 }, { title: '1 天前', value: 6 }, { title: '2 天前', value: 7 }, { title: '1 周前', value: 8 }]
  let date = new Date(day.cYear, day.cMonth - 1, day.cDay)
  date.setHours(0)
  date.setMinutes(0)
  date.setSeconds(0)
  date.setMilliseconds(0)
  let arr = schedules.value.filter((item) => {
    let start = item.start.date
    start.setHours(Number(item.start.time.split(":")[0]))
    start.setMinutes(Number(item.start.time.split(":")[1]))
    start.setSeconds(0)
    start.setMilliseconds(0)
    let end = item.end.date
    end.setHours(Number(item.end.time.split(":")[0]))
    end.setMinutes(Number(item.end.time.split(":")[1]))
    end.setSeconds(0)
    end.setMilliseconds(0)
    if (date < start) {
      return false
    }
    if (item.repeat == 0) {
      if (start <= date && end >= date) {
        item.first = true
        return true
      } else {
        item.first = false
        return false
      }
    } else if (item.repeat == 1) {
      item.first = true
      return true
    } else if (item.repeat == 2) {
      let nowweek = date.getDay()
      if (nowweek == 0) {
        nowweek = 7
      }
      let index = item.week.indexOf(nowweek)
      if (index >= 0) {
        item.first = true
        return true
      } else {
        item.first = false
        return false
      }
    } else if (item.repeat == 3) {
      let daystart = start.getDate()
      let dayend = end.getDate()

      if (daystart < day.cDay && dayend > day.cDay) {
        item.first = false
        return true
      } else if (daystart == day.cDay && dayend == day.cDay) {
        item.first = true
        return true
      } else {
        item.first = false
        return false
      }
    } else if (item.repeat == 4) {
      let monthstart = start.getMonth()
      let monthend = start.getMonth()
      let daystart = start.getDate()
      let dayend = end.getDate()
      if (daystart < day.cDay && monthstart < day.cMonth - 1 && dayend > day.cDay && monthend > day.cMonth - 1) {
        item.first = false
        return true
      } else if (
        daystart == day.cDay &&
        monthstart == day.cMonth - 1 &&
        dayend == day.cDay &&
        monthend == day.cMonth - 1
      ) {
        item.first = true
        return true
      } else {
        item.first = false
        return false
      }
    }
  })
  return arr
}

watch(schedule.value, () => {
  if (schedule.value.repeat == 3) {
    let month = schedule.value.start.date.getMonth()
    let year = schedule.value.start.date.getFullYear()
    let days = new Date(year, month, 0).getDate()
    minend.value = new Date(year, month, 1)
    maxend.value = new Date(year, month, days)
  } else if (schedule.value.repeat == 4) {
    let year = schedule.value.start.date.getFullYear()
    let days = new Date(year, 11, 0).getDate()
    minend.value = new Date(year, 0, 1)
    maxend.value = new Date(year, 11, days)
  }
})

const startupdate = function (e: any) {
  console.log(e)
  if (schedule.value.start.date > schedule.value.end.date) {
    schedule.value.end.date = schedule.value.start.date
  }
  if (schedule.value.repeat == 1) {
    schedule.value.end.date = e
    minend.value = e
    maxend.value = e
  } else if (schedule.value.repeat == 2) {
  } else if (schedule.value.repeat == 3) {
    let month = e.getMonth()
    let year = e.getFullYear()
    let days = new Date(year, month, 0).getDate()
    minend.value = new Date(year, month, 1)
    maxend.value = new Date(year, month, days)
  } else if (schedule.value.repeat == 4) {
    let year = e.getFullYear()
    let days = new Date(year, 11, 0).getDate()
    minend.value = new Date(year, 0, 1)
    maxend.value = new Date(year, 11, days)
  }
}

const endupdate = function (e: any) {
  if (schedule.value.start.date > e) {
    schedule.value.end.date = schedule.value.start.date
  }
}
</script>

<template>
  <div class="window">
    <v-dialog
      v-model="show"
      style="width: 600px"
    >
      <v-card>
        <template v-slot:title>新建日程</template>
        <template v-slot:text>
          <v-list>
            <v-list-item>
              <v-select
                v-model="schedule.repeat"
                label="重复"
                density="compact"
                hide-details="auto"
                :items="repeat"
              ></v-select>
            </v-list-item>
            <v-list-item>
              <v-text-field
                v-model="schedule.title"
                label="标题"
                density="compact"
                hide-details="auto"
              ></v-text-field>
            </v-list-item>
            <v-list-item>
              <v-text-field
                v-model="schedule.address"
                label="地点"
                density="compact"
                hide-details="auto"
              ></v-text-field>
            </v-list-item>
            <v-list-item v-show="schedule.repeat == 2">
              <v-select
                v-model="schedule.week"
                density="compact"
                hide-details="auto"
                :items="weekday"
                label="周"
                multiple
              >
                <template v-slot:selection="{ item }">
                  <v-chip>
                    {{ item.title }}
                  </v-chip>
                </template>
              </v-select>
            </v-list-item>
            <v-list-item v-show="schedule.repeat == 2 || schedule.repeat == 1">
              <v-select
                v-model="schedule.start.time"
                style="width: 100%"
                density="compact"
                hide-details="auto"
                label="时间"
                :items="gettimearr()"
              ></v-select>
            </v-list-item>
            <v-list-item v-show="schedule.repeat != 2 && schedule.repeat != 1">
              <div style="width: 100%; display: flex">
                <v-date-input
                  v-model="schedule.start.date"
                  @update:model-value="startupdate"
                  cancel-text="取消"
                  ok-text="确定"
                  style="width: 50%"
                  density="compact"
                  hide-details="auto"
                  label="开始日期"
                  prepend-icon=""
                ></v-date-input>
                <v-select
                  v-model="schedule.start.time"
                  style="width: 50%"
                  density="compact"
                  hide-details="auto"
                  label="时间"
                  :items="gettimearr()"
                ></v-select>
              </div>
            </v-list-item>
            <v-list-item v-show="schedule.repeat != 2 && schedule.repeat != 1">
              <div style="width: 100%; display: flex">
                <v-date-input
                  :min="minend"
                  :max="maxend"
                  v-model="schedule.end.date"
                  @update:model-value="endupdate"
                  cancel-text="取消"
                  ok-text="确定"
                  style="width: 50%"
                  density="compact"
                  hide-details="auto"
                  label="结束日期"
                  prepend-icon=""
                ></v-date-input>
                <v-select
                  v-model="schedule.end.time"
                  style="width: 50%"
                  density="compact"
                  hide-details="auto"
                  label="时间"
                  :items="gettimearr()"
                ></v-select>
              </div>
            </v-list-item>
            <v-list-item>
              <v-select
                v-model="schedule.clock"
                label="提醒"
                density="compact"
                hide-details="auto"
                :items="reminder"
              ></v-select>
            </v-list-item>
            <v-list-item>
              <v-textarea
                v-model="schedule.text"
                density="compact"
                hide-details="auto"
                label="描述"
              ></v-textarea>
            </v-list-item>
          </v-list>
        </template>
        <template v-slot:actions>
          <v-btn @click="show = false">取消</v-btn>
          <v-btn @click="submitschedule">执行</v-btn>
        </template>
      </v-card>
    </v-dialog>
    <v-card
      @wheel.self="wheelmonth"
      :style="{
        background: systemstore.btnbarbackground,
        backgroundSize: 'cover',
      }"
      class="btnbar"
    >
      <div
        style="
          display: flex;
          flex-direction: row;
          justify-content: center;
          align-items: center;
          font-size: 20px;
          height: 100%;
          margin-right: 20px;
          width: 150px;
        "
      >
        <div
          @wheel="wheelyear"
          style="height: 100%; line-height: 300%"
        >
          {{ date.year }} 年
        </div>
        <div
          @wheel="wheelmonth"
          style="height: 100%; line-height: 300%"
        >
          {{ date.month < 10 ? "0" + date.month : date.month }} 月
        </div>
      </div>
      <!-- <v-btn style="margin-right: 20px;" @click="show = true">
                    <template v-slot:prepend>
                        <v-icon>mdi-clipboard-text-clock</v-icon>
                    </template>
                    新建日程
                </v-btn> -->
    </v-card>
    <v-progress-linear
      color="black"
      :indeterminate="false"
    ></v-progress-linear>
    <div
      @wheel="wheelmonth"
      style="width: 100%; height: calc(100% - 64px); display: flex; overflow: hidden"
    >
      <v-month v-model:date="date">
        <template v-slot:default="{ day }">
          <div
            :style="{
              width: '100%',
              height: '20px',
              textAlign: 'center',
              background:
                new Date().getFullYear() == day.cYear &&
                new Date().getMonth() + 1 == day.cMonth &&
                new Date().getDate() == day.cDay
                  ? 'rgba(133,133,133,1)'
                  : '',
            }"
          >
            {{ day.cDay }}
            {{ day.IMonthCn }}
            {{ day.IDayCn }}
            <span style="color: blue">
              {{ day.lunarFestival }}
              {{ day.festival }}
              {{ day.Term }}
            </span>
          </div>
          <div
            v-for="item in filterschedule(day)"
            style="background-color: bisque; width: 100%; height: calc(100% - 20px); height: 20px; margin-bottom: 2px"
          >
            {{ item.first ? item.start.time + " " + item.title : "" }}
          </div>
        </template>
        <!-- <template v-slot:z="{ z }">
                    <div style="width: 100%;height: 20px;"></div>
                        {{ z }}
                </template> -->
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
</style>
