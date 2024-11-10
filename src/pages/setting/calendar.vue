<script setup lang="ts" >
import FullCalendar from '@fullcalendar/vue3'
import dayGridPlugin from '@fullcalendar/daygrid'
import interactionPlugin from '@fullcalendar/interaction'
import { ref } from 'vue';
import { CalendarOptions } from '@fullcalendar/core/index.js';
import zhcnLocale from '@fullcalendar/core/locales/zh-cn';
import moment from 'moment';
const calendarOptions = ref<CalendarOptions>({
    plugins: [ dayGridPlugin, interactionPlugin ],
    initialView: 'dayGridMonth',
    locale:zhcnLocale, 
    firstDay:1, // 设置一周中显示的第一天是哪天，周日是0，周一是1，类推
    eventColor: '#3BB2E3', // 全部日历日程背景色
    initialDate: moment().format('YYYY-MM-DD'), // 自定义设置背景颜色时一定要初始化日期时间
    eventMinHeight: 20, // 设置事件的最小高度
    headerToolbar: { // 日历头部按钮位置
        left: 'prevYear,prev',
        center: 'title',
        right: 'next,nextYear'
    },
    fixedWeekCount:true,
})
</script>

<template>
    <div class="window">
        <v-card
            style="width: 100%;height: 60px;display: flex;align-items: center;box-sizing: border-box;padding: 0 20px;filter:drop-shadow(0px 2px 5px gray)">
            <v-btn style="margin-right: 20px;">
                    <template v-slot:prepend>
                        <v-icon>mdi-clipboard-text-clock</v-icon>
                    </template>
                    新建日程
                </v-btn>
        </v-card>
        <v-progress-linear color="black" :indeterminate="false"></v-progress-linear>
        <div style="width: 100%;height: calc(100% - 60px);display: flex;overflow: hidden;background: white;flex-direction: row;">
            <FullCalendar style="width: 100%;height: 100%;" :options="calendarOptions" />
        </div>
    </div>
</template>

<style scoped>
.window{
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction:column
}
</style>