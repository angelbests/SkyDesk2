import { fetch } from "@tauri-apps/plugin-http"
// 9cda7ed49a914d5eb6987706d642da65
import { weatherStore } from "../stores/weather"
import { info } from "@tauri-apps/plugin-log"
export const getpoi = async function (city: string): Promise<any> {
  const w = weatherStore()
  if (w.apikey && city) {
    let respone = await fetch(`https://geoapi.qweather.com/v2/city/lookup?location=${city}&key=${w.apikey}&gzip=n`, {
      method: "GET",
    })
    if (respone.ok) {
      let json = await respone.json()
      console.log(json)
      return json.location
    }
  }
  return []
}
// 9cda7ed49a914d5eb6987706d642da65
export const getNowWeather = async function (location: string): Promise<any> {
  const w = weatherStore()
  let header = new Headers()
  header.append("content-encoding", "gzip")
  let respone = await fetch(`https://devapi.qweather.com/v7/weather/now?location=${location}&key=${w.apikey}`, {
    method: "GET",
    headers: header,
  })
  console.log(respone)
  try {
    let isgzip = false
    if (respone.ok) {
      // 解压gzip数据
      for (const pair of respone.headers.entries()) {
        // info(`${pair[0]}: ${pair[1]}`)
        if (pair[0] == "content-encoding" && pair[1] == "gzip") {
          isgzip = true
        }
      }
      // 解压Gzip
      if (isgzip) {
        let blob = await respone.blob()
        let ds = new DecompressionStream("gzip")
        const de = blob.stream().pipeThrough(ds)
        let json = await new Response(de).json()
        return json
      } else {
        let json = await respone.json()
        return json
      }
    }
  } catch (error) {
    info("weather_Error: " + error)
  }
}

// 9cda7ed49a914d5eb6987706d642da65
export const getD7Weather = async function (location: string): Promise<any> {
  const w = weatherStore()
  let header = new Headers()
  header.append("content-encoding", "gzip")
  let respone = await fetch(`https://devapi.qweather.com/v7/weather/7d?location=${location}&key=${w.apikey}`, {
    method: "GET",
    headers: header,
  })
  console.log(respone)
  try {
    let isgzip = false
    if (respone.ok) {
      // 解压gzip数据
      for (const pair of respone.headers.entries()) {
        // info(`${pair[0]}: ${pair[1]}`)
        if (pair[0] == "content-encoding" && pair[1] == "gzip") {
          isgzip = true
        }
      }
      // 解压Gzip
      if (isgzip) {
        let blob = await respone.blob()
        let ds = new DecompressionStream("gzip")
        const de = blob.stream().pipeThrough(ds)
        let json = await new Response(de).json()
        return json
      } else {
        let json = await respone.json()
        return json
      }
    }
  } catch (error) {
    info("weather_Error: " + error)
  }
}
