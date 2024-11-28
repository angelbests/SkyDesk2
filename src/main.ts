
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
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { LogicalSize } from "@tauri-apps/api/dpi";
let datastr = localStorage.getItem("size")
if(datastr){
  let data = JSON.parse(datastr)
  getCurrentWebviewWindow().setSize(new LogicalSize(data.width,data.height))
}
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
