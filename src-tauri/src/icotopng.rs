use image::{imageops::FilterType, ImageReader};
#[tauri::command]
pub async fn ico_to_png(from: String, to: String) -> i32 {
    let mut bool = 0;
    let img = ImageReader::open(from.clone()).unwrap();
    let de = img.decode();
    match de {
        Ok(d) => {
            d.save(to.clone()).unwrap();
            bool = 1;
        }
        _ => {
            println!("{to},{from}");
        }
    }
    bool
}

#[tauri::command]
pub async fn zipimage(imgpath: String, savepath: String) -> String {
    let img = ImageReader::open(&imgpath).unwrap();
    let de = img.decode();
    match de {
        Ok(d) => {
            let w = d.width();
            let h = d.height();
            if w > 500 {
                let e = d.resize(400, w / 400 * h as u32, FilterType::Triangle);
                e.save(&savepath).unwrap();
                return savepath;
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
    return imgpath;
}
