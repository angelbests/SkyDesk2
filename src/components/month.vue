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
                <div style="text-align: center;background-color: rgba(233,233,233,1);">
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
import { onMounted, ref } from "vue";
const days = ref([])
const month = defineModel('month',{
    "default":1,
    "type":Number
})
const year = defineModel('year',{
    "default":2025,
    "type":Number
})
onMounted(()=>{
    lastmonth();
})
const lastmonth = function(){

    let now =new Date();
    now.setFullYear(year.value)
    now.setMonth(month.value - 1)
    now.setDate(1);
    console.log(now)
    let week = getWeekday(year.value,month.value-1,1);
    now.setDate(now.getDate()-week);
    let nowdays = getMonthDays(year.value, month.value);
    let nextweek = getWeekday(year.value,month.value-1,nowdays)
    console.log(week,nowdays,nextweek)
    for(let i=0;i<(week+nowdays+(6-nextweek)+7);i++){
        now.setDate(now.getDate()+1)
        let fes = getFestival(now.getFullYear(),now.getMonth()+1,now.getDate())
        days.value.push(fes)
    }
    console.log(days.value)
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