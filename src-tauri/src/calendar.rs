use tyme4rs::tyme::{solar::SolarDay, Culture, Tyme};
#[derive(Debug, serde::Serialize)]
pub struct Festival {
    festival: String,
    until: i32,
    year: isize,
    month: usize,
    day: usize,
}
#[derive(Debug, serde::Serialize)]
pub struct TodayInfo {
    lunar_year: Option<String>,
    lunar_month: Option<String>,
    lunar_day: Option<String>,
    lunar_season: Option<String>,
    lunar_festival: Option<String>,
    lunar_term: Option<String>,
    constellation: Option<String>,
    solar_festival: Option<String>,
}

#[tauri::command]
pub async fn today(year: isize, month: usize, day: usize) -> Result<TodayInfo, String> {
    // 元旦节、春节、清明节、劳动节、端午节、中秋节、国庆节、国庆中秋、抗战胜利日
    // 元旦、三八妇女节、植树节、五一劳动节、五四青年节、六一儿童节、建党节、八一建军节、教师节、国庆节
    // 春节、元宵节、龙头节、上巳节、清明节、端午节、七夕节、中元节、中秋节、重阳节、冬至节、腊八节、除夕
    // println!("公历:{:?}", solar.to_string());
    // println!("农历节日:{:?}", get_lunar_festival(solar));
    // println!("农历季节:{:?}", get_season(solar));
    // println!("农历年:{:?}", get_lunar_year(solar));
    // println!("农历月:{:?}", get_lunar_month(solar));
    // println!("农历日:{:?}", get_lunar_day(solar));
    // println!("农历24节气:{:?}", get_term_day(solar));
    // println!("星座:{}", solar.get_constellation());
    // println!("公历现代节日:{:?}", get_festival(solar));
    // println!("法定假日:{:?}", get_legal_holiday(solar));
    let solar: SolarDay = SolarDay::from_ymd(year, month, day);
    return Ok(TodayInfo {
        lunar_year: get_lunar_year(solar),
        lunar_month: get_lunar_month(solar),
        lunar_day: get_lunar_day(solar),
        lunar_season: get_season(solar),
        lunar_festival: get_lunar_festival(solar),
        lunar_term: get_term_day(solar),
        constellation: Some(solar.get_constellation().to_string()),
        solar_festival: get_festival(solar),
    });
}

#[tauri::command]
pub async fn get_all_festival(
    year: isize,
    month: usize,
    day: usize,
) -> Result<Vec<Festival>, String> {
    let mut num = 0;
    let mut solar = SolarDay::from_ymd(year, month, day);
    let mut vec: Vec<Festival> = vec![];
    loop {
        if let Some(name) = get_festival(solar) {
            vec.push(Festival {
                until: num,
                festival: name,
                year: solar.get_year(),
                month: solar.get_month(),
                day: solar.get_day(),
            });
        }
        solar = solar.next(1);
        num += 1;
        if num > 366 {
            break;
        }
    }

    solar = SolarDay::from_ymd(year, month, day);
    num = 0;
    loop {
        if let Some(name) = get_lunar_festival(solar) {
            vec.push(Festival {
                until: num,
                festival: name,
                year: solar.get_year(),
                month: solar.get_month(),
                day: solar.get_day(),
            });
        }
        solar = solar.next(1);
        num += 1;
        if num > 366 {
            break;
        }
    }

    return Ok(vec);
}

// 节气
fn get_term_day(solar: SolarDay) -> Option<String> {
    let term = solar.get_term_day();
    if term.get_day_index() == 0 {
        return Some(term.get_name());
    } else {
        return None;
    }
}
//法定假日
// fn get_legal_holiday(solar: SolarDay) -> Option<String> {
//     let holiday = solar.get_legal_holiday();
//     if let Some(holiday) = holiday {
//         return Some(holiday.get_name());
//     } else {
//         return None;
//     }
// }

//公历现代节日
fn get_festival(solar: SolarDay) -> Option<String> {
    let festival = solar.get_festival();
    if let Some(festival) = festival {
        return Some(festival.get_name());
    } else {
        return None;
    }
}
//农历节日
fn get_lunar_festival(solar: SolarDay) -> Option<String> {
    let festival = solar.get_lunar_day().get_festival();
    if let Some(festival) = festival {
        return Some(festival.get_name());
    } else {
        return None;
    }
}

//农历季节
fn get_season(solar: SolarDay) -> Option<String> {
    return Some(
        solar
            .get_lunar_day()
            .get_lunar_month()
            .get_season()
            .get_name(),
    );
}

// 农历年
fn get_lunar_year(solar: SolarDay) -> Option<String> {
    return Some(
        solar
            .get_lunar_day()
            .get_lunar_month()
            .get_lunar_year()
            .get_name(),
    );
}

//农历月
fn get_lunar_month(solar: SolarDay) -> Option<String> {
    return Some(solar.get_lunar_day().get_lunar_month().get_name());
}

// 农历日
fn get_lunar_day(solar: SolarDay) -> Option<String> {
    return Some(solar.get_lunar_day().get_name());
}
