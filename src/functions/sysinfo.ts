import { listen, UnlistenFn, Event } from "@tauri-apps/api/event"

type NetSpeed = {
  speed_r: 0
  speed_s: 0
}
class Netspeed {
  netspeed_unlisten: UnlistenFn | undefined
  async listen_netspeed(fn: (e: Event<NetSpeed>) => void) {
    this.netspeed_unlisten = await listen("netspeed", (e: Event<NetSpeed>) => {
      fn(e)
    })
  }

  cancel_listen_netspeed() {
    if (this.netspeed_unlisten) {
      this.netspeed_unlisten()
      this.netspeed_unlisten = undefined
    }
  }
}
export { Netspeed }
export type { NetSpeed }
