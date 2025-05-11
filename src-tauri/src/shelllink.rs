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
            IShellItem, IShellItemImageFactory, SHCreateItemFromParsingName,
            SHCreateStreamOnFileEx, SHGetFileInfoW, SHFILEINFOW, SHGFI_DISPLAYNAME,
            SIIGBF_ICONONLY,
        },
    },
};
#[tauri::command]
pub fn get_lnk_png(path: &str, savepath: &str, width: i32, height: i32) {
    println!("path:{:?}", path);
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
        println!("{:?}", display_name);
        Some(display_name)
    }
}
