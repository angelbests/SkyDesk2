<template>
  <av-bars src="" bar-color="#CCC"> </av-bars>
</template>
<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { onMounted } from "vue";
listen("audio_chunk", (e: { payload: number[] }) => {
  playWavByteArray(new Uint8Array(e.payload as number[]));
});

function playWavByteArray(wavByteArray: Uint8Array<ArrayBuffer>) {
  var audioContext = new window.AudioContext();

  audioContext.decodeAudioData(wavByteArray.buffer, function (buffer) {
    var source = audioContext.createBufferSource();
    source.buffer = buffer;
    source.connect(audioContext.destination);
    source.start(0);
  });
}

// 示例音频字节数组

// 播放音频字节数组

onMounted(() => {});
</script>
<style scoped></style>
