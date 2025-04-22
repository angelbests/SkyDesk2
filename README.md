# SkyDesk2 - 一个简单易用的本地工具

![Static Badge](https://img.shields.io/badge/Tauri-2-blue?logo=tauri)
![Static Badge](https://img.shields.io/badge/Rust-1.81.0-blue?logo=rust)
![Static Badge](https://img.shields.io/badge/vue3-vuetifyjs-gray)

## 下载

[SkyDesk2下载](https://github.com/angelbests/SkyDesk2/releases)

## 开发

此项目需要安装 rust 环境

```
// npm pkg install
npm install
// dev
npm run tauri dev
// build
npm run tauri build
```

## 功能介绍

### 快捷

扫描：扫描电脑内的快捷文件，并抓取对应程序的程序图标和名称。通过拖拽的方法拖拽到右侧的快捷栏。

添加：添加本地程序到快捷列表

设置：删除tab，删除快捷，修改快捷

合计：添加合集tab

![](./screenshot/2.png)

![](./screenshot/1.png)

添加桌面合集：将快捷栏中的程序拖拽的桌面合集中即可，拖回主程序删除图标，通过鼠标右键点击弹起自定义配置，自定义桌面合集的样式和大小

![](./screenshot/13.png)

鼠标轮盘：点按鼠标中键启动，滑向对应的程序图标既可启动程序。

![](./screenshot/3.png)

快捷搜索：组合键 Alt+Space，可搜索windows本地文件和程序等；效果跟随windows 搜索设置。

输入搜索文本+回车->调起本地浏览器，使用bing网络搜索

输入搜索文本+键盘up 或 键盘 down+回车->调起本地搜索的选择项

![](./screenshot/16.png)

![](./screenshot/17.png)

windows搜索设置：设置->隐私和安全性->搜索windows;

![](./screenshot/18.png)

![](./screenshot/19.png)

### 壁纸

添加：添加静态壁纸、视频壁纸、网页壁纸，适应多屏。可设置锁屏。

关闭：关闭所有壁纸

在线壁纸：在线下载静态壁纸；壁纸源 [wallhaven](https://wallhaven.cc/) ，壁纸界面右键可弹出搜索选项，壁纸下载在windows->图片->skydesk2文件夹，添加的壁纸会自动添加到列表

天气：配置天气api的key，以及配置城市；天气api：[和风天气](https://www.qweather.com/)

![](./screenshot/4.png)

![](./screenshot/7.png)

设置：配置壁纸的天气，音频，音乐，时间等显示。

音乐支持网易云音乐（需要betterncm支持并安装smtc插件），QQ音乐,Spotify

点击桌面封面控制音乐的播放和暂停

在封面上按住左键向右滑动并释放，切换下一曲

在封面上按住左键向左滑动并释放，切换上一曲

![](./screenshot/20.png)

### 便签

添加桌面便签，输入内容或拖拽图片进入即可保存对应信息，鼠标右键点击弹起自定义配置选项。

自定义配置包含背景色配置，透明度配置，窗口置顶，置底和正常配置。

映入：将便签嵌入到windows 壁纸层。
映出：将便签恢复到windows 桌面层。

![](./screenshot/5.png)

![](./screenshot/6.png)

### 日历

鼠标在日历区域滚动滑轮切换月份，悬浮在年份上滚动切换年份

![](./screenshot/12.png)

### AI

需要安装 ollama，并下载安装对应的 AI 模型。

环境变量配置：

变量名：OLLAMA_HOST 变量值:0.0.0.0

变量名：OLLAMA_ORIGINS 变量值：\*

![](./screenshot/10.png)

### 录屏

录屏可以随意框选录屏范围，由于个人能力问题，录屏暂无法录制声音。

![](./screenshot/11.png)

### 快捷按键

程序托盘状态下可使用 CTRL+1 调起，CTRL+2 隐藏。

### 任务栏和网速

任务栏：根据壁纸桌面1配置 控制音乐的播放、上一曲、下一曲。音乐程序关闭状态下，点击播放控制可打开对应的音乐程序（需要扫描桌面快捷）

![](./screenshot/21.png)

网速
![](./screenshot/9.png)

### 设置主程序界面和配置

![](./screenshot/15.png)

![](./screenshot/22.png)

![](./screenshot/23.png)

## 感谢

#### 前端库

vue
vue-router
vue-draggable-plus
vuetify
pinia
pinia-plugin-persistedstate
wangeditor5
ollama
js-calendar-converter
fabric
markdown-it

#### rust 库

windows-rs
rdev
sysinfo
image
windows-capture
tiny_http
chrono

#### sidecar

[resources_extract](https://www.nirsoft.net/utils/resources_extract.html)

## License

This project is licensed under the [MIT License](LICENSE).
