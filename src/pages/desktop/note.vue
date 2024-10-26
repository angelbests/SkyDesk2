<script lang="ts" setup>
import {onMounted, reactive, ref, shallowRef} from 'vue';
import { uuid,gettime } from '../../functions';
import RightBar from '../../components/RightBar.vue';

import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { open } from '@tauri-apps/plugin-dialog';
import { convertFileSrc } from '@tauri-apps/api/core';
import { writeFile,BaseDirectory, copyFile } from '@tauri-apps/plugin-fs';
import { appDataDir, basename } from '@tauri-apps/api/path';

import '@wangeditor/editor/dist/css/style.css'
import { Editor, Toolbar } from '@wangeditor/editor-for-vue';
import { IToolbarConfig,Boot,IButtonMenu, IDomEditor,IEditorConfig } from '@wangeditor/editor'
import { emit } from '@tauri-apps/api/event';
import { windowStore,noteStore } from '../../stores/window';
//#region vue
const appWindow = getCurrentWebviewWindow()
const editorData = reactive({
    value:'',
    date:gettime(),
    color:'213,213,213',
    opacity:100,
    label:appWindow.label,
    always:"normal"
});
const show = ref(true)
onMounted(()=>{
    // 初始化数据
    setTimeout(function(){
        let dataStr = localStorage.getItem(appWindow.label);
        if(dataStr){
            let data = JSON.parse(dataStr);
            editorData.date = data.date
            editorData.value = data.value
            editorData.color = data.color
            editorData.opacity = data.opacity
            editorData.always = data.always
            setconfig()
        }else{
            localStorage.setItem(appWindow.label,JSON.stringify(editorData))
            setconfig()
            emit("note",editorData)
        }
    },1)

    //文件拖拽监听
    filedrop();

    appWindow.listen("tauri://blur",function(){
        show.value = false
    })

    appWindow.listen("tauri://focus",function(){
        show.value = true
    })
})

const setconfig =async function(){
    let doc = document.getElementsByTagName('body')[0];
    doc.style.background = `rgba(${editorData.color},${editorData.opacity/100})`
    let tool:any = document.getElementsByClassName('w-e-toolbar')[0];
    tool.style.background = `rgba(${editorData.color},${editorData.opacity/100})`
    let text:any = document.getElementById('w-e-textarea-1');
    text.style.background = `rgba(${editorData.color},${editorData.opacity/100})`
    let con:any = document.getElementsByClassName('w-e-text-container')[0]
    con.style.background = 'transparent'
    if(editorData.always == "top"){
        await getCurrentWebviewWindow().setAlwaysOnBottom(false)
        await getCurrentWebviewWindow().setAlwaysOnTop(true)
    }else if(editorData.always == "bottom"){
        await getCurrentWebviewWindow().setAlwaysOnTop(false)
        await getCurrentWebviewWindow().setAlwaysOnBottom(true)
    }else if(editorData.always == "normal"){
        await getCurrentWebviewWindow().setAlwaysOnBottom(false)
        await getCurrentWebviewWindow().setAlwaysOnTop(false)
    }
}

//#region 监听图片文件的拖拽
const filedrop = async function(){
    await appWindow.listen("tauri://drag-drop",async (e:any) => {
        for(let i = 0 ;i<e.payload.paths.length;i++){
            // 判断是否是合法的图片格式，若不是则跳出此次循环
            let name = await basename(e.payload.paths[i]);
            let arr = name.split('.');
            let ext = arr[arr.length-1];
            let extarr = ['png','jpg','jpeg','gif','bmp','svg','apng','webp'];
            let boolarr = extarr.filter((item)=>{
                return ext === item
            });
            if(boolarr.length==0){
                continue;
            };
            name = uuid()+'.'+ext;
            await copyFile(e.payload.paths[i],"note\\"+name,{toPathBaseDir:BaseDirectory.AppData})
            let appDataDirPath = await appDataDir();
            let src = convertFileSrc(appDataDirPath+"\\note\\"+name)
            let img = {
                type: 'image',
                src:src,
                children: [{ text: '' }],
                style:{
                    width:'100%',
                    height:'auto'
                }
            }
            editorRef.value.insertNode(img)
        }
    });
}
//#endregion

