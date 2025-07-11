import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow"
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
        name: 'wallpaper',
        component: () => import("./../pages/setting/wallpaper.vue"),
      },
      {
        path: "datenote",
        name: 'datenote',
        component: () => import("./../pages/setting/datenote.vue"),
      },
      {
        path: "shortcut",
        name: 'shortcut',
        component: () => import("./../pages/setting/shortcut.vue"),
      },
      {
        path: "note",
        name: 'note',
        component: () => import("./../pages/setting/note.vue"),
      },
      {
        path: "ollama",
        name: 'ollama',
        component: () => import("./../pages/setting/ollama.vue"),
      },
      {
        path: "webdav",
        name: 'webdav',
        component: () => import("./../pages/setting/webdav.vue")
      }
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
      {
        path: "search",
        component: () => import("./../pages/desktop/search.vue"),
      },
      {
        path: "dragfile",
        component: () => import("./../pages/desktop/dragfile.vue"),
      },
    ],
  },
]

const router = createRouter({
  routes,
  history: createWebHashHistory(),
})

router.beforeEach(async (to) => {
  document.title = "skydesk2-" + (await getCurrentWebviewWindow().label)
  let index = to.fullPath.indexOf("setting")
  if (index >= 0) {
    document.getElementsByTagName("body")[0].style.background = "white"
  } else {
    document.getElementsByTagName("body")[0].style.background = "transparent"
  }
})

export default router
