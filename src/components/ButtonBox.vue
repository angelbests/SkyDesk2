<script setup lang="ts">
import { toRefs } from "vue";
const props = defineProps<{
  boxs: {
    text: string;
    value: string;
  }[];
}>();
const { boxs } = toRefs(props);
const value = defineModel<string>({
  required: true,
});

const checkbox = function (e: { text: string; value: string }) {
  value.value = e.value;
};
</script>

<template>
  <div class="div">
    <v-btn v-for="item in boxs" class="button" size="small"
      :style="{ background: value == item.value ? '#5865f2' : 'white' }" @click="checkbox(item)">
      {{ item.text }}
    </v-btn>
  </div>
</template>

<style scoped>
.button {
  margin-right: 10px;
  width: 80px;
  font-size: 13px;
}

.div {
  display: flex;
  flex-direction: row;
}
</style>
