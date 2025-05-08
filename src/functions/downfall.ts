// import { sakurabase64 } from "./sakura"

// let list: Down[] = []
// let canvas = document.createElement("canvas")
// canvas.style.position = "absolute"
// canvas.style.zIndex = "501"
// canvas.style.left = "0px"
// canvas.style.top = "0px"
// canvas.width = window.innerWidth
// canvas.height = window.innerHeight
// let cxt = canvas.getContext("2d") as CanvasRenderingContext2D
// class Down {
//   public image: HTMLImageElement
//   public x: number = 0 // 初始x坐标
//   public y: number = 0 // 初始y坐标
//   public size: number = 0 // 初始大小
//   public speedX: number = 0 // x轴速度
//   public speedY: number = 0 // y轴速度
//   public rotateDeg: number = 0 // 旋转初始角度
//   public rotateSpeed: number = 0 // 旋转速度
//   public width: number
//   public height: number
//   public cxt: CanvasRenderingContext2D
//   constructor(cxt: CanvasRenderingContext2D, image: HTMLImageElement, width: number, height: number) {
//     this.width = width
//     this.height = height
//     this.image = image
//     this.init()
//     this.cxt = cxt
//   }

//   protected init() {
//     this.x = Math.random() * this.width - 100
//     this.y = Math.random() * this.height
//     this.size = 10 + Math.random() * 50
//     this.speedX = -Math.random() * 0 + -1
//     this.speedY = Math.random() + 20
//     this.rotateDeg = Math.random() * 360
//     this.rotateSpeed = Math.random() * 2 - 1
//   }

//   public update() {
//     this.x += this.speedX
//     this.y += this.speedY
//     this.rotateDeg += this.rotateSpeed

//     if (this.y > this.height) {
//       this.init()
//       this.y = -this.size
//     }
//   }

//   public draw(fn?: (e: CanvasRenderingContext2D) => void) {
//     if (fn) {
//       fn(this.cxt)
//     } else {
//       this.cxt.save()
//       this.cxt.translate(this.x, this.y)
//       this.cxt.rotate((this.rotateDeg * Math.PI) / 180)
//       this.cxt.drawImage(this.image, this.size / 2, this.size / 2, this.size, this.size)
//       this.cxt.restore()
//     }
//   }
// }

// const start = function () {
//   let img = new Image()
//   img.src = sakurabase64
//   document.body.appendChild(canvas)
//   for (let i = 0; i <= 100; i++) {
//     list.push(new Down(cxt, img, window.innerWidth, window.innerHeight))
//   }
//   document.body.appendChild(canvas)
//   console.log(list)
//   animate1()
// }
// const animate1 = function () {
//   cxt.clearRect(0, 0, window.innerWidth, window.innerHeight)

//   for (let i = 0; i <= list.length - 1; i++) {
//     list[i].update()
//     list[i].draw((cxt) => {
//       cxt.save()
//       cxt.imageSmoothingEnabled = true
//       cxt.translate(list[i].x, list[i].y)
//       cxt.rotate((list[i].rotateDeg * Math.PI) / 180)
//       cxt.drawImage(list[i].image, list[i].size / 2, list[i].size / 2, list[i].size, list[i].size)
//       cxt.restore()
//     })
//   }
//   requestAnimationFrame(animate1)
// }
