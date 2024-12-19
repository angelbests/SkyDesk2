<script setup lang="ts">
import { PhysicalPosition } from "@tauri-apps/api/dpi";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { onMounted, reactive, ref } from "vue";
import { convertFileSrc } from "@tauri-apps/api/core";
import { Command } from "@tauri-apps/plugin-shell";
import { shortcutStore } from "../../stores/window";
const shortcutstore = shortcutStore();
const wheel = ref<any[]>([]);
const app = getCurrentWebviewWindow();
app.setShadow(false);
const position = reactive({
  x: 0,
  y: 0,
});
onMounted(async () => {
  updatewheel();
  window.addEventListener("storage", (e) => {
    console.log(e);
    if (e.key == "shortcut") {
      shortcutstore.$hydrate();
      updatewheel();
    }
  });
});

const updatewheel = () => {
  wheel.value.splice(0);
  for (let i = 0; i < 8; i++) {
    if (shortcutstore.wheels[i] != undefined) {
      wheel.value.push(shortcutstore.wheels[i]);
    } else {
      wheel.value.push({
        targetPath: "",
        iconLocationPeFile: "",
        iconLocation: "",
        lnkPath: "",
        icoPath: "",
        name: "",
      });
    }
  }
};

listen("wheel-click", async (e: any) => {
  let scaleFactor = await getCurrentWebviewWindow().scaleFactor();
  if (e.payload.message == "ButtonRelease(Middle)") {
    await app.hide();
  } else if (e.payload.message == "ButtonPress(Middle)") {
    await app.setFocus();
    await app.setPosition(
      new PhysicalPosition(
        Math.trunc(position.x - 120 * scaleFactor),
        Math.trunc(position.y - 120 * scaleFactor)
      )
    );
    await app.show();
  }
});

listen("mouse-move", (e: { payload: { message: string } }) => {
  let mouse = JSON.parse(e.payload.message as string);
  position.x = mouse.x;
  position.y = mouse.y;
});

const index = ref(-2);
const exec = async function (item: any, i: number) {
  index.value = i;

  if (item.name) {
    if (item.lnkPath) {
      await Command.create("powershell", `& "${item.lnkPath}"`, {
        encoding: "GBK",
      }).execute();
    } else {
      await Command.create("powershell", item.targetPath).execute();
    }
    app.hide();
  }
  index.value = -2;
};
</script>

<template>
  <svg
    style=""
    width="240"
    height="240"
    viewBox="-120 -120 240 240"
    xmlns="http://www.w3.org/2000/svg"
    transform="rotate(22.5)"
  >
    <g
      @mouseenter="exec(item, i)"
      v-for="(item, i) in wheel"
      :transform="'rotate(' + 45 * (1 + i) + ')'"
    >
      <path
        d="M 0 0 L 120 0 A 120 120 0 0 1 85 85 Z"
        :fill="index == i ? 'rgba(123,123,123,0)' : 'rgba(123,123,123,0.1)'"
      />
    </g>
    <circle cx="0px" cy="0px" r="30px" fill="white" fill-opacity="1"></circle>
  </svg>
  <div class="container">
    <div
      v-for="(item, i) in wheel"
      @mouseenter="exec(item, i)"
      class="img-container"
      :style="{ transform: `rotate(${45 * i + 135}deg) translate(80px)` }"
    >
      <img
        v-if="item.icoPath ? true : false"
        class="img"
        :style="{ transform: `rotate(${-(90 + 45 * i)}deg)` }"
        :src="convertFileSrc(item.icoPath)"
      />
    </div>
  </div>
</template>

<style>
svg {
  position: absolute;
  z-index: 100;
  left: 0;
  top: 0;
}

.image {
  border-radius: 50%;
}

.container {
  width: 100vw;
  height: 100vh;
  border-radius: 50%;
  background: rgba(123, 123, 123, 0.3);
  position: relative;
  transform-origin: 120px 120px;
  transform: rotate(-45deg);
}

.img-container {
  width: 40px;
  height: 40px;
  transform-origin: 20px 20px;
  position: absolute;
  z-index: 100;
  left: 100px;
  top: 100px;
}

.img {
  width: 40px;
  height: 40px;
  transform-origin: 20px 20px;
  border-radius: 50%;
}
</style>
