import { createApp } from "vue"
import App from "./App.vue"
import { createPinia } from "pinia"
import piniaPluginPersistedstate from "pinia-plugin-persistedstate"
import router from "./router/index"
// npm install @tauri-apps/cli@latest @tauri-apps/api@latest @tauri-apps/plugin-clipboard-manager @tauri-apps/plugin-dialog @tauri-apps/plugin-fs @tauri-apps/plugin-global-shortcut @tauri-apps/plugin-http @tauri-apps/plugin-log @tauri-apps/plugin-opener @tauri-apps/plugin-process @tauri-apps/plugin-shell @tauri-apps/plugin-updater @tauri-apps/plugin-upload
// Vuetify
import "@mdi/font/css/materialdesignicons.css"
import "vuetify/styles"
import { createVuetify } from "vuetify"
import * as components from "vuetify/components"
import * as directives from "vuetify/directives"
import { VNumberInput } from "vuetify/labs/VNumberInput"
import { VDateInput } from "vuetify/labs/VDateInput"
import { zhHans } from "vuetify/locale"
import i18n from "./i18n"

const vuetify = createVuetify({
  components: {
    VNumberInput,
    VDateInput,
    ...components,
  },
  directives,
  icons: {
    defaultSet: "mdi",
  },
  locale: {
    locale: "zhHans",
    fallback: "zhHans",
    messages: { zhHans },
  },
})

const app = createApp(App)

const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)
app.use(pinia)
app.use(i18n)
app.use(router)
app.use(vuetify)
app.mount("#app")
