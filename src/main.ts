import { BaseDirectory, mkdir } from '@tauri-apps/plugin-fs';
await mkdir("lnk",{"baseDir":BaseDirectory.AppData,"recursive":true})
await mkdir("window",{baseDir:BaseDirectory.AppData,"recursive":true})
await mkdir("wallpapers",{baseDir:BaseDirectory.AppData,"recursive":true})
await mkdir("wallpapers\\image",{baseDir:BaseDirectory.AppData,"recursive":true})
await mkdir("wallpapers\\html",{baseDir:BaseDirectory.AppData,"recursive":true})
await mkdir("wallpapers\\video",{baseDir:BaseDirectory.AppData,"recursive":true})
await mkdir("note",{baseDir:BaseDirectory.AppData,"recursive":true})
await mkdir("ollama",{baseDir:BaseDirectory.AppData,"recursive":true})
await mkdir("skydesk2",{baseDir:BaseDirectory.Picture,"recursive":true})
import { createApp } from "vue";
import App from "./App.vue";
import { createPinia } from "pinia"
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import router from "./router/index"
// Vuetify
import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import { VNumberInput } from 'vuetify/labs/VNumberInput'
import { VDateInput } from 'vuetify/labs/VDateInput'
import { zhHans } from 'vuetify/locale'
const vuetify = createVuetify({
    components:{
      VNumberInput,
      VDateInput,
      ...components
    },
    directives,
    icons:{
      defaultSet: 'mdi'
    },
    locale: {
      locale: 'zhHans',
      fallback: 'zhHans',
      messages: { zhHans },
    },
})

const app = createApp(App)
const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)
app.use(pinia)
app.use(router)
app.use(vuetify)
app.mount("#app");
