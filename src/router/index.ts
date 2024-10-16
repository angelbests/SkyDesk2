
import { createRouter, createWebHashHistory, RouteRecordRaw } from "vue-router";

const routes:RouteRecordRaw[] = [
    {
        path:"/",
        redirect:"/pages/setting/shortcut"
    },
    {
        path:"/pages/setting",
        component:()=>import('./../pages/setting.vue'),
        children:[
            {
                path:"wallpaper",
                component:()=>import("./../pages/setting/wallpaper.vue")
            },
            {
                path:"monitor",
                component:()=>import("./../pages/setting/monitor.vue")
            },
            {
                path:'shortcut',
                component:()=>import("./../pages/setting/shortcut.vue")
            },
            {
                path:'window',
                component:()=>import("./../pages/setting/window.vue")
            },
            {
                path:"system",
                component:()=>import("./../pages/setting/system.vue")
            },
            {
                path:"capture",
                component:()=>import("./../pages/setting/capture.vue")
            },
            {
                path:"note",
                component:()=>import("./../pages/setting/note.vue")   
            }
        ]
    },
    {
        path:"/pages/desktop",
        component:null,
        children:[
            {
                path:'folder',
                component:()=>import("./../pages/desktop/folder.vue")
            },
            {
                path:'note',
                component:()=>import('./../pages/desktop/note.vue')
            },
            {
                path:'shortcut',
                component:()=>import('./../pages/desktop/shortcut.vue')
            },
            {
                path:'shortcutSetting',
                component:()=>import("./../pages/desktop/shortcutSetting.vue")
            },
            {
                path:'time',
                component:()=>import('./../pages/desktop/time.vue')
            },
            {
                path:'wallpaper',
                component:()=>import('./../pages/desktop/wallpaper.vue')
            },
            {
                path:'weather',
                component:()=>import('./../pages/desktop/weather.vue')
            },
            {
                path:'capture',
                component:()=>import('./../pages/desktop/capture.vue')
            },
            {
                path:'captureWindow',
                component:()=>import('./../pages/desktop/captureWindow.vue')
            },
            {
                path:'captureBtn',
                component:()=>import('./../pages/desktop/captureBtn.vue')
            },
            {
                path:'wheel',
                component:()=>import('./../pages/desktop/wheel.vue')
            },
            {
                path:'tray',
                component:()=>import('./../pages/desktop/tray.vue')
            }
        ]
    },
    {
        path:"/pages/server",
        component:()=>import("./../pages/server.vue")
    }
]

const router = createRouter({
    routes,
    history:createWebHashHistory()
})

router.beforeEach((to)=>{
    let index = to.fullPath.indexOf("setting")
    if(index>=0){
        document.getElementsByTagName("body")[0].style.background = "white"
    }else{
        document.getElementsByTagName("body")[0].style.background = "transparent"
    }
})

export default router