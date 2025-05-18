import { convertFileSrc } from "@tauri-apps/api/core"
import { writeFile } from "@tauri-apps/plugin-fs"

export const getvideothumb = function (file: string, savepath: string): Promise<string> {
  return new Promise((resolve, reject) => {
    let url = convertFileSrc(file)
    const video = document.createElement("video")
    video.src = url
    video.crossOrigin = "anonymous"
    video.muted = true
    video.playsInline = true
    video.addEventListener("loadeddata", () => {
      video.currentTime = 5
    })
    let canvas = document.createElement("canvas")
    video.addEventListener("seeked", () => {
      // 创建 canvas
      canvas.width = video.videoWidth
      canvas.height = video.videoHeight
      const ctx = canvas.getContext("2d")
      if (!ctx) {
        return reject("")
      }
      ctx.drawImage(video, 0, 0, canvas.width, canvas.height)
      canvas.toBlob(async (blob) => {
        if (blob) {
          let buffer = await blob.arrayBuffer()
          await writeFile(savepath, new Uint8Array(buffer))
          resolve(savepath)
        } else {
          reject("")
        }
      }, "image/png")
    })
  })
}
