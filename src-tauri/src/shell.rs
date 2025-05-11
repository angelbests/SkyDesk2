use std::{ffi::OsStr, os::windows::ffi::OsStrExt, ptr::null_mut};
use windows::{
    core::{Interface, HSTRING, PCWSTR},
    Win32::{
        Foundation::SIZE,
        Graphics::{
            Gdi::{HBITMAP, HPALETTE},
            Imaging::{
                CLSID_WICImagingFactory, GUID_ContainerFormatPng, GUID_VendorMicrosoftBuiltIn,
                GUID_WICPixelFormat32bppBGRA, IWICBitmapFrameEncode, IWICImagingFactory,
                WICBitmapEncoderNoCache, WICBitmapUseAlpha,
            },
        },
        Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES,
        System::Com::{
            CoCreateInstance, CoInitializeEx, IStream, StructuredStorage::IPropertyBag2,
            CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, STGM_CREATE, STGM_SHARE_EXCLUSIVE,
            STGM_WRITE,
        },
        UI::Shell::{
            Common::{ITEMIDLIST, STRRET},
            IShellItem, IShellItemImageFactory, SHCreateItemFromParsingName,
            SHCreateStreamOnFileEx, SHGetFileInfoW, SHParseDisplayName, SHFILEINFOW,
            SHGFI_DISPLAYNAME, SIIGBF_ICONONLY,
        },
    },
};
use windows::{Win32::Foundation::*, Win32::System::Com::*, Win32::UI::Shell::*};
#[tauri::command]
pub fn get_lnk_png(path: &str, savepath: &str, width: i32, height: i32) {
    let path = path.to_string();
    let savepath = savepath.to_string();
    tauri::async_runtime::spawn(async move {
        let h = get_file_thumbnail(&path, width, height);
        save_bitmap_as_png(h, &savepath);
    });
}

fn get_file_thumbnail(path: &str, width: i32, height: i32) -> HBITMAP {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let item: IShellItem = SHCreateItemFromParsingName(&HSTRING::from(path), None).unwrap();

        let image_factory: IShellItemImageFactory = item.cast().unwrap();
        let size = SIZE {
            cx: width,
            cy: height,
        };

        let hbitmap = image_factory.GetImage(size, SIIGBF_ICONONLY).unwrap();
        hbitmap
    }
}

fn save_bitmap_as_png(hbitmap: HBITMAP, savepath: &str) {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let imaging_factory: IWICImagingFactory =
            CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER).unwrap();
        let wic_bitmap = imaging_factory
            .CreateBitmapFromHBITMAP(hbitmap, HPALETTE::default(), WICBitmapUseAlpha)
            .unwrap();
        let stream: IStream = SHCreateStreamOnFileEx(
            &HSTRING::from(savepath),
            STGM_CREATE.0 | STGM_WRITE.0 | STGM_SHARE_EXCLUSIVE.0,
            0,
            true,
            None,
        )
        .unwrap();
        let encoder: windows::Win32::Graphics::Imaging::IWICBitmapEncoder = imaging_factory
            .CreateEncoder(&GUID_ContainerFormatPng, &GUID_VendorMicrosoftBuiltIn)
            .unwrap();
        let _ = encoder.Initialize(&stream, WICBitmapEncoderNoCache);
        let mut frame: Option<IWICBitmapFrameEncode> = None;
        let mut props: Option<IPropertyBag2> = None;
        encoder.CreateNewFrame(&mut frame, &mut props).unwrap();
        let frame = frame.unwrap();
        let _ = frame.Initialize(None);
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        wic_bitmap.GetSize(&mut width, &mut height).unwrap();
        let _ = frame.SetSize(width, height);
        let mut pixel_format = GUID_WICPixelFormat32bppBGRA;
        let _ = frame.SetPixelFormat(&mut pixel_format as *mut _);
        let _ = frame.WriteSource(&wic_bitmap, null_mut());
        let _ = frame.Commit();
        let _ = encoder.Commit();
    }
}

#[tauri::command]
pub async fn get_localized_display_name(path: String) -> Option<String> {
    let wide_path: Vec<u16> = OsStr::new(&path).encode_wide().chain(Some(0)).collect();
    let mut file_info = SHFILEINFOW::default();

    unsafe {
        let result = SHGetFileInfoW(
            PCWSTR(wide_path.as_ptr()),
            FILE_FLAGS_AND_ATTRIBUTES(0),
            Some(&mut file_info),
            std::mem::size_of::<SHFILEINFOW>() as u32,
            SHGFI_DISPLAYNAME,
        );
        if result == 0 {
            return None;
        }
        // Convert display name (null-terminated UTF-16)
        let display_name = {
            let len = (0..)
                .take_while(|&i| file_info.szDisplayName[i] != 0)
                .count();
            String::from_utf16_lossy(&file_info.szDisplayName[..len])
        };
        Some(display_name)
    }
}

