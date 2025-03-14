import { getWeather } from "../api/weather"
import { weatherStore } from "../stores/window"

type WeatherType = {
  temp: string // 温度
  feelsLike: string // 体感温度
  icon: string // 天气图标
  text: string // 文本描述
  windDir: string // 风向
  windScale: string // 风等级
  windSpeed: string // 风速
  humidity: string // 相对湿度
  precip: string // 过去1小时降水量，默认单位：毫米
  pressure: string // 大气压强
  vis: string // 能见度
  cloud: string // 云量
  dew: string // 露点温度
  city: string
}

async function weather_wallpaper(fn: (e: WeatherType) => void): Promise<WeatherType> {
  const weatherstore = weatherStore()
  let { city, apikey, citycode } = weatherstore
  let unlisten = setInterval(
    async () => {
      if (apikey && city) {
        let res = await getWeather(citycode)
        if (res.code == 200) {
          fn({
            ...res.now,
            city: city,
          })
        }
      } else {
        clearInterval(unlisten)
      }
    },
    60 * 60 * 60,
  )
  if (apikey && city) {
    let res = await getWeather(citycode)
    if (res.code == 200) {
      return {
        ...res.now,
        city: city,
      } as WeatherType
    }
  }
  // Ensure a default return value in case no other return is reached
  return {
    temp: "",
    feelsLike: "",
    icon: "",
    text: "",
    windDir: "",
    windScale: "",
    windSpeed: "",
    humidity: "",
    precip: "",
    pressure: "",
    vis: "",
    cloud: "",
    dew: "",
    city: "",
  } as WeatherType
}

export { weather_wallpaper }
export type { WeatherType }
