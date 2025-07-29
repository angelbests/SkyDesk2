<script setup lang="ts">
import { ref } from 'vue'
import { get_all_festival, Festival } from '../../functions/calendar'

const today = ref<{
  year: number
  month: number
  day: number
}>({
  year: new Date().getFullYear(),
  month: new Date().getMonth() + 1,
  day: new Date().getDate(),
})
const festivals = ref<Festival[]>([])
festivals.value = get_all_festival(today.value.year, today.value.month, today.value.day)
console.log(festivals)
</script>
<template>
  <div class="date">
    <div style="font-size: 20px; font-weight: 400; height: 20px">
      {{ festivals[0].festival }}
    </div>
    <div style="font-size: 40px">{{ festivals[0].until }}</div>
  </div>
</template>
<style>
.date {
  position: absolute;
  right: 20px;
  top: 20px;
  width: 100px;
  height: 100px;
  border-radius: 10px;
  background: white;
  color: black;
}
</style>
