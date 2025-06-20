<script setup lang="ts">
import { Child, Command } from '@tauri-apps/plugin-shell'
import { computed, reactive, ref } from 'vue'
import { webdavStore } from '../../stores/webdav'
import { systemStore } from '../../stores/system'
import { useVuelidate } from '@vuelidate/core'
import { required, ipAddress, helpers, minValue, maxValue } from '@vuelidate/validators'
import { writeTextFile } from '@tauri-apps/plugin-fs'
import { appDataDir, resolve } from '@tauri-apps/api/path'
import { open } from '@tauri-apps/plugin-dialog'
import GridContainer from '../../components/GridContainer.vue'
import bcrypt from 'bcryptjs'
const webdavstore = webdavStore()
const systemstore = systemStore()
const dialog = ref(false)
defineOptions({
  name: 'webdav',
})

const user = reactive<{
  username: string
  password: string
  directory: string
  permissions: string[]
}>({
  username: '',
  password: '',
  directory: '/',
  permissions: ['R'],
})

const baseconfigrules = computed(() => ({
  address: { required: helpers.withMessage('必填', required), ipAddress: helpers.withMessage('错误的IP地址', ipAddress) },
  port: { required: helpers.withMessage('必填', required), minValue: helpers.withMessage('最小值为1', minValue(1)), maxValue: helpers.withMessage('最大值为65535', maxValue(65535)) },
}))
const checkusername = (value: string) => {
  console.log('checkusername', value)
  let res = webdavstore.config.users.some((u) => u.username === value)
  console.log('checkusername result', res)
  return !res
}
const userrules = computed(() => ({
  username: { required: helpers.withMessage('必填', required), checkusername: helpers.withMessage('用户名已存在', checkusername) },
  permissions: { required: helpers.withMessage('必填', required) },
  password: { required: helpers.withMessage('必填', required) },
  directory: { required: helpers.withMessage('必填', required) },
}))

const vbase$ = useVuelidate(baseconfigrules, webdavstore.config)
const vuser$ = useVuelidate(userrules, user)
// 添加用户
const adduser = function () {
  if (vuser$.value.$invalid) {
    vuser$.value.$touch()
    return
  }
  webdavstore.config.users.push({
    ...user,
  })
}
// 删除用户
const deluser = function (user: { username: string; password: string; directory: string; permissions: string[] }) {
  const index = webdavstore.config.users.indexOf(user)
  if (index !== -1) {
    webdavstore.config.users.splice(index, 1)
  }
}

const getdir = async function () {
  const path = await open({
    directory: true,
    multiple: false,
    title: '选择文件夹',
  })
  if (path) {
    user.directory = path as string
  }
}

// 保存配置
const saveconfig = async function () {
  console.log('保存配置', webdavstore.config)
  if (vbase$.value.$invalid) {
    vbase$.value.$touch()
    return
  }
  if (webdavstore.config.users.length === 0) {
    webdavstore.config.users.push({
      username: 'admin',
      password: 'admin',
      directory: '/',
      permissions: ['C', 'U', 'R', 'D'],
    })
    return
  }

  let userconfig = webdavstore.config.users.map((user) => {
    return {
      username: user.username,
      password: '{bcrypt}' + bcrypt.hashSync(user.password, 10), // 使用bcrypt加密密码
      directory: user.directory,
      permissions: user.permissions.join(''),
    }
  })
  let json = {
    address: webdavstore.config.address,
    port: webdavstore.config.port,
    directory: '.',
    tls: false,
    cert: 'cert.pem',
    key: 'key.pem',
    prefix: '/',
    debug: false,
    noSniff: false,
    behindProxy: false,
    permissions: 'R',
    rules: [],
    rulesBehavior: 'overwrite',
    log: {
      format: 'console',
      colors: true,
      outputs: ['stderr'],
    },
    cors: {
      enabled: true,
      credentials: true,
      allowed_headers: ['Depth'],
      allowed_hosts: ['http://localhost:8080'],
      allowed_methods: ['GET'],
      exposed_headers: ['Content-Length', 'Content-Range'],
    },
    users: userconfig,
  }
  let path = await resolve(await appDataDir(), 'webdav/config.json')
  await writeTextFile(path, JSON.stringify(json), { create: true })
  dialog.value = false
}

// 进程控制
const webdavprocess = ref<Child | undefined>()
const webdevcontrol = async function () {
  if (webdavprocess.value) {
    webdavprocess.value.kill()
    webdavprocess.value = undefined
    return
  } else {
    let path = await resolve(await appDataDir(), 'webdav/config.json')
    webdavprocess.value = await Command.sidecar('bin/webdav/webdav', ['-c', path], {
      encoding: 'GBK',
    }).spawn()
  }
}
</script>

