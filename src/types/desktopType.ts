import type { Event } from "@tauri-apps/api/event"
enum MouseAction {
  LeftDown = "LeftDown",
  LeftUp = "LeftUp",
  RightDown = "RightDown",
  RightUp = "RightUp",
  MiddleDown = "MiddleDown",
  MiddleUp = "MiddleUp",
  Move = "Move",
  Wheel = "Wheel",
  None = "None",
}
type MouseEvent = {
  x: number
  y: number
  mouse: MouseAction
}

export type { MouseEvent, Event }
export { MouseAction }