// 记录数据
const onChangeTime = ref<any>();
const onChange = function(){
    clearTimeout(onChangeTime.value);
    onChangeTime.value=setTimeout(function(){
        localStorage.setItem(appWindow.label,JSON.stringify(editorData))
        emit("note",editorData)
    },100)
}

//#endregion

//#region wangeditor 初始化配置
const mode = ref<string>('default');
const toolbarConfig:Partial<IToolbarConfig> = {}
toolbarConfig.toolbarKeys = [
    'todo','numberedList','codeBlock','bold','italic','underline','through','upimage','color','lineHeight'
]

const editorConfig:Partial<IEditorConfig> = { 
    MENU_CONF: {
        "lineHeight":{
            lineHeightList: ['1', '1.5', '2', '2.5']
        }
    },
    placeholder: '请输入内容...' ,
    hoverbarKeys:{
        'text':{
            menuKeys:["insertLink","through",'bold','clearStyle']
        },
        "image":{
            menuKeys:['imageWidth30', 'imageWidth50', 'imageWidth100', 'editImage', 'deleteImage']
        },
        "link":{
            menuKeys:['editLink', 'unLink']
        },
        "pre":{
            menuKeys:['enter', 'codeBlock']
        },
        "divider":{
            menuKeys:['enter']
        },
        "video":{
            menuKeys:[]
        },
        "table":{
            menuKeys:[]
        },
        
    }
}

const editorRef = shallowRef();
const handleCreated = (editor:any) => {
    editorRef.value = editor
}
//#endregion

//#region  创建注册image上传按钮
// 创建组件类
class upButtonMenu implements IButtonMenu {   // TS 语法
    title:string
    tag:string
    iconSvg:string
    constructor() {
        this.title = '图片' // 自定义菜单标题
        this.iconSvg = '<svg viewBox="0 0 1024 1024"><path d="M828.708571 585.045333a48.761905 48.761905 0 0 0-48.737523 48.761905v18.529524l-72.143238-72.167619a135.972571 135.972571 0 0 0-191.585524 0l-34.133334 34.133333-120.880762-120.953905a138.898286 138.898286 0 0 0-191.585523 0l-72.167619 72.167619V292.400762a48.786286 48.786286 0 0 1 48.761904-48.761905h341.23581a48.737524 48.737524 0 0 0 34.474667-83.285333 48.737524 48.737524 0 0 0-34.474667-14.287238H146.236952A146.212571 146.212571 0 0 0 0 292.400762v585.289143A146.358857 146.358857 0 0 0 146.236952 1024h584.996572a146.212571 146.212571 0 0 0 146.236952-146.310095V633.807238a48.786286 48.786286 0 0 0-48.761905-48.761905zM146.261333 926.45181a48.737524 48.737524 0 0 1-48.761904-48.761905v-174.128762l141.409523-141.458286a38.497524 38.497524 0 0 1 53.126096 0l154.526476 154.624 209.627428 209.724953H146.236952z m633.734096-48.761905c-0.073143 9.337905-3.145143 18.383238-8.777143 25.843809l-219.843048-220.94019 34.133333-34.133334a37.546667 37.546667 0 0 1 53.613715 0l140.873143 141.897143V877.714286zM1009.615238 160.231619L863.329524 13.897143a48.737524 48.737524 0 0 0-16.091429-10.24c-11.849143-4.87619-25.161143-4.87619-37.059047 0a48.761905 48.761905 0 0 0-16.067048 10.24l-146.236952 146.334476a49.005714 49.005714 0 0 0 69.217523 69.241905l62.902858-63.390476v272.627809a48.761905 48.761905 0 1 0 97.475047 0V166.083048l62.902857 63.390476a48.737524 48.737524 0 0 0 69.217524 0 48.761905 48.761905 0 0 0 0-69.241905z"></path></svg>'
        this.tag = 'button'
    }

