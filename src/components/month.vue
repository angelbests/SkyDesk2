<template>        
    <div style="display: flex;flex-direction: column;width: 100%;height: 100%;box-sizing: border-box;padding: 10px;">
        <div style="display: grid;width: 100%;grid-template-columns:repeat(7,1fr);grid-template-rows: repeat(1,1fr);">
            <div style="border:1px solid gray;border-bottom: none;text-align: center;">
                星期一
            </div>
            <div style="border-right: 1px solid gray;border-top: 1px solid gray;text-align: center;">
                星期二
            </div>
            <div style="border-right: 1px solid gray;border-top: 1px solid gray;text-align: center;">
                星期三
            </div>
            <div style="border-right: 1px solid gray;border-top: 1px solid gray;text-align: center;">
                星期四
            </div>
            <div style="border-right: 1px solid gray;border-top: 1px solid gray;text-align: center;">
                星期五
            </div>
            <div style="border-right: 1px solid gray;border-top: 1px solid gray;text-align: center;">
                星期六
            </div>
            <div style="border-right: 1px solid gray;border-top: 1px solid gray;text-align: center;">
                星期日
            </div>
        </div>
        <div class="month">
            <div class="day" v-for="day in days">
                <div :style="{textAlign: 'center',background:(new Date().getFullYear() == day.cYear)&&(new Date().getMonth()+1 == day.cMonth)&&(new Date().getDate() == day.cDay)?'rgba(133,133,133,1)':'rgba(233,233,233,1)'}" >
                    {{ day.cDay }}
                    {{ day.IMonthCn }}
                    {{ day.IDayCn }}
                    <span style="color: blue;">
                        {{ day.lunarFestival }}
                        {{ day.festival }}
                        {{ day.Term }}
                    </span>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import calendar from  "js-calendar-converter"
import { onMounted, ref, watch } from "vue";
const days = ref([])
const date = defineModel('date',{
    "default":{
        year:new Date().getFullYear(),
        month:new Date().getMonth()
    },
    "type":{
        year:Number,
        month:Number
    }
})

onMounted(()=>{
    days.value = monthdays(date.value.year,date.value.month)
})

watch(date.value,()=>{
   days.value = monthdays(date.value.year,date.value.month)
   console.log(days.value)
})

const monthdays = function(year,month){
    let days = []
    let now =new Date();
    now.setFullYear(year)
    now.setMonth(month - 1)
    now.setDate(1);
    let week = getWeekday(year,month-1,1);
    now.setDate(now.getDate()-week);
    for(let i=0;i<42;i++){
        now.setDate(now.getDate()+1)
        let fes = getFestival(now.getFullYear(),now.getMonth()+1,now.getDate())
        days.push(fes)
    }
    return days;
}

const getMonthDays = function (year, month) {
    return new Date(year, month, 0).getDate()
}

const getFestival = function(year,month,day){
    return calendar.solar2lunar(year,month,day)
}

const getWeekday = function (year, month,day) {
    let week = new Date(year, month, day).getDay()
    if(week == 0){
        week = 7
    }
    return week
}
</script>

<style>
.month {
    width: 100%;
    height: 100%;
    display: grid;
    grid-template-rows: repeat(6, 1fr);
    grid-template-columns: repeat(7, 1fr);
}
.day:nth-child(7n+1){
    border-left: 1px solid gray;
}
.day:nth-child(-n + 7){
    border-top: 1px solid gray;
}
.day {
    font-size: 13px;
    border-right: 1px solid gray;
    border-bottom: 1px solid gray;
}
</style>