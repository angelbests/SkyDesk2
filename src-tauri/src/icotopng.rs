use image::ImageReader;
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