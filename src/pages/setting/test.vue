<template>
  <!-- <av-bars src="" bar-color="#CCC"> </av-bars> -->
</template>
<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
listen("audio_chunk", (e: { payload: number[] }) => {
  console.log(e.payload);
});
</script>
<style scoped></style>
