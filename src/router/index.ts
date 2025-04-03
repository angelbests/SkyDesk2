import { createRouter, createWebHashHistory, RouteRecordRaw } from "vue-router"

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    redirect: "/pages/setting/shortcut",
  },
  {
    path: "/pages/setting",
    component: () => import("./../pages/setting.vue"),
    children: [
      {
        path: "wallpaper",
        component: () => import("./../pages/setting/wallpaper.vue"),
      },
      {
        path: "datenote",
        component: () => import("./../pages/setting/datenote.vue"),
      },
      {
        path: "shortcut",
        component: () => import("./../pages/setting/shortcut.vue"),
      },
      {
        path: "capture",
        component: () => import("./../pages/setting/capture.vue"),
      },
      {
        path: "note",
        component: () => import("./../pages/setting/note.vue"),
      },
      {
        path: "ollama",
        component: () => import("./../pages/setting/ollama.vue"),
      },
    ],
  },
  {
    path: "/pages/desktop",
    component: null,
    children: [
      {
        path: "note",
        component: () => import("./../pages/desktop/note.vue"),
      },
      {
        path: "shortcut",
        component: () => import("./../pages/desktop/shortcut.vue"),
      },
      {
        path: "shortcutSetting",
        component: () => import("./../pages/desktop/shortcutSetting.vue"),
      },
      {
        path: "wallpaper",
        component: () => import("./../pages/desktop/wallpaper.vue"),
      },
      {
        path: "capture",
        component: () => import("./../pages/desktop/capture.vue"),
      },
      {
        path: "captureWindow",
        component: () => import("./../pages/desktop/captureWindow.vue"),
      },
      {
        path: "captureBtn",
        component: () => import("./../pages/desktop/captureBtn.vue"),
      },
      {
        path: "wheel",
        component: () => import("./../pages/desktop/wheel.vue"),
      },
      {
        path: "tray",
        component: () => import("./../pages/desktop/tray.vue"),
      },
      {
        path: "netspeed",
        component: () => import("./../pages/desktop/netspeed.vue"),
      },
      {
        path: "wallpapersetting",
        component: () => import("./../pages/desktop/wallpapersetting.vue"),
      },
      {
        path: "taskbar",
        component: () => import("./../pages/desktop/taskbar.vue"),
      },
      {
        path: "hovertop",
        component: () => import("./../pages/desktop/hovertop.vue"),
      },
    ],
  },
]

const router = createRouter({
  routes,
  history: createWebHashHistory(),
})

router.beforeEach((to) => {
  let index = to.fullPath.indexOf("setting")
  if (index >= 0) {
    document.getElementsByTagName("body")[0].style.background = "white"
  } else {
    document.getElementsByTagName("body")[0].style.background = "transparent"
  }
})

export default router
