<script setup lang="ts">
import markdownit from "markdown-it"
import highlightjs from "markdown-it-highlightjs"
import hljs from "highlight.js"
import { fetch } from "@tauri-apps/plugin-http"
import "github-markdown-css/github-markdown.css"
import { computed, nextTick, onMounted, ref, watch } from "vue"
import { appDataDir, extname, resolve } from "@tauri-apps/api/path"
import { exists, readTextFile, writeTextFile, remove, writeFile } from "@tauri-apps/plugin-fs"
import { md5 } from "js-md5"
import { Command } from "@tauri-apps/plugin-shell"
import { useRouter } from "vue-router"
import { systemStore } from "../../stores/system"
import { fileToBase64, uuid } from "../../functions"
import { convertFileSrc, invoke } from "@tauri-apps/api/core"
const systemstore = systemStore()
const router = useRouter()
// https://github.com/ollama/ollama/releases/download/v0.4.0-rc5/OllamaSetup.exe
const modellist = ref()
const model = ref()
const messages = ref<
  {
    role: "user" | "system" | "assistant"
    md: string
    content: string
    images?: string[]
  }[]
>([])
const message = ref("")
const loading = ref(false)
const md = markdownit({
  html: true,
}).use(highlightjs, {
  hljs: hljs,
})
let baseurl = "http://127.0.0.1:11434/api"
const setupstatus = ref(true)
const loadsetup = computed(() => {
  return !setupstatus.value
})
onMounted(async () => {
  await checkollama()
  let list = await getmodellist()
  console.log(list)
  if (list) {
    modellist.value = list
    model.value = modellist.value[0]
    message.value = "你好！"
    sendmessage()
  }
  window.addEventListener("storage", (e) => {
    if (e.key == "system") {
      systemstore.$hydrate()
    }
  })
})

const checkollama = async function () {
  try {
    let cmd = await Command.create("ollama", ["--version"]).execute()
    console.log(cmd)
    return cmd.code == 0 ? (setupstatus.value = true) : (setupstatus.value = false)
  } catch (error) {
    return (setupstatus.value = false)
  }
}

const getmodellist = async function () {
  try {
    let res = await fetch(baseurl + "/tags", {
      method: "get",
      mode: "cors",
    })
    let json = await res.json()
    console.log(json)
    return json.models
  } catch (error) {
    snackbar.value = {
      bool: true,
      text: "未安装ollama或ollama服务未启动!",
    }
    setTimeout(() => {
      snackbar.value.bool = false
    }, 1000)
    console.log(error)
    return null
  }
}

const sendmessage = async function () {
  // if (!message.value) return;
  if (!model.value) {
    message.value = ""
    snackbar.value = {
      bool: true,
      text: "未安装ollama或ollama服务未启动!",
    }
    setTimeout(() => {
      snackbar.value.bool = false
    }, 1000)
    return
  }
  loading.value = true
  let m: {
    role: "user" | "system" | "assistant"
    content: string
    md: string
    images?: string[]
  } = {
    role: "user",
    md: message.value,
    content: message.value,
  }
  message.value = ""
  let basestr = ""
  let upfile = document.getElementById("upfile") as HTMLInputElement
  if (upfile.files && upfile.files.length > 0) {
    let file: File = upfile.files[0]
    let ext = await extname(file.name)
    let dir = (await appDataDir()) + "\\ollama\\temp\\" + uuid() + "." + ext
    await writeFile(dir, new Uint8Array(await file.arrayBuffer()))
    invoke("zipimage", {
      imgpath: dir,
      savepath: dir,
    })
    upfile.value = ""
    let base64 = await fileToBase64(file)
    basestr = base64.split(",")[1]
    m["images"] = [basestr]
    m.md = m.md + `\n <img src="${convertFileSrc(dir)}" width="50%" style="margin-left:50%">`
  }

  messages.value.push(m)
  let res = await fetch(baseurl + "/chat", {
    method: "post",
    mode: "cors",
    body: JSON.stringify({
      messages: [...messages.value],
      model: model.value.name,
      stream: false,
    }),
  })

  let json = await res.json()
  if (json.error != undefined) {
    messages.value.push({
      role: "assistant",
      md: "系统错误！,此问题我无法回答！",
      content: "系统错误！,此问题我无法回答！",
    })
  } else {
    messages.value[messages.value.length - 1].images = []
    messages.value.push({
      role: "assistant",
      md: json.message.content,
      content: json.message.content,
    })
  }
  loading.value = false
  let path = await resolve(await appDataDir(), "ollama", md5(model.value.name) + ".txt")
  await writeTextFile(path, JSON.stringify([...messages.value]), {
    create: true,
  })
  let dom = document.getElementById("chatdiv")
  if (dom) {
    dom.scroll({
      top: dom.scrollHeight,
      behavior: "smooth",
    })
  }
}
const snackbar = ref<{
  bool: boolean
  text: string
}>({
  bool: false,
  text: "",
})

watch(model, async () => {
  let path = await resolve(await appDataDir(), "ollama", md5(model.value.name) + ".txt")
  // console.log(path)
  if (await exists(path)) {
    let str = await readTextFile(path)
    messages.value = JSON.parse(str)
  } else {
    messages.value = []
    message.value = "你好！"
    await sendmessage()
    writeTextFile(path, JSON.stringify([...messages.value]), {
      create: true,
    })
  }
  nextTick(() => {
    let dom = document.getElementById("chatdiv")
    if (dom) {
      dom.scroll({
        top: 99999999999999,
        behavior: "smooth",
      })
    }
  })
})

const delmsg = async function () {
  let path = await resolve(await appDataDir(), "ollama", md5(model.value.name) + ".txt")
  await remove(path)
  messages.value = []
}

