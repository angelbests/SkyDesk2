<script setup lang="ts">
import { onMounted, ref, toRefs, watch } from "vue";
import { get_pe_ico, get_uwp, setIcon2 } from "../../functions/peIcon";
import { convertFileSrc } from "@tauri-apps/api/core";
import { systemStore } from "../../stores/system";
import { shortcutStore } from "../../stores/shortcut";
import { VueDraggable } from "vue-draggable-plus";
import { createWindow } from "../../functions/window";
import { uuid } from "../../functions";
import { open } from "@tauri-apps/plugin-dialog";
import { exec } from "../../functions/open";
import { basename } from "@tauri-apps/api/path";
import GridContainer from "../../components/GridContainer.vue";
import { emit } from "@tauri-apps/api/event";
import { ShortCut } from "../../types/storeType";
import { DragDropEvent, getCurrentWebviewWindow, WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { exists } from "@tauri-apps/plugin-fs";
import { LogicalPosition, LogicalSize } from "@tauri-apps/api/dpi";
const systemstore = systemStore();
const tab = ref();
const tabshow = ref(false);
const tabtitle = ref("");
const deltabshow = ref(false);
const dialog2 = ref(false);
const dialog = ref(false);
const index = ref(0);
const setting = ref(false);
const selecttype = ref<string>("file")
const selectmuti = ref<'single' | 'multiple'>('single')
const shortcut = ref<ShortCut>({
  type: "openPath",
  lnkPath: "",
  icoPath: "",
  name: "",
  targetPath: "",
});
const shortcutstore = shortcutStore();
const { shortcuts, shortcutsTemp, wheels } = toRefs(shortcutstore);
const scanbtn = ref<boolean>(true);
const scanbar = ref(false);
onMounted(async () => {
  if (shortcutstore.shortcutsTemp.length == 0) {
    scanProgram()
  }
  window.addEventListener("storage", (e) => {
    if (e.key == "system") {
      systemstore.$hydrate();
    }
    if (e.key == "shortcut") {
      shortcutstore.$hydrate();
    }
  });


});

const scanProgram = async function () {
  if (scanbtn.value) {
    scanbtn.value = false;
    if (!scanbar.value) {
      scanbar.value = true;
      let arr = await get_uwp();
      for (let i = 0; i < arr.length; i++) {
        console.log(arr[i].lnkPath)
      }
      let res = await setIcon2();
      res.push(...arr)
      shortcutsTemp.value = res
      const map = new Map();
      res.filter((v) => !map.has(v.lnkPath) && map.set(v.lnkPath, v));
      scanbar.value = false;
    }
  } else {
    scanbtn.value = true;
    scanbar.value = false;
  }
};

const settingbtn = function () {
  setting.value = !setting.value;
};

const delSshorcut = function (item: any) {
  let index = shortcuts.value[tab.value].shortcut.indexOf(item);
  shortcuts.value[tab.value].shortcut.splice(index, 1);
};

//#region  创建桌面docker
const createDocker = async function () {
  let label = "shortcut-" + uuid();
  let data = {
    label: label,
    setting: {
      w: 120,
      h: 120,
      c: 2,
      r: 2,
      background: "white",
      blur: 0,
      font: false,
      alwaysOnTop: false,
      alwaysOnBottom: false,
    },
    shortcuts: [],
  };
  localStorage.setItem(label, JSON.stringify(data));
  await createWindow(label, {
    x: 100,
    y: 200,
    width: 120,
    height: 120,
    decorations: false,
    transparent: true,
    dragDropEnabled: false,
    shadow: false,
    alwaysOnBottom: true,
    maximizable: false,
    resizable: false,
    skipTaskbar: true,
    url: "/#/pages/desktop/shortcut?label=" + label,
    title: "shorcut"
  });
};
//#endregion

// 获取快捷文件并自动生成名称和图标
const getlnk = async function () {
  let res = await open({
    "directory": selecttype.value == "file" ? false : true,
    "multiple": selectmuti.value == 'multiple' ? true : false,
    "title": "选择文件",
    filters: [
      {
        extensions: ["lnk", "exe", "url"],
        name: "程序文件",
      },
      {
        extensions: ["*"],
        name: "任何文件"
      }
    ],
  });
  if (selectmuti.value == 'multiple') {
    if (res) {
      let arr: ShortCut[] = []
      for (let i = 0; i < res.length; i++) {
        let name = await basename(res[i])
        name = name.split(".")[0]
        let icoPath = await get_pe_ico(res[i])
        arr.push({
          type: "openPath",
          targetPath: res[i],
          lnkPath: res[i],
          icoPath,
          name,
        })
      }
      shortcuts.value[tab.value].shortcut.push(...arr);
      cancelsubmit();
    }
  } else {
    if (res) {
      let name = await basename(res)
      shortcut.value.name = name.split(".")[0]
      shortcut.value.icoPath = await get_pe_ico(res)
      shortcut.value.lnkPath = res;
    }
  }
};
// 用户选择图标
const getico = async function () {
  let res = await open({
    filters: [
      {
        extensions: ["ico", "png"],
        name: "Image",
      },
    ],
  });
  if (res) {
    shortcut.value.icoPath = res;
  }
};

// 修改shortcut 显示dialog
const editshortcut = function (i: any) {
  dialog.value = true;
  index.value = i;
  shortcut.value = {
    ...shortcuts.value[tab.value].shortcut[index.value],
  };
};

// 提交修改
const submitshortcut = async function () {
  if (
    shortcut.value.lnkPath &&
    shortcut.value.icoPath &&
    shortcut.value.name
  ) {
    shortcuts.value[tab.value].shortcut[index.value].type = "openPath",
      shortcuts.value[tab.value].shortcut[index.value].lnkPath =
      shortcut.value.lnkPath;
    shortcuts.value[tab.value].shortcut[index.value].name = shortcut.value.name;
    shortcuts.value[tab.value].shortcut[index.value].icoPath = shortcut.value.icoPath;
  }
  cancelsubmit();
};

// 新增shorcut 提交
const submitshortcut2 = async function () {
  if (
    shortcut.value.lnkPath &&
    shortcut.value.name &&
    shortcut.value.icoPath
  ) {
    shortcuts.value[tab.value].shortcut.push({
      type: "openPath",
      lnkPath: shortcut.value.lnkPath,
      icoPath: shortcut.value.icoPath,
      name: shortcut.value.name,
      targetPath: shortcut.value.targetPath
    });
  }
  cancelsubmit();
};

// 清除输入
const cancelsubmit = function () {
  dialog2.value = false;
  dialog.value = false;
  shortcut.value = {
    type: "openPath",
    lnkPath: "",
    icoPath: "",
    name: "",
    targetPath: ""
  };
  selecttype.value = "file"
  selectmuti.value = 'single'
};

// 拖拽窗口
let dragwindow: WebviewWindow = new WebviewWindow("dragfile", {
  url: "/#/pages/desktop/dragfile",
  shadow: false,
  resizable: false,
  maximizable: false,
  decorations: false,
  transparent: true,
  parent: 'main',
  visible: false,
})

dragwindow.listen<DragDropEvent>("tauri://drag-drop", async (e) => {
  console.log(e)
  if (e.payload && Array.isArray((e.payload as any).paths) && (e.payload as any).paths.length > 0) {
    dragwindow.hide();
    cancelsubmit();
    const paths = (e.payload as any).paths;
    let arr: ShortCut[] = []
    for (let i = 0; i < paths.length; i++) {
      if (!(await exists(paths[i]))) continue;
      let name = await basename(paths[i])
      name = name.split(".")[0]
      let icoPath = await get_pe_ico(paths[i])
      arr.push({
        type: "openPath",
        targetPath: paths[i],
        lnkPath: paths[i],
        icoPath,
        name,
      });
    }
    shortcuts.value[tab.value].shortcut.push(...arr)
  }
})


watch(dialog2, async () => {
  if (!dialog2.value) {
    await dragwindow.hide()
  } else {
    await dragwindow.show()
    let position = await getCurrentWebviewWindow().outerPosition()
    let factor = await getCurrentWebviewWindow().scaleFactor()
    let dom = document.getElementById("dragfile");
    if (!dom) return
    let { left, top, right, bottom } = dom.getBoundingClientRect()
    await dragwindow.setPosition(new LogicalPosition(Math.ceil(position.x / factor + left) + 13, Math.ceil(position.y / factor + top) - 13))
    await dragwindow.setSize(new LogicalSize(right - left + 4, bottom - top + 20))
  }
})

//#region ///////////////拖拽//////////////////////////////////////////
// 设置拖拽附带的数据
const setdata = function (d: DataTransfer, h: HTMLElement) {
  console.log(h.dataset.lnk);
  if (h.dataset.lnk) {
    d.setData("lnk", h.dataset.lnk);
  }
};
//克隆
const clone = function (element: any) {
  if (wheels.value.length < 8) {
    return {
      ...element,
    };
  }
};

// 左边拖拽到右键
const clonelefttoright = function (element: any): any {
  return {
    ...element,
  };
};
//拖拽到下方wheel
const adddown = function () {
  let map = new Map();
  wheels.value = wheels.value.filter(
    (v) => !map.has(v.name) && map.set(v.name, v)
  );
};

// 拖拽到右边
const addright = function () {
  let map = new Map();
  shortcuts.value[tab.value].shortcut = shortcuts.value[
    tab.value
  ].shortcut.filter((v) => !map.has(v.name) && map.set(v.name, v));
};

// 拖拽回左边
const addleft = function () {
  let map = new Map();
  shortcutsTemp.value = shortcutsTemp.value.filter(
    (v) => !map.has(v.name) && map.set(v.name, v)
  );
};

const dragover = function (e: DragEvent) {
  e.preventDefault();
};

// 拖拽放下
const drop = function (e: DragEvent) {
  if (e.dataTransfer?.getData("dellnk")) {
    let lnk = JSON.parse(e.dataTransfer.getData("dellnk"));
    emit("dellnk", lnk);
  }
};

//#endregion

//#region 选项卡
// 添加选项卡
const addtab = function () {
  if (tabtitle.value) {
    shortcuts.value.push({
      title: tabtitle.value,
      index: shortcuts.value.length,
      shortcut: [],
    });
    tab.value = shortcuts.value.length - 1;
    tabshow.value = false;
  }
};

// 删除选项卡
const deltab = function () {
  shortcuts.value.splice(tab.value, 1);
  tab.value = shortcuts.value.length - 1;
  for (let i = 0; i < shortcuts.value.length; i++) {
    shortcuts.value[i].index = i;
  }
  deltabshow.value = false;
};
//#endregion

</script>

<template>
  <div id="shortcut" style="width: 100%; height: 100%; position: relative">
    <!-- 删除合集 -->
    <v-dialog max-width="500" v-model="deltabshow">
      <v-card>
        <v-card-text>
          删除合集
        </v-card-text>
        <v-card-actions>
          <v-btn @click="deltabshow = false"> 取消 </v-btn>
          <v-btn @click="deltab"> 确认 </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <!-- 添加合集 -->
    <v-dialog max-width="500" v-model="tabshow">
      <v-card>
        <v-list>
          <v-list-item>
            <v-text-field v-model="tabtitle" density="compact" hide-details="auto" label="合集名称"></v-text-field>
          </v-list-item>
        </v-list>
        <v-card-actions>
          <v-btn style="margin-right: 10px" @click="tabshow = false">取消</v-btn>
          <v-btn style="margin-right: 10px" @click="addtab">确认</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <!-- 修改快捷 -->
    <v-dialog max-width="500" v-model="dialog">
      <v-card>
        <v-list>
          <v-list-item>
            <v-radio-group v-model="selecttype" inline density="compact" hide-details="auto">
              <v-radio style="width: 100px;" label="文件" value="file"></v-radio>
              <v-radio style="width: 100px;" label="文件夹" value="dir"></v-radio>
            </v-radio-group>
          </v-list-item>
          <v-list-item>
            <v-text-field v-model="shortcut.lnkPath" density="compact" hide-details="auto" :readonly="true"
              @click="getlnk" :label="selecttype == 'file' ? '选择文件' : '选择文件夹'"></v-text-field>
          </v-list-item>
          <v-list-item>
            <v-text-field v-model="shortcut.icoPath" density="compact" hide-details="auto" :readonly="true"
              @click="getico" label="图标路径"></v-text-field>
          </v-list-item>
          <v-list-item>
            <v-text-field v-model="shortcut.name" density="compact" hide-details="auto" label="快捷名称"></v-text-field>
          </v-list-item>
        </v-list>
        <v-card-actions>
          <v-btn style="margin-right: 10px" @click="cancelsubmit">取消</v-btn>
          <v-btn style="margin-right: 10px" @click="submitshortcut">确认</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <!-- 添加快捷 -->
    <v-dialog max-width="500" height="350" v-model="dialog2">
      <v-card style="width: 100%;height: 100%;">
        <v-card-text>
          <div style="display: flex;flex-direction: row;width: 100%;height: 100%;">
            <v-list style="width: 350px;">
              <v-list-item>
                <v-radio-group v-model="selecttype" inline density="compact" hide-details="auto">
                  <v-radio style="width: 100px;" label="文件" value="file"></v-radio>
                  <v-radio style="width: 100px;" label="文件夹" value="dir"></v-radio>
                </v-radio-group>
              </v-list-item>
              <v-list-item>
                <v-radio-group v-model="selectmuti" inline density="compact" hide-details="auto">
                  <v-radio style="width: 100px;" label="单选" value="single"></v-radio>
                  <v-radio style="width: 100px;" label="多选" value="multiple"></v-radio>
                </v-radio-group>
              </v-list-item>
              <v-list-item>
                <v-text-field v-model="shortcut.lnkPath" density="compact" hide-details="auto" :readonly="true"
                  @click="getlnk" :label="selecttype == 'file' ? '选择文件' : '选择文件夹'"></v-text-field>
              </v-list-item>
              <v-list-item v-show="selectmuti == 'single'">
                <v-text-field v-model="shortcut.icoPath" density="compact" hide-details="auto" :readonly="true"
                  @click="getico" label="图标路径"></v-text-field>
              </v-list-item>
              <v-list-item v-show="selectmuti == 'single'">
                <v-text-field v-model="shortcut.name" density="compact" hide-details="auto" label="快捷名称"></v-text-field>
              </v-list-item>
            </v-list>
            <div id="dragfile" style="width: 150px;height: 90%;margin-top: 3%;
             background-image: linear-gradient(to top, #cfd9df 0%, #e2ebf0 100%);
             display: flex;justify-content: center;align-items: center;color: gray;">
              拖入
            </div>
          </div>
        </v-card-text>
        <v-card-actions>
          <v-btn style="margin-right: 10px" @click="cancelsubmit">取消</v-btn>
          <v-btn style="margin-right: 10px" @click="submitshortcut2">确认</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <v-card :style="{
      background: systemstore.btnbarbackground,
      backgroundSize: 'cover',
    }" class="btnbar">
      <v-btn style="margin-right: 20px" @click="scanProgram">
        <template v-slot:prepend>
          <v-icon>mdi-magnify-scan</v-icon>
        </template>
        {{ scanbtn ? "扫描" : "关闭" }}
      </v-btn>
      <v-btn @click="dialog2 = true" style="margin-right: 20px">
        <template v-slot:prepend>
          <v-icon>mdi-view-grid-plus-outline</v-icon>
        </template>
        添加
      </v-btn>
      <v-btn @click="settingbtn" style="margin-right: 20px">
        <template v-slot:prepend>
          <v-icon>mdi-cog-outline</v-icon>
        </template>
        设置
      </v-btn>
      <v-btn @click="tabshow = true" style="margin-right: 20px">
        <template v-slot:prepend>
          <v-icon>mdi-tab-plus</v-icon>
        </template>
        合集
      </v-btn>
      <v-btn @click="createDocker">
        <template v-slot:prepend>
          <v-icon>mdi-apps</v-icon>
        </template>
        桌面合集
      </v-btn>
    </v-card>
    <v-progress-linear color="black" :indeterminate="scanbar"></v-progress-linear>
    <div @dragover="dragover($event)" @drop="drop($event)" style="
        width: 100%;
        height: calc(100% - 180px);
        display: flex;
        overflow: hidden;
      ">
      <!-- 左 -->
      <div :style="{ width: scanbtn ? '100%' : '50%' }" v-show="!scanbtn">
        <GridContainer v-model="shortcutsTemp" :animation="150" :group="{
          name: 'shortcut',
          pull: 'clone',
        }" :clone="clonelefttoright" :gridwidth="90" :gridheight="90" :onAdd="addleft" :setData="setdata">
          <template v-slot="{ item }">
            <div :style="{
              background: systemstore.shortcutbackground,
              backgroundSize: 'cover',
            }" :key="item.lnkPath" class="shortcut-container">
              <div class="icon-div">
                <img class="icon" :src="item.icoPath == ''
                  ? '/icons/program.png'
                  : convertFileSrc(item.icoPath)
                  " />
              </div>
              <div style="
                  font-size: 10px;
                  width: 60px;
                  height: 30px;
                  text-wrap: balance;
                  text-align: center;
                  text-overflow: clip;
                  overflow: hidden;
                  line-height: 15px;
                  filter: drop-shadow(0px 5px 5px gray);
                ">
                {{ item.name }}
              </div>
            </div>
          </template>
        </GridContainer>
      </div>
      <!-- 右 -->
      <div :style="{
        width: scanbtn ? '100%' : '50%',
        height: '100%',
        transition: 'all 0.1s linear',
      }">
        <v-tabs id="shortcuttab" density="compact" v-model="tab" center-active style="
            height: 36px;
            width: 100%;
            background: rgba(223, 223, 223, 0.4);
          " hide-slider show-arrows>
          <v-tab v-for="item in shortcuts" :value="item.index">
            <div style="display: flex; justify-content: center">
              <div style="width: calc(100% - 20px); margin-right: 10px">
                {{ item.title }}
              </div>
              <v-btn v-show="setting" @click="deltabshow = true" icon size="mini" style="
                  width: 20px;
                  height: 20px;
                  box-shadow: none;
                  background: transparent;
                ">
                <v-icon color="white" size="mini">mdi-close</v-icon>
              </v-btn>
            </div>
          </v-tab>
        </v-tabs>
        <div style="width: 100%; height: calc(100% - 36px)">
          <v-tabs-window v-model="tab" style="width: 100%; height: 100%; padding: 10px">
            <v-tabs-window-item v-for="item1 in shortcuts" style="width: 100%; height: 100%; min-height: 100%">
              <GridContainer v-model="item1.shortcut" :animation="150" :gridwidth="90" :gridheight="setting ? 120 : 90"
                :group="{ name: 'shortcut', pull: 'clone' }" :setData="setdata" :clone="clone" :onAdd="addright">
                <template v-slot="{ item, index }">
                  <div :style="{
                    height: setting ? '110px' : '80px',
                    background: systemstore.shortcutbackground,
                    backgroundSize: 'cover',
                  }" class="shortcut-container">
                    <div class="icon-div" @click="exec(item)">
                      <img class="icon" :src="item.icoPath == ''
                        ? '/icons/program.png'
                        : convertFileSrc(item.icoPath)
                        " />
                    </div>
                    <div style="
                        font-size: 10px;
                        width: 60px;
                        height: 30px;
                        filter: drop-shadow(0px 5px 5px gray);
                        text-wrap: balance;
                        text-align: center;
                        text-overflow: clip;
                        overflow: hidden;
                      ">
                      {{ item.name }}
                    </div>
                    <div v-show="setting" style="
                        width: 80px;
                        height: 20px;
                        display: flex;
                        justify-content: space-around;
                        align-items: flex-end;
                      ">
                      <v-btn @click="delSshorcut(item)" style="
                          font-size: 12px;
                          box-shadow: none;
                          background: none;
                          color: gray;
                        " size="mini">
                        <template v-slot:prepend>
                          <v-icon>mdi-delete-outline</v-icon>
                        </template>
                        删
                      </v-btn>
                      <v-btn @click="editshortcut(index)" style="
                          font-size: 10px;
                          box-shadow: none;
                          background: none;
                          color: gray;
                        " size="mini">
                        <template v-slot:prepend>
                          <v-icon>mdi-pencil-box-outline</v-icon>
                        </template>
                        改
                      </v-btn>
                    </div>
                  </div>
                </template>
              </GridContainer>
            </v-tabs-window-item>
          </v-tabs-window>
        </div>
      </div>
    </div>
    <!-- 下 -->
    <VueDraggable v-model="shortcutstore.wheels" :group="{ name: 'shortcut' }" :animation="150"
      class="VueDraggable-wheel" :onAdd="adddown" :setData="setdata">
      <div :style="{
        background: systemstore.shortcutbackground,
        backgroundSize: 'cover',
      }" v-for="item in shortcutstore.wheels" :key="item.name" :data-lnk="JSON.stringify(item)"
        class="shortcut-container">
        <div class="icon-div">
          <img class="icon" :src="item.icoPath == ''
            ? '/icons/program.png'
            : convertFileSrc(item.icoPath)
            " />
        </div>
        <div style="
            font-size: 10px;
            width: 70px;
            height: 30px;
            filter: drop-shadow(0px 5px 5px gray);
            text-wrap: balance;
            text-align: center;
            text-overflow: clip;
            overflow: hidden;
            line-height: 15px;
          ">
          {{ item.name }}
        </div>
      </div>
    </VueDraggable>
  </div>
