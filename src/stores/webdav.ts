import { defineStore } from "pinia";
import type { DavConfig } from "../types/webdavType";
export const webdavStore = defineStore("webdav", {
    state:()=> {
        return {
            config: {
                address: '0.0.0.0',
                port: 6065,
                users: [
                    {
                        username: 'admin',
                        password: 'admin',
                        directory: '/',
                        permissions: ['R']
                    }
                ]
            } as DavConfig
        }
    },
    persist:true
})