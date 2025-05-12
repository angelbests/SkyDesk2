use std::{mem::zeroed, ptr::copy_nonoverlapping};

use clipboard_win::{formats, Clipboard, Setter};
use windows::Win32::{
    Foundation::HGLOBAL,
    System::{
        Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE},
        Ole::CF_HDROP,
    },
    UI::Shell::DROPFILES,
};

#[tauri::command]
pub fn copyfile(path: &str) {
    let mut wide_paths: Vec<u16> = path.encode_utf16().collect();
    wide_paths.push(0); // 额外的 null 终止整个列表

    // 计算所需的内存大小
    let dropfiles_size = size_of::<DROPFILES>();
    let paths_size = wide_paths.len() * size_of::<u16>();
    let total_size = dropfiles_size + paths_size;

    // 分配全局内存
    unsafe {
        let hglobal: HGLOBAL =
            GlobalAlloc(GMEM_MOVEABLE, total_size).expect("Failed to allocate global memory");

        let ptr = GlobalLock(hglobal) as *mut u8;

        // 填写 DROPFILES 结构体
        let dropfiles = ptr as *mut DROPFILES;
        *dropfiles = zeroed();
        (*dropfiles).pFiles = dropfiles_size as u32;
        (*dropfiles).fWide = true.into(); // 使用 Unicode

        // 复制文件路径数据
        let data_ptr = ptr.add(dropfiles_size) as *mut u16;
        copy_nonoverlapping(wide_paths.as_ptr(), data_ptr, wide_paths.len());

        let _ = GlobalUnlock(hglobal);

        // 打开剪贴板并设置数据
        let _clip = Clipboard::new_attempts(10);
        let data_ptr = GlobalLock(hglobal) as *const u8;
        let data_slice = std::slice::from_raw_parts(data_ptr, total_size);
        let _ = formats::RawData(CF_HDROP.0 as u32).write_clipboard(&data_slice);
        let _ = GlobalUnlock(hglobal);
    }
}