    // 获取菜单执行时的 value ，用不到则返回空 字符串或 false // 插入的内容
    getValue(_editor: IDomEditor): string | boolean {   // TS 语法
        return false;
    }

    // 菜单是否需要激活（如选中加粗文本，“加粗”菜单会激活），用不到则返回 false
    isActive(editor: IDomEditor): boolean {  // TS 语法
        console.log(editor)
        return false
    }

    // 菜单是否需要禁用（如选中 H1 ，“引用”菜单被禁用），用不到则返回 false
    isDisabled(editor: IDomEditor): boolean {   // TS 语法
        console.log(editor)
        return false
    } 

    // 点击菜单时触发的函数
    async exec(editor: IDomEditor, _value: string | boolean) {   // TS 语法
        if (this.isDisabled(editor)) return
        let res =await open({
            'title':'选择图片',
            "multiple":true,
            'filters':[{
                name:"Image",
                extensions:['png','jpg','jpeg','gif','bmp','svg','apng','webp']
            }]
        })
        res?.filter(async (item)=>{
            let name =await basename(item);
            await copyFile(item,"note\\"+name,{toPathBaseDir:BaseDirectory.AppData})
            let src = await appDataDir() + '\\note\\'+name
            let imgsrc = convertFileSrc(src);
            let img = {
                type: 'image',
                src:imgsrc,
                children: [{ text: '' }],
                style:{
                    width:'100%',
                    height:'auto'
                }
            }
            editor.insertNode(img)
        })
    }
}
const upimage = {
  key: 'upimage', // 定义 menu key ：要保证唯一、不重复（重要）
  factory() {
    return new upButtonMenu() // 把 `YourMenuClass` 替换为你菜单的 class
  },
}
Boot.registerMenu(upimage)
//#endregion

//#region 剪贴板 粘贴事件

const customPaste =async function(editor:any, event:any, callback:any){
    const html = event.clipboardData.getData('text/html') // 获取粘贴的 html
    const text = event.clipboardData.getData('text/plain') // 获取粘贴的纯文本
    // const rtf = event.clipboardData.getData('text/rtf') // 获取 rtf 数据（如从 word wps 复制粘贴）
    // if(rtf){
    //     editor.dangerouslyInsertHtml(rtf)
    // }else 
    if(html){
        editor.dangerouslyInsertHtml(html)
    }else if(text){
        editor.insertText(text)
    }

    let imgarr = [];
    if (event.clipboardData || event.originalEvent) {
        let clipboardData = (event.clipboardData || event.originalEvent.clipboardData);
        if(clipboardData.items){
            for (let i = 0; i < clipboardData.items.length; i++) {
                console.log(clipboardData.items[i])
                if (clipboardData.items[i].type.indexOf("image") !== -1) {
                    let name:string = uuid() + '.' + clipboardData.items[i].type.split('/')[1]
                    imgarr.push({
                        name,
                        file:clipboardData.items[i].getAsFile(),
                        type:'image'
                    })
                }
            }
        } 
    }
 
    for(let i=0;i<imgarr.length;i++){
        let blob = await imgarr[i].file.arrayBuffer()
        await writeFile("note\\"+imgarr[i].name,blob, { "baseDir":BaseDirectory.AppData });
        const appDataDirPath = await appDataDir();
        let src = convertFileSrc(appDataDirPath+"\\note\\"+imgarr[i].name)
        let img = {
                type: 'image',
                src:src,
                children: [{ text: '' }],
                style:{
                    width:'100%',
                    height:'auto'
                }
            }
            editor.insertNode(img)
    }

    imgarr.splice(0,imgarr.length);
    event.preventDefault()
    callback(false) // 返回值（注意，vue 事件的返回值，不能用 return）
}
//#endregion

//#region 右键菜单

// 颜色选择
const colorChange = function(){
    document.getElementById('colorBoard')?.click()
}