const upfile = function () {
  let dom = document.getElementById("upfile") as HTMLInputElement
  dom.value = ""
  dom.click()
}
</script>

<template>
  <div class="window">
    <v-dialog
      v-model="loadsetup"
      width="auto"
      data-tauri-drag-region
    >
      <v-card
        prepend-icon="mdi-update"
        title="安装ollama"
      >
        <template v-slot:text>
          <div
            class="markdown-body"
            v-html="
              md.render(
                '#### 下载并安装ollama \n  https://ollama.com/download/ \n #### 查找想要安装的模型 \n  https://ollama.com/library \n #### 进入想要下载的模型页面;选择适合电脑的模型;打开CMD并输入： \n \`\`\`javascript \n ollama run 模型名称 \n ollama run qwen2.5:7b #通问千义 \n ollama run llama3.1:8b # Meta llama3.1 \n ``` \n #### 安装完成后页面自动关闭并可以使用AI对话！ ',
              )
            "
          ></div>
        </template>
        <template v-slot:actions>
          <v-btn
            style="margin-right: 20px"
            @click="checkollama()"
          >
            安装完成
          </v-btn>
          <v-btn
            style="margin-right: 20px"
            @click="
              () => {
                router.push({ path: '/pages/setting/shortcut' })
              }
            "
          >
            暂不使用
          </v-btn>
        </template>
      </v-card>
    </v-dialog>
    <v-card
      :style="{
        background: systemstore.btnbarbackground,
        backgroundSize: 'cover',
      }"
      class="btnbar"
    >
      <v-btn
        style="margin-right: 20px"
        @click="delmsg"
      >
        <template v-slot:prepend>
          <v-icon>mdi-delete-outline</v-icon>
        </template>
        清空
      </v-btn>
    </v-card>
    <v-snackbar
      multi-line
      v-model="snackbar.bool"
    >
      {{ snackbar.text }}
    </v-snackbar>
    <v-progress-linear
      color="black"
      :indeterminate="loading"
    ></v-progress-linear>
    <div style="width: 100%; height: calc(100% - 64px); overflow: hidden">
      <v-tabs-window
        v-model="model"
        style="width: 100%; height: 100%"
      >
        <v-tabs
          density="compact"
          show-arrows
          v-model="model"
          hide-slider
          style="background: rgba(223, 223, 223, 0.4)"
        >
          <v-tab
            v-for="item in modellist"
            :key="item.name"
            :text="item.name"
            :value="item"
          ></v-tab>
        </v-tabs>
        <div
          id="chatdiv"
          style="
            width: 100%;
            height: calc(100% - 60px);
            display: flex;
            overflow: hidden;
            overflow-y: scroll;
            flex-direction: column;
            box-sizing: border-box;
            padding: 10px;
            transition: all 0.3s linear;
            user-select: text !important;
          "
        >
          <template v-for="item in messages">
            <div
              v-if="item.role == 'assistant'"
              style="
                width: 100%;
                display: flex;
                justify-content: flex-start;
                margin-bottom: 20px;
                transition: all 0.3s linear;
              "
            >
              <div style="width: calc(100% - 50px); display: flex; justify-content: flex-start">
                <v-avatar
                  icon="mdi-robot-outline"
                  style="margin-right: 10px"
                  :style="{ color: systemstore.fontcolor }"
                ></v-avatar>
                <div
                  class="markdown-body"
                  :style="{
                    background: 'rgba(220,220,220,0.4)',
                    color: systemstore.fontcolor,
                  }"
                  v-html="md.render(item.md)"
                ></div>
              </div>
            </div>
            <div
              v-else
              style="
                width: 100%;
                display: flex;
                justify-content: flex-end;
                margin-bottom: 20px;
                transition: all 0.3s linear;
              "
            >
              <div style="width: calc(100% - 50px); display: flex; justify-content: flex-end">
                <div
                  class="markdown-body"
                  v-html="md.render(item.md)"
                  :style="{
                    background: 'rgba(220,220,220,0.4)',
                    color: systemstore.fontcolor,
                    marginRight: '10px',
                  }"
                ></div>
                <v-avatar
                  icon="mdi-account-circle-outline"
                  :style="{ color: systemstore.fontcolor }"
                ></v-avatar>
              </div>
            </div>
          </template>
        </div>
        <div
          style="
            width: 100%;
            height: 60px;
            display: flex;
            justify-content: center;
            align-items: flex-end;
            box-sizing: border-box;
            padding: 10px 10px;
          "
        >
          <v-text-field
            width="50px"
            v-model="message"
            hide-details="auto"
            density="compact"
            variant="solo"
            clearable
            v-on:keyup.enter="sendmessage"
          >
            <template v-slot:append>
              <input
                type="file"
                id="upfile"
                accept="image/png, image/jpeg"
                style="width: 0px"
              />
              <v-icon
                @click="upfile"
                style="margin-right: 15px; font-size: 40px"
              >
                mdi-image
              </v-icon>
              <v-btn
                style="height: 40px"
                @click="sendmessage"
              >
                <template v-slot:prepend>
                  <v-icon>mdi-send-variant-outline</v-icon>
                </template>
                提问
              </v-btn>
            </template>
          </v-text-field>
        </div>
      </v-tabs-window>
    </div>
  </div>
</template>

<style>
.window {
  width: 100%;
  height: 100%;
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

.markdown-body {
  box-sizing: border-box;
  min-width: 200px;
  max-width: 980px;
  padding: 10px;
  border-radius: 10px;
  text-align: left;
}

@media (max-width: 767px) {
  .markdown-body {
    padding: 15px;
  }
}
</style>
