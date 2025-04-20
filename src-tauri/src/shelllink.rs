use std::path::Path;
use tauri::{AppHandle, Manager};
use windows::core::BOOL;
use windows::core::{Interface, PCWSTR};
use windows::Win32::Foundation::{FreeLibrary, HMODULE, MAX_PATH};
use windows::Win32::Storage::FileSystem::WIN32_FIND_DATAW;
use windows::Win32::System::Com::{CoUninitialize, STGM_READ};
use windows::Win32::System::LibraryLoader::{
    EnumResourceNamesW, FindResourceW, LoadLibraryExW, LoadResource, LockResource, SizeofResource,
    LOAD_LIBRARY_AS_DATAFILE,
};
use windows::Win32::UI::WindowsAndMessaging::{RT_GROUP_ICON, RT_ICON};
use windows::Win32::{
    System::Com::{
        CoCreateInstance, CoInitializeEx, IPersistFile, CLSCTX_INPROC_SERVER,
        COINIT_APARTMENTTHREADED,
    },
    UI::Shell::{IShellLinkW, ShellLink},
};

#[repr(C)]
#[derive(Debug, Clone)]
struct IconDir {
    reserved: u16,
    r#type: u16,
    count: u16,
}

#[repr(C)]
#[derive(Debug, Clone)]
struct IconDirEntry {
    width: u8,
    height: u8,
    color_count: u8,
    reserved: u8,
    planes: u16,
    bit_count: u16,
    bytes_in_res: u32,
    id: u16, // RT_ICON resource ID
}

