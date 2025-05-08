import type { Event } from "@tauri-apps/api/event"
import { Monitor } from "./storeType"
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
  monitor: Monitor
}

export type { MouseEvent, Event }
export { MouseAction }
