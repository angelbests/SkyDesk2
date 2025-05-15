import { SolarDay } from "tyme4ts"

export type DayInfo = {
  constellation: string | null | undefined
  lunar_day: string | null | undefined
  lunar_festival: string | null | undefined
  lunar_month: string | null | undefined
  lunar_season: string | null | undefined
  lunar_term: string | null | undefined
  lunar_year: string | null | undefined
  solar_festival: string | null | undefined
  solar_year: number
  solar_month: number
  solar_day: number
}

export const today = function (year: number, month: number, day: number) {
  let solar = SolarDay.fromYmd(year, month, day)
  let festival = null
  if (solar.getFestival()) {
    festival = solar.getFestival()?.getName()
    if (festival == "五一劳动节") {
      festival = festival.replace("五一", "")
    } else if (festival == "五四青年节") {
      festival = festival.replace("五四", "")
    } else if (festival == "八一建军节") {
      festival = festival.replace("八一", "")
    } else if (festival == "六一儿童节") {
      festival = festival.replace("六一", "")
    } else if (festival == "三八妇女节") {
      festival = festival.replace("三八", "")
    }
  }
  // 冬月 11月  腊月 12月
  let lunar_month = solar.getLunarDay().getLunarMonth().getName()
  // if (lunar_month == "一月") {
  //   lunar_month = "正月"
  // } else if (lunar_month == "二月") {
  //   lunar_month = "杏月"
  // } else if (lunar_month == "三月") {
  //   lunar_month = "桃月"
  // } else if (lunar_month == "四月") {
  //   lunar_month = "槐月"
  // } else if (lunar_month == "五月") {
  //   lunar_month = "蒲月"
  // } else if (lunar_month == "六月") {
  //   lunar_month = "荷月"
  // } else if (lunar_month == "七月") {
  //   lunar_month = "巧月"
  // } else if (lunar_month == "八月") {
  //   lunar_month = "桂月"
  // } else if (lunar_month == "九月") {
  //   lunar_month = "菊月"
  // } else if (lunar_month == "十月") {
  //   lunar_month = "孟冬"
  // } else
  if (lunar_month == "十一月") {
    lunar_month = "冬月"
  } else if (lunar_month == "十二月") {
    lunar_month = "腊月"
  }
  let today: DayInfo = {
    constellation: solar.getConstellation().toString(),
    lunar_day: solar.getLunarDay().getName(),
    lunar_festival: solar.getLunarDay().getFestival() ? solar.getLunarDay().getFestival()?.getName() : null,
    lunar_month,
    lunar_season: solar.getLunarDay().getLunarMonth().getSeason().getName(),
    lunar_term: solar.getTermDay().getDayIndex() == 0 ? solar.getTermDay().getName() : null,
    lunar_year: solar.getLunarDay().getLunarMonth().getLunarYear().getName(),
    solar_festival: festival,
    solar_year: year,
    solar_month: month,
    solar_day: day,
  }
  return today
}

export type Festival = {
  festival: string | null | undefined
  until: number
  year: number
  month: number
  day: number
}

export const get_all_festival = function (year: number, month: number, day: number) {
  let festivals: Festival[] = []
  let solar = SolarDay.fromYmd(year, month, day)
  for (let i = 0; i < 366; i++) {
    if (solar.getFestival()) {
      let festival = solar.getFestival()?.getName()
      if (festival == "五一劳动节") {
        festival = festival.replace("五一", "")
      } else if (festival == "五四青年节") {
        festival = festival.replace("五四", "")
      } else if (festival == "八一建军节") {
        festival = festival.replace("八一", "")
      } else if (festival == "六一儿童节") {
        festival = festival.replace("六一", "")
      } else if (festival == "三八妇女节") {
        festival = festival.replace("三八", "")
      }
      festivals.push({
        until: i,
        festival,
        year,
        month,
        day,
      })
    }
    solar = solar.next(1)
  }
  solar = SolarDay.fromYmd(year, month, day)
  for (let i = 0; i < 366; i++) {
    if (solar.getLunarDay().getFestival()) {
      let festival = solar.getFestival()?.getName()

      festivals.push({
        until: i,
        festival,
        year,
        month,
        day,
      })
    }
    solar = solar.next(1)
  }
  return festivals
}
