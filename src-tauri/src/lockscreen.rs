use windows::{core::HSTRING, Storage::StorageFile, System::UserProfile::LockScreen};

#[tauri::command]
pub fn setlockscreen(path: String) {
    tauri::async_runtime::spawn(async move {
        let file_op = StorageFile::GetFileFromPathAsync(&HSTRING::from(path))
            .unwrap()
            .get()
            .unwrap();
        LockScreen::SetImageFileAsync(&file_op).unwrap();
    });
}
