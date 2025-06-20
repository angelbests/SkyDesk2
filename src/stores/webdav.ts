import { defineStore } from "pinia";
import type { DavConfig } from "../types/webdavType";
export const webdavStore = defineStore("webdav", {
    state:()=> {
        return {
            config:[] as DavConfig[]
        }
    },
})