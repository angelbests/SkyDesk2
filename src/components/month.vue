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
            <div class="day" v-for="item in lastmonth()">
                <div style="text-align: center;background-color: rgba(233,233,233,1);">
                    {{ item<10?'0'+item:item }} 
                </div>
            </div>
            <div class="day" v-for="item in getMonthDays(year, month)">
                <div style="text-align: center;background-color: rgba(233,233,233,1);">
                    {{ item<10?'0'+item:item }} 
                    {{ getFestival(year,month,item).festival }}
                    {{ getFestival(year,month,item).lunarFestival }}
                    {{ getFestival(year,month,item).IMonthCn }}
                    {{ getFestival(year,month,item).IDayCn }}
                </div>
            </div>
            <div class="day" v-for="item in nextmonth()">
                <div style="text-align: center;background-color: rgba(233,233,233,1);">
                    {{ item<10?'0'+item:item }} 
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import calendar from  "js-calendar-converter"

const month = defineModel('month',{
    "default":10,
    "type":Number
})
const year = defineModel('year',{
    "default":2024,
    "type":Number
})

const lastmonth = function(){
    let days = getWeekday(year.value,month.value,1)-1
    return days
}

const nextmonth = function(){
    let days = getWeekday(year.value,month.value,getMonthDays(year.value,month.value))
    return 7-days
}

const getMonthDays = function (year, month) {
    return new Date(year, month, 0).getDate()
}

const getFestival = function(year,month,day){
    return calendar.solar2lunar(year,month,day)
}

const getWeekday = function (year, month,day) {
    return new Date(year, month-1, day).getDay()
}


</script>

<style>
.month {
    width: 100%;
    height: 100%;
    display: grid;
    grid-template-rows: repeat(5, 1fr);
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