</template>

<style>
.window {
  width: 100%;
  height: 100%;
  position: relative;
}

.btnbar {
  width: 100%;
  height: 60px;
  display: flex;
  align-items: center;
  box-sizing: border-box;
  padding: 0 20px;
  filter: drop-shadow(0px 2px 5px gray);
  background: transparent;
}

.scan-container {
  height: calc(100% - 120px);
  display: flex;
  flex-wrap: wrap;
  overflow: scroll;
  box-sizing: border-box;
  padding: 10px;
  transition: all 0.1s linear;
}

.shorcuts-container {
  height: calc(100% - 120px);
  display: flex;
  flex-wrap: wrap;
  overflow: scroll;
  box-sizing: border-box;
  padding: 10px;
  transition: all 0.1s linear;
  position: relative;
}

.VueDraggable {
  width: 100%;
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  align-content: flex-start;
}

.VueDraggable-wheel {
  width: 100%;
  height: 120px;
  position: absolute;
  z-index: 1000;
  left: 0px;
  bottom: 0px;
  background: rgba(223, 223, 223, 0.2);
  display: flex;
  justify-content: center;
  align-items: center;
}

.icon-div {
  width: 40px;
  height: 40px;
  display: flex;
  justify-content: center;
  filter: drop-shadow(0px 5px 5px gray);
}

.icon {
  width: 35px;
  height: 35px;
  border-radius: 5px;
  transition: all 0.1s linear;
}

.icon:hover {
  width: 40px;
  height: 40px;
}

.shortcut-container {
  width: 80px;
  height: 80px;
  margin: 5px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  border-radius: 10px;
  box-shadow: 0px 0px 15px 2px rgba(223, 223, 223, 0.2);
  transition: height 0.1s linear;
}
</style>
