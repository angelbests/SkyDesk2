type TimeWallpaper = {
  year: string
  month: string
  day: string
  week: string
  hour: string
  minute: string
  second: string
}

function get_time(): TimeWallpaper {
  let date = new Date()
  let week = ""
  switch (date.getDay()) {
    case 0:
      week = "星期天"
      break
    case 1:
      week = "星期一"
      break
    case 2:
      week = "星期二"
      break
    case 3:
      week = "星期三"
      break
    case 4:
      week = "星期四"
      break
    case 5:
      week = "星期五"
      break
    case 6:
      week = "星期六"
      break
  }

  return {
    year: date.getFullYear() + "",
    month: date.getMonth() + 1 > 9 ? date.getMonth() + 1 + "" : "0" + (date.getMonth() + 1),
    day: date.getDate() < 10 ? "0" + date.getDate() : date.getDate() + "",
    week: week,
    hour: date.getHours() < 10 ? "0" + date.getHours() : date.getHours() + "",
    minute: date.getMinutes() < 10 ? "0" + date.getMinutes() : date.getMinutes() + "",
    second: date.getSeconds() < 10 ? "0" + date.getSeconds() : date.getSeconds() + "",
  } as TimeWallpaper
}

export { get_time }
export type { TimeWallpaper }
