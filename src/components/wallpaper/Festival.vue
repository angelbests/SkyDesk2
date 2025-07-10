<script setup lang="ts">
import { ref } from 'vue'
import { get_all_festival } from '../../functions/calendar'

const today = ref<{
  year: number
  month: number
  day: number
}>({
  year: new Date().getFullYear(),
  month: new Date().getMonth() + 1,
  day: new Date().getDate(),
})
const festivals = get_all_festival(today.value.year, today.value.month, today.value.day)
console.log(festivals)
</script>
<template>
  <div class="date"></div>
</template>
<style></style>