#[tauri::command]
pub fn shelllink(app: AppHandle, path: String) {
    unsafe {
        let filename = app.path().file_name(&path).unwrap();
        let c: Vec<&str> = filename.split(".").collect();
        let name = c[0];
        print!("name:{}", name);
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let shell_link: IShellLinkW =
            CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER).unwrap();
        let persist_file: IPersistFile = shell_link.cast().unwrap();

        let wide_path: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();
        let _ = persist_file.Load(PCWSTR(wide_path.as_ptr()), STGM_READ);

        let mut icon_path: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
        let mut icon_index: i32 = 0;
        shell_link
            .GetIconLocation(&mut icon_path, &mut icon_index)
            .unwrap();
        let mut icon_path_str = String::from_utf16_lossy(&icon_path);
        let str = icon_path_str.trim_end_matches('\0');
        println!("{}", str.is_empty());
        if str.is_empty() {
            let mut find_data = WIN32_FIND_DATAW::default();
            let mut path: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
            shell_link.GetPath(&mut path, &mut find_data, 0).unwrap();
            icon_path_str = String::from_utf16_lossy(&path);
            let str = icon_path_str.trim_end_matches('\0');
            if str.is_empty() {
                return;
            }
            icon_path = path;
        }
        println!(
            "path:{},index:{}",
            icon_path_str.trim_end_matches('\0'),
            icon_index
        );
        let icopath = icon_path_str.trim_end_matches('\0');
        let filename = app.path().file_name(icopath).unwrap();
        let c: Vec<&str> = filename.split(".").collect();

        let ext = c[1];
        println!("{:?}", ext);
        println!("{:?}", icopath);
        let path = Path::new(&icopath);
        let bool = path.exists();
        if !bool {
            return;
        }
        if ext == "ico" {
            return;
        } else if ext == "exe" || ext == "dll" {
            let hmodule =
                LoadLibraryExW(PCWSTR(icon_path.as_ptr()), None, LOAD_LIBRARY_AS_DATAFILE).unwrap();

            // https://learn.microsoft.com/zh-cn/windows/win32/api/libloaderapi/nf-libloaderapi-findresourcew

            let mut group_id: Vec<PCWSTR> = vec![];
            let _bool = EnumResourceNamesW(
                Some(hmodule),
                RT_GROUP_ICON,
                Some(enum_group_icon_callback),
                &mut group_id as *mut _ as isize,
            );
            println!("group_id:{:?}", group_id);
            for id in group_id.iter() {
                let bool = is_int_resource(*id);
                if !bool {
                    println!("group_id:{:?}", *id);
                    return;
                }
                let group_data = load_resource_data(hmodule, *id, RT_GROUP_ICON).unwrap();
                let header = IconDir {
                    reserved: u16::from_le_bytes([group_data[0], group_data[1]]),
                    r#type: u16::from_le_bytes([group_data[2], group_data[3]]),
                    count: u16::from_le_bytes([group_data[4], group_data[5]]),
                };
                println!("header:{:?}", header);

                let mut entries: Vec<IconDirEntry> = vec![];
                for i in 0..header.count {
                    let offset = 6 + i as usize * 14;
                    let entry = IconDirEntry {
                        width: group_data[offset],
                        height: group_data[offset + 1],
                        color_count: group_data[offset + 2],
                        reserved: group_data[offset + 3],
                        planes: u16::from_le_bytes([
                            group_data[offset + 4],
                            group_data[offset + 5],
                        ]),
                        bit_count: u16::from_le_bytes([
                            group_data[offset + 6],
                            group_data[offset + 7],
                        ]),
                        bytes_in_res: u32::from_le_bytes([
                            group_data[offset + 8],
                            group_data[offset + 9],
                            group_data[offset + 10],
                            group_data[offset + 11],
                        ]),
                        id: u16::from_le_bytes([group_data[offset + 12], group_data[offset + 13]]),
                    };
                    entries.push(entry);
                }

                // 读取所有 RT_ICON 并构建 ICO 文件
                let mut ico_data = vec![];
                ico_data.extend_from_slice(&group_data[0..6]); // ICONDIR
                for entry in entries.iter() {
                    let icon_bin =
                        load_resource_data(hmodule, makeintresource(entry.id), RT_ICON).unwrap();
                    let mut new_entry = entry.clone();
                    new_entry.id = 0; // 替换为 offset
                    new_entry.bytes_in_res = icon_bin.len() as u32;
                    let offset = 6 + (14 * entries.len()) + ico_data.len() as usize;
                    let mut entry_buf = vec![
                        new_entry.width,
                        new_entry.height,
                        new_entry.color_count,
                        new_entry.reserved,
                    ];
                    entry_buf.extend_from_slice(&new_entry.planes.to_le_bytes());
                    entry_buf.extend_from_slice(&new_entry.bit_count.to_le_bytes());
                    entry_buf.extend_from_slice(&new_entry.bytes_in_res.to_le_bytes());
                    entry_buf.extend_from_slice(&(offset as u32).to_le_bytes());
                    let mut ico: Vec<u8> = vec![0, 0, 1, 0, 1, 0];
                    ico.extend(entry_buf);
                    ico[18] = 22; // 修改偏移
                    ico.extend(icon_bin);
                    let p = format!("target\\{}-{}.ico", name, entry.id);
                    let _ = std::fs::write(p, &ico);
                }
            }
            let _ = FreeLibrary(hmodule);
        }

        CoUninitialize();
    };
}

fn load_resource_data(hmodule: HMODULE, res_id: PCWSTR, res_type: PCWSTR) -> Option<Vec<u8>> {
    unsafe {
        let hres_info = FindResourceW(Some(hmodule), res_id, res_type);
        let hres_data = LoadResource(Some(hmodule), hres_info);
        match hres_data {
            Ok(hres_data) => {
                let size = SizeofResource(Some(hmodule), hres_info) as usize;
                let ptr = LockResource(hres_data) as *const u8;
                Some(std::slice::from_raw_parts(ptr, size).to_vec())
            }
            Err(e) => {
                println!("{:?},res_id:{:?},res_type:{:?}", e, res_id, res_type);
                None
            }
        }
    }
}

fn makeintresource(id: u16) -> PCWSTR {
    PCWSTR(id as usize as *const u16)
}

fn is_int_resource(res: PCWSTR) -> bool {
    (res.0 as usize) >> 16 == 0
}

unsafe extern "system" fn enum_group_icon_callback(
    _hmodule: HMODULE,
    itype: PCWSTR,
    name: PCWSTR,
    lparam: isize,
) -> BOOL {
    if itype.0 as u16 == 14 {
        let id_ptr = lparam as *mut Vec<PCWSTR>;
        (*id_ptr).push(name);
    }
    return true.into();
}
