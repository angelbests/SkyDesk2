import { createI18n } from "vue-i18n"
const i18n = createI18n({
  locale: "zhHans",
  fallbackLocale: "zhHans",
  messages: {
    zhHans: {
      message: {
        menu: {
          shortcut: "快捷",
          wallpaper: "壁纸",
          note: "便签",
          ai: "AI",
          capture: "录屏",
          datenote: "日历",
        },
        setting: {
          tray: "启动到托盘",
          start: "开机自启",
          netspeed: "网速控件",
          Taskbar: "任务栏",
          wheel: "轮盘",
          clearuser: "清除用户数据",
          clear: "clear",
        },
        background: {
          backgroundsetting: "背景设置",
          mainwindow: "主窗",
          menu: "菜单",
          title: "标题",
          shortcut: "快捷",
          button: "按钮",
          buttonbar: "按钮栏",
          text: "文本",
        },
        about: {
          title: "关于",
          check: "检查更新",
        },
      },
    },
  },
})

export default i18n
