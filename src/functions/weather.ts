import { getNowWeather, getD7Weather } from "../api/weather"
import { weatherStore } from "../stores/weather"

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

type WeatherD7Type = {
  cloud: string // 云量，百分比数值。可能为空
  fxDate: string //预报日期
  humidity: string // 相对湿度，百分比数值
  iconDay: string // 预报白天天气状况的图标代码
  iconNight: string // 预报夜间天气状况的图标代码
  moonPhase: string //  月相名称
  moonPhaseIcon: string // 月相图标代码
  moonrise: string // 当天月升时间
  moonset: string //  当天月落时间
  precip: string // 预报当天总降水量，默认单位：毫米
  pressure: string // 大气压强，默认单位：百帕
  sunrise: string // 日出时间，在高纬度地区可能为空
  sunset: string // 日落时间，在高纬度地区可能为空
  tempMax: string // 预报当天最高温度
  tempMin: string //  预报当天最低温度
  textDay: string // 预报白天天气状况文字描述，包括阴晴雨雪等天气状态的描述
  textNight: string // 预报晚间天气状况文字描述，包括阴晴雨雪等天气状态的描述
  uvIndex: string // 紫外线强度指数
  vis: string // 能见度，默认单位：公里
  wind360Day: string // 预报白天风向360角度
  wind360Night: string //  预报夜间风向360角度
  windDirDay: string // 预报白天风向
  windDirNight: string //  预报夜间当天风向
  windScaleDay: string //  预报白天风力等级
  windScaleNight: string // 预报夜间风力等级
  windSpeedDay: string // 报白天风速，公里/小时
  windSpeedNight: string // 预报夜间风速，公里/小时
}

async function weatherNowWallpaper(fn: (e: WeatherType) => void): Promise<WeatherType> {
  const weatherstore = weatherStore()
  let { city, apikey, citycode } = weatherstore
  // 定时请求
  let unlisten = setInterval(
    async () => {
      if (apikey && city) {
        let res = await getNowWeather(citycode)
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
    1000 * 60 * 60,
  )

  // 第一次请求
  if (apikey && city) {
    let res = await getNowWeather(citycode)
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

async function weatherD7Wallpaper(fn: (e: { city: string; daily: WeatherD7Type[] }) => void): Promise<{
  city: string
  daily: WeatherD7Type[]
}> {
  const weatherstore = weatherStore()
  let { city, apikey, citycode } = weatherstore
  // 定时请求
  let unlisten = setInterval(
    async () => {
      if (apikey && city) {
        let res = await getD7Weather(citycode)
        if (res.code == 200) {
          fn({
            daily: res.daily,
            city: city,
          })
        }
      } else {
        clearInterval(unlisten)
      }
    },
    1000 * 60 * 60,
  )

  // 第一次请求
  if (apikey && city) {
    let res = await getD7Weather(citycode)
    console.log(res)
    if (res.code == 200) {
      return {
        daily: res.daily,
        city: city,
      }
    }
  }
  // Ensure a default return value in case no other return is reached
  return {
    city: city,
    daily: [],
  }
}
export { weatherNowWallpaper, weatherD7Wallpaper }
export type { WeatherType, WeatherD7Type }
