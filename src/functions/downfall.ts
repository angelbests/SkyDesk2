export class Downfall {
  public image: HTMLImageElement
  public x: number = 0 // 初始x坐标
  public y: number = 0 // 初始y坐标
  public size: number = 0 // 初始大小
  public speedX: number = 0 // x轴速度
  public speedY: number = 0 // y轴速度
  public rotateDeg: number = 0 // 旋转初始角度
  public rotateSpeed: number = 0 // 旋转速度
  public width: number
  public height: number
  public cxt: CanvasRenderingContext2D
  constructor(cxt: CanvasRenderingContext2D, image: HTMLImageElement, width: number, height: number, size: number) {
    this.width = width
    this.height = height
    this.image = image
    this.x = Math.random() * this.width - 100
    this.y = -10
    this.size = 1 + Math.random() * size
    this.speedX = Math.random() * 1 + 1
    this.speedY = Math.random() * 1 + 1
    this.rotateSpeed = Math.random() * 1 + 1
    this.rotateDeg = Math.random() * 360 - 1
    this.cxt = cxt
  }

  public setSpeed(fn: () => void) {
    fn()
  }

  public update() {
    this.x += this.speedX
    this.y += this.speedY
    this.rotateDeg += this.rotateSpeed
    if (this.y > this.height) {
      this.y = -100
      this.x = Math.random() * this.width - 100
    }
  }

  public draw(fn?: (e: CanvasRenderingContext2D) => void) {
    if (fn) {
      fn(this.cxt)
    } else {
      this.cxt.save()
      this.cxt.translate(this.x, this.y)
      this.cxt.rotate((this.rotateDeg * Math.PI) / 180)
      this.cxt.drawImage(this.image, this.size / 2, this.size / 2, this.size, this.size)
      this.cxt.restore()
    }
  }
}
