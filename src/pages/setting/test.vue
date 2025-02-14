<template>
  <div style="width: 100%; height: 100%">
    <canvas id="canvas" width="600px" height="600px"></canvas>
  </div>
</template>

<script lang="ts" setup>
import { listen } from "@tauri-apps/api/event";
import PCMPlayer from "pcm-player";
import { onMounted } from "vue";
const player = new PCMPlayer({
  inputCodec: "Float32",
  channels: 2,
  sampleRate: 48000,
  flushTime: 100,
  fftSize: 512,
});
player.volume(0);
listen("audio_chunk", (e: { payload: number[] }) => {
  player.feed(new Uint8Array(e.payload));
  console.log(e.payload);
});

const bufferLength = player.analyserNode.frequencyBinCount / 2;
const dataArray = new Uint8Array(bufferLength);
function draw() {
  requestAnimationFrame(draw);
  // 时域数据
  // player.analyserNode.getByteTimeDomainData(dataArray);
  player.analyserNode.getByteFrequencyData(dataArray);
  // console.log(dataArray);
  const canvas = document.getElementById("canvas") as HTMLCanvasElement;
  if (canvas == null) {
    console.log("canvas error");
    return;
  }
  const canvasCtx = canvas.getContext("2d");
  if (canvasCtx == null) {
    console.log("canvasctx error");
    return;
  }

  const WIDTH = canvas.width,
    HEIGHT = canvas.height;
  canvasCtx.clearRect(0, 0, WIDTH, HEIGHT);
  canvasCtx.fillStyle = "rgba(220,220,220,1)";
  const angle = (Math.PI * 2) / bufferLength;
  canvasCtx.save();
  canvasCtx.translate(canvas.width / 2, canvas.height / 2);
  for (let i = 0; i < bufferLength; i++) {
    canvasCtx.save();
    canvasCtx.rotate(angle * i + Math.PI);
    canvasCtx.beginPath();

    const h = (dataArray[i] / 256) * 60;
    // canvasCtx.fillStyle = `rgba(${dataArray[i] * Math.random()},${
    //   dataArray[i] * Math.random()
    // },${dataArray[i] * Math.random()},0.8)`;
    canvasCtx.roundRect(0, 140, 4, h < 4 ? 4 : h, 4);
    canvasCtx.fill();
    canvasCtx.restore();
  }
  canvasCtx.restore();
}

onMounted(() => {
  draw();
});
</script>

<style></style>