// 获取颜色
const colorInput = function(e:any){
    const {red,green,blue } = hexToRgba(e.srcElement.value,1);
    editorData.color = red+','+green+','+blue
    localStorage.setItem(appWindow.label,JSON.stringify(editorData))
    emit("note",editorData)
    setconfig()
}
// 16进制color转10进制
function hexToRgba(hex:string, opacity:number|string) {
    let RGBA = "rgba(" + parseInt("0x" + hex.slice(1, 3)) + "," + parseInt("0x" + hex.slice(3, 5)) + "," + parseInt( "0x" + hex.slice(5, 7)) + "," + opacity + ")";
    return {
        red: parseInt("0x" + hex.slice(1, 3)),
        green: parseInt("0x" + hex.slice(3, 5)),
        blue: parseInt("0x" + hex.slice(5, 7)),
        rgba: RGBA
    }
}

// 透明度
const transparentbool = ref(false);
const rangeChange = function(e:any){
    editorData.opacity = e.srcElement.value
    localStorage.setItem(appWindow.label,JSON.stringify(editorData))
    emit("note",editorData)
    setconfig()
}

// 设置层级
const alwaysicon = ref("mdi-rectangle")
const setalways = function(){
    if(editorData.always == "top"){
        editorData.always = "bottom"
        alwaysicon.value = "mdi-arrange-send-backward"
    }else if(editorData.always == "bottom"){
        editorData.always = "normal"
        alwaysicon.value = "mdi-rectangle"
    }else if(editorData.always == "normal"){
        editorData.always = "top"
        alwaysicon.value = "mdi-arrange-bring-forward"   
    }
    setconfig()
}

// 关闭

const close = async function(){
    const windowstore = windowStore()
    console.log(windowstore.windows)
    let res = windowstore.windows.filter(item=>{
        return item.label == getCurrentWebviewWindow().label
    })
    const notestore = noteStore()
    let index = notestore.note.findIndex(item=>{
        return item.label == getCurrentWebviewWindow().label
    })
    console.log(res)
    if(index>=0){
        notestore.note[index].option = res[0].option
    }
    appWindow.close()
}

// 右键菜单事件
const rightClick = function(){
    transparentbool.value = false
}

//#endregion
</script>

<template>
    <right-bar :border-radius="'0px'" @right-click="rightClick">
        <div style="display: flex;justify-content: space-evenly;align-items: center;width: 100%;height: 100%;" data-tauri-drag-region>
            <v-btn icon="mdi-opacity" size="small"  @click="transparentbool = !transparentbool"></v-btn>
            <v-btn icon="mdi-palette" size="small" @click="colorChange"></v-btn>
            <v-btn :icon="alwaysicon" size="small" @click="setalways"></v-btn>
            <v-btn icon="mdi-close" size="small"  @click="close()"></v-btn>
        </div>
    </right-bar>
        <!-- 背景色选择 -->
        <input type="color" v-show="true" id="colorBoard" class="color-board" @input="colorInput">
        <!-- 透明度 -->
        <input type="range" v-show="transparentbool" @input="rangeChange" id="transparent" class="transparent" step="1" max="100" min="0">
    <div class="editor-container">
        <Toolbar
        v-show="show"
        class="toolbar"
        :editor="editorRef"
        :defaultConfig="toolbarConfig"
        :mode="mode"
      />
      <Editor
        :style="{height:show?'calc( 100vh - 40px ) !important':'100vh !important'}"
        class="editor"
        v-model="editorData.value"
        :defaultConfig="editorConfig"
        :mode="mode"
        @onCreated="handleCreated"
        @customPaste = "customPaste"
        @onChange="onChange"
      />
    </div>
</template>
<style>

/* 透明度 */
.transparent{
    position: absolute;
    z-index: 210;
    top: 10vh;
    width: 80vw;
    margin-left: 10vw;
}
/* 色板 */
.color-board{
    position: absolute;
    bottom: 0px;
    opacity: 0;
}

.editor-container{
    width: 100vw;
    height: 100vh;
    overflow: hidden;
}

/* tool */
.toolbar{
    width: 100vw;
    height: 40px;
}
/* editor */
.editor{
    width: 100vw;
}

p {
    margin-block-start: 0em !important;
    margin-block-end: 0em !important;
}

::-webkit-scrollbar{
    display: none;
}
</style>