<script setup lang="ts">
import { computed, onMounted, ref} from 'vue';
import { uuid } from '../../functions';
import { createWindow } from '../../functions/window';
import { listen } from '@tauri-apps/api/event';
import { noteStore } from '../../stores/window';
import { getAllWebviewWindows } from '@tauri-apps/api/webviewWindow';
const noteref = ref<HTMLDivElement>()
const notestore = noteStore()
window.addEventListener('storage', (e) => {
  if (e.key === "note") {
    notestore.$hydrate()
  }
})
const noteWidth = ref()
onMounted(() => {
    updateElementHeight()
    window.addEventListener('resize', updateElementHeight);
    listen("note", (e: any) => {
        let index = notestore.note.findIndex(item => {
            return item.label == e.payload.label
        })

        if (index >= 0) {
            notestore.note[index] = e.payload
        } else {
            notestore.note.push(e.payload)
        }
        console.log(e.payload)
    })
})

const updateElementHeight = function () {
    if (noteref.value) {
        noteWidth.value = noteref.value.offsetWidth
    }
}

const noteListHeight = computed(() => {
    return Math.ceil((notestore.note.length / Math.trunc(noteWidth.value / 420))) * 320 + 'px';
})

const createnote = function () {
    let label = "note-" + uuid()
    createWindow(label, {
        x: 0,
        y: 0,
        width: 400,
        height: 400,
        minWidth: 400,
        shadow: false,
        decorations: false,
        transparent: true,
        skipTaskbar:true,
        url: "/#/pages/desktop/note"
    })
}

const opennote = async function (item: { label: string; }) {
    let note = notestore.note.filter(note => {
        return note.label == item.label
    })
    getAllWebviewWindows().then(async res => {
        let w = res.filter((window: any) => {
            return window.label == item.label
        })
        if (w[0]) {
            w[0].setFocus()
        } else {
            if (note && note[0].option) {
                let w = await createWindow(note[0].label, note[0].option)
                w?.setFocus()
            }
        }
    })
}

//删除note
const delnote = function (item: { label: string }) {
    let index = notestore.note.findIndex(note => {
        return item.label == note.label
    })
    console.log(index)
    if (index >= 0) {
        localStorage.removeItem(notestore.note[index].label)
        notestore.note.splice(index, 1)

    }
    getAllWebviewWindows().then(res => {
        res.filter((window: { label: string; close: () => void; }) => {
            if (window.label == item.label) {
                window.close()
            }
        })
    })

}
</script>

<template>
    <div class="window">
        <v-card
            style="width: 100%;height: 60px;display: flex;align-items: center;box-sizing: border-box;padding: 0 20px;filter:drop-shadow(0px 2px 5px gray)">
            <v-btn style="margin-right: 20px;" @click="createnote">
                <template v-slot:prepend>
                    <v-icon>mdi-note-plus-outline</v-icon>
                </template>
                添加便签
            </v-btn>
        </v-card>
        <div style="width: 100%;height: calc(100% - 60px);display: flex;overflow: hidden;background: white;">
            <div class="note" id="note" ref="noteref">
                <div class="note-list" :style="{ height: noteListHeight, minHeight: '100%' }">
                    <v-card prepend-icon="" width="400" height="305" variant="elevated" elevation="10"
                        v-for="item in notestore.note">
                        <v-card-text
                            :style="{ width: '100%', height: '255px', background: 'rgba(' + item.color + ',' + item.opacity / 100 + ')', boxSizing: 'border-box', padding: '10px', overflow: 'hidden', overflowY: 'scroll' }">
                            <div v-html="item.value">

                            </div>
                        </v-card-text>
                        <v-card-actions>
                            <v-btn @click="opennote(item)">
                                <template v-slot:prepend>
                                    <v-icon>mdi-note-edit-outline</v-icon>
                                </template>
                                编辑
                            </v-btn>
                            <v-btn @click="delnote(item)">
                                <template v-slot:prepend>
                                    <v-icon>mdi-note-remove-outline</v-icon>
                                </template>
                                删除
                            </v-btn>
                        </v-card-actions>
                    </v-card>
                </div>
            </div>
        </div>
    </div>
</template>

<style>
.window {
    width: 100%;
    height: 100%;
}

.note {
    width: 100%;
    background: white;
    display: flex;
    justify-content: center;
    overflow-x: hidden;
    overflow-y: scroll;
}

.note-list {
    width: 100%;
    height: auto;
    display: grid;
    grid-template-columns: repeat(auto-fill, 420px);
    grid-template-rows: repeat(auto-fit, 320px);
    grid-auto-flow: row;
    justify-items: center;
    align-items: center;
    justify-content: center;

}
</style>