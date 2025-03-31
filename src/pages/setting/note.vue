<script setup lang="ts">
import { onMounted } from "vue";
import { uuid } from "../../functions";
import { createWindow } from "../../functions/window";
import { listen } from "@tauri-apps/api/event";
import { noteStore, systemStore, windowStore } from "../../stores/window";
import { getAllWebviewWindows, getCurrentWebviewWindow, } from "@tauri-apps/api/webviewWindow";
import GridContainer from "../../components/GridContainer.vue";
const notestore = noteStore();
const systemstore = systemStore();
window.addEventListener("storage", (e) => {
  if (e.key === "note") {
    notestore.$hydrate();
  }
  if (e.key === "system") {
    systemstore.$hydrate();
  }
});
onMounted(() => {
  listen("note", (e: any) => {
    let index = notestore.note.findIndex((item) => {
      return item.label == e.payload.label;
    });

    if (index >= 0) {
      notestore.note[index] = e.payload;
    } else {
      notestore.note.push(e.payload);
    }
  });
});

const createnote = async function () {
  let label = "note-" + uuid();
  let w = await createWindow(label, {
    x: 200,
    y: 200,
    width: 330,
    height: 330,
    minWidth: 330,
    minHeight: 100,
    shadow: false,
    decorations: false,
    transparent: true,
    skipTaskbar: true,
    url: "/#/pages/desktop/note",
  });
  w?.center();
};

const opennote = async function (item: { label: string }) {
  let note = notestore.note.filter((note) => {
    return note.label == item.label;
  });
  getAllWebviewWindows().then(async (res) => {
    let w = res.filter((window: any) => {
      return window.label == item.label;
    });
    if (w[0]) {
      w[0].setFocus();
    } else {
      if (note && note[0].option) {
        let w = await createWindow(note[0].label, note[0].option);
        w?.setFocus();
      }
    }
  });
};

//删除note
const delnote = function (item: { label: string }) {
  let index = notestore.note.findIndex((note) => {
    return item.label == note.label;
  });
  console.log(index);
  if (index >= 0) {
    localStorage.removeItem(notestore.note[index].label);
    notestore.note.splice(index, 1);
  }
  getAllWebviewWindows().then((res) => {
    res.filter((window: { label: string; close: () => void }) => {
      if (window.label == item.label) {
        window.close();
      }
    });
  });
};
import { setWindowToMonitor, cancelwallpaper } from "../../functions/monitor";
const notewallpaper = async function (note: any) {
  // 检查窗口是否存在
  let all = await getAllWebviewWindows();
  let index = all.findIndex((item) => {
    return item.label == note.label;
  });
  if (index >= 0) {
    let win = all[index];
    const windowstore = windowStore();
    if (note.wallpaper) {
      windowstore.windows.filter((item, index) => {
        if (item.label == win.label) {
          cancelwallpaper(
            win.label,
            windowstore.windows[index].wallpaper.x,
            windowstore.windows[index].wallpaper.y,
            windowstore.windows[index].wallpaper.w,
            windowstore.windows[index].wallpaper.h
          );
          windowstore.windows[index].wallpaper.status = false;
        }
      });
      notestore.note.filter((item, index) => {
        if (item.label == win.label) {
          notestore.note[index].wallpaper = false;
          localStorage.setItem(
            note.label,
            JSON.stringify(notestore.note[index])
          );
        }
      });
    } else if (!note.wallpaper) {
      let factor = await getCurrentWebviewWindow().scaleFactor();
      console.log(factor);
      windowstore.windows.filter((item, index) => {
        if (item.label == win.label) {
          windowstore.windows[index].wallpaper.x = Math.trunc(
            (windowstore.windows[index].option.x as number) * factor
          );
          windowstore.windows[index].wallpaper.y = Math.trunc(
            (windowstore.windows[index].option.y as number) * factor
          );
          windowstore.windows[index].wallpaper.w = Math.trunc(
            (windowstore.windows[index].option.width as number) * factor
          );
          windowstore.windows[index].wallpaper.h = Math.trunc(
            (windowstore.windows[index].option.height as number) * factor
          );
          windowstore.windows[index].wallpaper.z = 1;
          windowstore.windows[index].wallpaper.status = true;
          setWindowToMonitor(
            win.label,
            windowstore.windows[index].wallpaper.x,
            windowstore.windows[index].wallpaper.y,
            windowstore.windows[index].wallpaper.w,
            windowstore.windows[index].wallpaper.h,
            1
          );
        }
      });
      notestore.note.filter((item, index) => {
        if (item.label == win.label) {
          notestore.note[index].wallpaper = true;
          localStorage.setItem(
            note.label,
            JSON.stringify(notestore.note[index])
          );
        }
      });
    }
  }
};
</script>

<template>
  <div class="window">
    <v-card :style="{
      background: systemstore.btnbarbackground,
      backgroundSize: 'cover',
    }" class="btnbar">
      <v-btn style="margin-right: 20px" @click="createnote">
        <template v-slot:prepend>
          <v-icon>mdi-note-plus-outline</v-icon>
        </template>
        添加
      </v-btn>
    </v-card>
    <v-progress-linear color="black" :indeterminate="false"></v-progress-linear>
    <div style="
        width: 100%;
        height: calc(100% - 64px);
        display: flex;
        overflow: hidden;
        box-sizing: border-box;
        padding: 10px;
      ">
      <GridContainer style="width: 100%; height: 100%; min-height: 100%" v-model="notestore.note" :gridheight="240"
        :gridwidth="320" :padding="10">
        <template v-slot="{ item }">
          <v-card style="width: 100%; height: 100%" variant="elevated" elevation="5">
            <v-card-text :style="{
              width: '100%',
              height: '80%',
              background:
                'rgba(' + item.color + ',' + item.opacity / 100 + ')',
              boxSizing: 'border-box',
              padding: '10px',
              overflow: 'hidden',
              overflowY: 'scroll',
            }">
              <div style="width: 100%; height: auto" v-html="item.value"></div>
            </v-card-text>
            <v-card-actions style="height: 20%; padding: 0px 0px 0px 10px">
              <v-btn size="small" border="opacity-50 sm" @click="notewallpaper(item)">
                {{ item.wallpaper ? "映出" : "映入" }}
              </v-btn>
              <v-btn size="small" border="opacity-50 sm" @click="opennote(item)">
                编辑
              </v-btn>
              <v-btn size="small" border="opacity-50 sm" @click="delnote(item)">
                删除
              </v-btn>
            </v-card-actions>
          </v-card>
        </template>
      </GridContainer>
    </div>
  </div>
</template>

<style>
.window {
  width: 100%;
  height: 100%;
}

ol {
  list-style-position: inside !important;
}

.btnbar {
  width: 100%;
  height: 60px;
  display: flex;
  align-items: center;
  box-sizing: border-box;
  padding: 0 20px;
  filter: drop-shadow(0px 2px 5px gray);
}

.note {
  width: 100%;
  background: white;
  display: flex;
  justify-content: center;
  overflow-x: hidden;
  overflow-y: scroll;
}
</style>