<template>
  <div class="window">
    <v-dialog v-model="dialog">
      <v-card style="width: calc(100vw - 48px); height: calc(100vh - 48px); padding: 20px">
        <v-card-text style="padding: 0px">
          <v-row>
            <v-col :cols="6">
              <v-list variant="tonal">
                <v-list-item>
                  <v-text-field
                    v-model="webdavstore.config.address"
                    :error-messages="vbase$.address.$errors.map((e) => String(e.$message))"
                    placeholder="IP"
                    density="compact"
                    @blur="vbase$.address.$touch"
                    @input="vbase$.address.$touch"></v-text-field>
                </v-list-item>
                <v-list-item>
                  <v-text-field
                    v-model.number="webdavstore.config.port"
                    :error-messages="vbase$.port.$errors.map((e) => (typeof e.$message === 'string' ? e.$message : String(e.$message?.value ?? '')))"
                    placeholder="端口"
                    density="compact"
                    @blur="vbase$.port.$touch"
                    @input="vbase$.port.$touch"></v-text-field>
                </v-list-item>
                <v-list-item>
                  <v-text-field
                    v-model="user.username"
                    :error-messages="vuser$.username.$errors.map((e) => (typeof e.$message === 'string' ? e.$message : String(e.$message?.value ?? '')))"
                    @blur="vuser$.username.$touch"
                    @input="vuser$.username.$touch"
                    placeholder="用户名"
                    density="compact"></v-text-field>
                </v-list-item>
                <v-list-item>
                  <v-text-field
                    v-model="user.password"
                    :error-messages="vuser$.password.$errors.map((e) => (typeof e.$message === 'string' ? e.$message : String(e.$message?.value ?? '')))"
                    @blur="vuser$.password.$touch"
                    @input="vuser$.password.$touch"
                    placeholder="密码"
                    density="compact"></v-text-field>
                </v-list-item>
                <v-list-item>
                  <v-text-field
                    v-model="user.directory"
                    :error-messages="vuser$.directory.$errors.map((e) => (typeof e.$message === 'string' ? e.$message : String(e.$message?.value ?? '')))"
                    @blur="vuser$.directory.$touch"
                    @input="vuser$.directory.$touch"
                    placeholder="路径"
                    :readonly="true"
                    @click="getdir"
                    density="compact"></v-text-field>
                </v-list-item>
                <v-list-item>
                  <v-combobox
                    v-model="user.permissions"
                    :error-messages="vuser$.permissions.$errors.map((e) => (typeof e.$message === 'string' ? e.$message : String(e.$message?.value ?? '')))"
                    @blur="vuser$.permissions.$touch"
                    @input="vuser$.permissions.$touch"
                    density="compact"
                    label="权限"
                    :items="['C', 'U', 'R', 'D']"
                    multiple></v-combobox>
                </v-list-item>
                <v-list-item>
                  <v-btn @click="adduser" block>添加用户</v-btn>
                </v-list-item>
              </v-list>
            </v-col>
            <v-col :cols="6">
              <div style="height: calc(100vh - 88px - 52px); overflow: hidden; overflow-y: scroll">
                <v-list variant="tonal" v-for="user in webdavstore.config.users">
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
                  <v-list-item>
                    <v-btn block @click="deluser(user)">删除</v-btn>
                  </v-list-item>
                </v-list>
              </div>
            </v-col>
          </v-row>
        </v-card-text>
        <v-card-actions>
          <v-btn @click="dialog = false" color="primary">关闭</v-btn>
          <v-btn @click="saveconfig">确定</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <v-card
      :style="{
        background: systemstore.btnbarbackground,
        backgroundSize: 'cover',
      }"
      class="btnbar">
      <v-btn style="margin-right: 20px" @click="dialog = true">
        <template v-slot:prepend>
          <v-icon>mdi-tune</v-icon>
        </template>
        配置
      </v-btn>
      <v-btn style="margin-right: 20px" @click="webdevcontrol">
        <template v-slot:prepend>
          <v-icon v-if="webdavprocess == undefined">mdi-play</v-icon>
          <v-icon v-else>mdi-stop</v-icon>
        </template>
        {{ webdavprocess == undefined ? '启动' : '停止' }}
      </v-btn>
      <div>{{ webdavstore.config.address }}:{{ webdavstore.config.port }}</div>
    </v-card>
    <v-progress-linear color="black" :indeterminate="false"></v-progress-linear>
    <div style="width: 100%; height: calc(100% - 64px); display: flex; overflow: hidden; padding: 10px; box-sizing: border-box">
      <GridContainer style="width: 100%; height: 100%; min-height: 100%" v-model="webdavstore.config.users" :gridheight="260" :gridwidth="320" :padding="10">
        <template v-slot="{ item }">
          <v-card style="width: 100%; height: 100%; background: transparent" variant="elevated" elevation="5">
            <v-list style="background: transparent">
              <v-list-item title="用户名">
                <template v-slot:append>
                  {{ item.username }}
                </template>
              </v-list-item>
              <v-list-item title="密码">
                <template v-slot:append>
                  {{ item.password }}
                </template>
              </v-list-item>
              <v-list-item title="文件夹">
                <template v-slot:append>
                  <div style="width: 220px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; text-align: right">
                    {{ item.directory }}
                  </div>
                </template>
              </v-list-item>
              <v-list-item title="权限">
                <template v-slot:append>
                  {{ item.permissions.join('') }}
                </template>
              </v-list-item>
              <v-list-item>
                <v-btn block @click="deluser(item)">删除</v-btn>
              </v-list-item>
            </v-list>
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

.btnbar {
  width: 100%;
  height: 60px;
  display: flex;
  align-items: center;
  box-sizing: border-box;
  padding: 0 20px;
  filter: drop-shadow(0px 2px 5px gray);
}
</style>
