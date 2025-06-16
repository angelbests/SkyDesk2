<script setup lang="ts">
import { Child, Command } from '@tauri-apps/plugin-shell'
import { ref } from 'vue'
import { webdavStore } from '../../stores/webdav'
const webdavstore = webdavStore()

const baseconfig = ref<{
  address: string
  port: number
  permissions: []
  users: {
    username: string
    password: string
    directory: string
    permissions: string[]
  }[]
}>({
  address: '0.0.0.0',
  port: 6065,
  permissions: [],
  users: [],
})

const user = ref<{
  username: string
  password: string
  directory: string
  permissions: string[]
}>({
  username: '',
  password: '',
  directory: '',
  permissions: [],
})

const adduser = function () {
  baseconfig.value.users.push({
    ...user.value,
  })
}

// 进程控制
const wevdavprocess = ref<Child | undefined>()
const startwebdev = async function () {
  wevdavprocess.value = await Command.sidecar('bin/webdav/webdav', [], {
    encoding: 'GBK',
  }).spawn()
}
const stopwebdav = async function () {
  if (wevdavprocess.value) {
    wevdavprocess.value.kill()
  }
}
const dialog = ref(true)
</script>

<template>
  <div class="window">
    <v-dialog v-model="dialog" max-width="800">
      <v-card style="padding: 20px">
        <v-card-text>
          <v-row>
            <v-col :cols="7">
              <v-card variant="tonal" style="margin-bottom: 20px">
                <v-list>
                  <v-list-item>
                    <v-text-field v-model="baseconfig.address" placeholder="IP" density="compact" hide-details="auto" :rules="[]"></v-text-field>
                  </v-list-item>
                  <v-list-item>
                    <v-text-field v-model.number="baseconfig.port" placeholder="端口" density="compact" hide-details="auto"></v-text-field>
                  </v-list-item>
                  <v-list-item>
                    <v-combobox v-model="baseconfig.permissions" density="compact" hide-details="auto" label="权限" :items="['C', 'U', 'R', 'D']" multiple></v-combobox>
                  </v-list-item>
                  <v-list-item>
                    <v-btn block style="border-radius: 0px" variant="flat" color="grey-lighten-3">保存</v-btn>
                  </v-list-item>
                </v-list>
              </v-card>
              <v-card variant="tonal">
                <v-list>
                  <v-list-item>
                    <v-text-field v-model="user.username" placeholder="用户名" density="compact" hide-details="auto"></v-text-field>
                  </v-list-item>
                  <v-list-item>
                    <v-text-field v-model="user.password" placeholder="密码" density="compact" hide-details="auto"></v-text-field>
                  </v-list-item>
                  <v-list-item>
                    <v-text-field v-model="user.directory" placeholder="路径" density="compact" hide-details="auto"></v-text-field>
                  </v-list-item>
                  <v-list-item>
                    <v-combobox v-model="user.permissions" density="compact" hide-details="auto" label="权限" :items="['C', 'U', 'R', 'D']" multiple></v-combobox>
                  </v-list-item>
                  <v-list-item>
                    <v-btn @click="adduser" block style="border-radius: 0px" variant="flat" color="grey-lighten-3">添加用户</v-btn>
                  </v-list-item>
                </v-list>
              </v-card>
            </v-col>
            <v-col :cols="5">
              <div style="width: 100%; height: 100%; overflow: hidden; overflow-y: scroll">
                <v-list>
                  <v-list-item title="IP地址">
                    <template v-slot:append>
                      {{ baseconfig.address }}
                    </template>
                  </v-list-item>
                  <v-list-item title="端口">
                    <template v-slot:append>
                      {{ baseconfig.port }}
                    </template>
                  </v-list-item>
                  <v-list-item title="权限">
                    <template v-slot:append>
                      {{ baseconfig.permissions.join('') }}
                    </template>
                  </v-list-item>
                  <v-list-item v-for="user in baseconfig.users">
                    <v-card>
                      <v-list variant="tonal">
                        <v-list-item title="用户名">
                          <template v-slot:append>
                            {{ user.username }}
                          </template>
                        </v-list-item>
                        <v-list-item title="密码">
                          <template v-slot:append>
                            {{ user.password }}
                          </template>
                        </v-list-item>
                        <v-list-item title="文件夹">
                          <template v-slot:append>
                            {{ user.directory }}
                          </template>
                        </v-list-item>
                        <v-list-item title="权限">
                          <template v-slot:append>
                            {{ user.permissions.join('') }}
                          </template>
                        </v-list-item>
                      </v-list>
                    </v-card>
                  </v-list-item>
                </v-list>
              </div>
              <v-btn block>新增</v-btn>
            </v-col>
          </v-row>
        </v-card-text>
      </v-card>
    </v-dialog>
    <v-card style="width: 100%; height: 100%">
      <v-btn @click="startwebdev">启动</v-btn>
      <v-btn @click="stopwebdav">停止</v-btn>
    </v-card>
  </div>
</template>

<style>
.window {
  width: 100%;
  height: 100%;
}
</style>