#[derive(Clone, serde::Serialize)]
pub struct ProgramInfo {
    r#type: String,
    name: String,
    icoPath: String,
    targetPath: String,
    lnkPath: String,
}

#[tauri::command]
pub async fn get_uwp(path: &str) -> Result<Vec<ProgramInfo>, String> {
    let mut arr: Vec<ProgramInfo> = vec![];
    unsafe {
        // 初始化 COM
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        // 获取 IShellFolder 接口的 desktop folder
        let desktop = SHGetDesktopFolder().unwrap();

        // 解析 shell:AppsFolder 的 PIDL
        let mut pidl_apps_folder: *mut ITEMIDLIST = std::ptr::null_mut();
        let _ = SHParseDisplayName(
            PCWSTR::from_raw(
                "shell:AppsFolder\0"
                    .encode_utf16()
                    .collect::<Vec<u16>>()
                    .as_ptr(),
            ),
            None,
            &mut pidl_apps_folder,
            0,
            Some(std::ptr::null_mut()),
        );

        // 绑定到 AppsFolder 的 IShellFolder
        let apps_folder: IShellFolder = desktop.BindToObject(pidl_apps_folder, None).unwrap();

        // 获取枚举器
        let mut enum_id_list: Option<IEnumIDList> = None;
        let _ = apps_folder.EnumObjects(
            HWND::default(),
            (SHCONTF_FOLDERS.0 | SHCONTF_NONFOLDERS.0)
                .try_into()
                .unwrap(),
            &mut enum_id_list,
        );

        let enum_id_list = enum_id_list.unwrap();

        // 枚举所有项
        loop {
            let mut pidl: [*mut ITEMIDLIST; 1] = [std::ptr::null_mut()];
            let mut fetched = 0;
            let hr = enum_id_list.Next(&mut pidl, Some(&mut fetched));

            if hr != S_OK || fetched == 0 {
                break;
            }

            // 获取显示名称
            let mut strret = STRRET {
                uType: 0,
                Anonymous: Default::default(),
            };
            let _ = apps_folder.GetDisplayNameOf(pidl[0], SHGDN_NORMAL, &mut strret);

            let mut name_buf = [0u16; 260];
            let _ = StrRetToBufW(
                &mut strret as *mut _,
                Some(pidl[0] as *const _),
                &mut name_buf,
            );
            let name = String::from_utf16_lossy(
                &name_buf[..name_buf.iter().position(|&c| c == 0).unwrap_or(0)],
            );
            ///////////////////////////////////////////////
            // 创建完整 PIDL（AppsFolder + relative PIDL）
            let full_pidl = ILCombine(Some(pidl_apps_folder), Some(pidl[0] as *const _));
            // 转换为 IShellItem
            let item: IShellItem = SHCreateItemFromIDList(full_pidl).unwrap();
            // 获取程序的启动路径
            let start_path = item.GetDisplayName(SIGDN_DESKTOPABSOLUTEPARSING);
            match start_path {
                Ok(start_path) => {
                    let start_path = start_path.to_string().unwrap();
                    if !start_path.contains("://")
                        && !start_path.starts_with("shell:")
                        && start_path.contains("!")
                    {
                        // 保存图标
                        let image_factory: IShellItemImageFactory = item.cast().unwrap();
                        let hbitmap = image_factory
                            .GetImage(SIZE { cx: 64, cy: 64 }, SIIGBF_ICONONLY)
                            .unwrap();
                        let path = format!("{:}\\{:}.png", path, name);
                        save_bitmap_as_png(hbitmap, &path);
                        let powershell_str = format!("explorer shell:AppsFolder\\{:}", start_path);
                        arr.push(ProgramInfo {
                            r#type: String::from("cmd"),
                            name: name,
                            icoPath: path,
                            targetPath: powershell_str.clone(),
                            lnkPath: powershell_str,
                        });
                    }
                }
                Err(e) => {
                    println!("{:?}", e)
                }
            }
            // 释放 PIDL
            CoTaskMemFree(Some(pidl[0] as *const _));
        }
        CoUninitialize();
    }
    Ok(arr)
}
