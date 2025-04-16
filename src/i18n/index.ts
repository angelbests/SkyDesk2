import { createI18n } from "vue-i18n"
const i18n = createI18n({
  locale: "zhHans",
  fallbackLocale: "zhHans",
  messages: {
    zhHans: {
      message: {
        hello: "你好，世界",
      },
    },
  },
})

export default i18n
