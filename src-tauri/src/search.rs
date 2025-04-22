use windows::{
    core::{w, GUID, PCWSTR},
    Win32::System::{
        Com::{
            CLSIDFromProgID, CoCreateInstance, IDispatch, CLSCTX_ALL, DISPATCH_METHOD,
            DISPATCH_PROPERTYGET, DISPPARAMS,
        },
        Ole::{OleInitialize, OleUninitialize},
        Variant::{VariantToBoolean, VARIANT},
    },
};

#[derive(Clone, serde::Serialize)]
pub struct SearchItem {
    kind: String,
    name: String,
    path: String,
    ext: String,
    title: String,
    mime: String,
    itemtype: String,
    pathdisplay: String,
    namedisplay: String,
}
// https://learn.microsoft.com/zh-cn/windows/win32/properties/props
// https://learn.microsoft.com/zh-cn/windows/win32/search/windows-search
// https://learn.microsoft.com/zh-cn/windows/win32/search/-search-3x-wds-propertymappings?redirectedfrom=MSDN
#[tauri::command]
pub async fn search_query(str: String) -> Vec<SearchItem> {
    unsafe {
        // 初始化 OLE / COM
        let _ = OleInitialize(Some(std::ptr::null_mut()));

        // 创建 ADODB.Connection 对象
        let conn: IDispatch = CoCreateInstance(
            &CLSIDFromProgID(w!("ADODB.Connection")).unwrap(),
            None,
            CLSCTX_ALL,
        )
        .unwrap();

        // 调用 .Open() 方法
        let open_dispid = get_dispid(&conn, w!("Open"));
        let conn_str = VARIANT::from(
            r#"Provider=Search.CollatorDSO.1;EXTENDED PROPERTIES="Application=Windows""#,
        );
        let mut args = [
            VARIANT::default(), // Options
            VARIANT::default(), // UserID
            VARIANT::default(), // Password
            conn_str,
        ];
        invoke_method(&conn, open_dispid, &mut args);

        // 调用 .Execute() 执行查询
        let execute_dispid = get_dispid(&conn, w!("Execute"));
        let sql = format!("SELECT TOP 100 System.ItemNameDisplay,System.ItemPathDisplay,System.ItemTypeText,System.MIMEType,System.Title,System.ItemName,System.ItemUrl,System.KindText,System.FileExtension FROM SYSTEMINDEX WHERE System.ItemName like '%{}%' and System.Kind is not null AND NOT System.ItemPathDisplay LIKE '%Recycle.Bin%' ORDER BY System.DateModified DESC",str);
        println!("{:?}", sql);
        let sql = VARIANT::from(sql.as_str());
        let mut execute_args = [sql];
        let recordset = invoke_method_return_dispatch(&conn, execute_dispid, &mut execute_args);
        let mut vec: Vec<SearchItem> = vec![];
        // 遍历 Recordset
        loop {
            if get_property_bool(&recordset, w!("EOF")) == 1 {
                break;
            }
            let fields = get_property_dispatch(&recordset, w!("Fields"));
            let namedisplay = get_field_value(&fields, "System.ItemNameDisplay");
            let pathdisplay = get_field_value(&fields, "System.ItemPathDisplay");
            let itemtype = get_field_value(&fields, "System.ItemTypeText");
            let mime = get_field_value(&fields, "System.MIMEType");
            let title = get_field_value(&fields, "System.Title");
            let name = get_field_value(&fields, "System.ItemName");
            let path = get_field_value(&fields, "System.ItemUrl");
            let ext = get_field_value(&fields, "System.FileExtension");
            let kind = get_field_value(&fields, "System.KindText");
            println!("{} - {} - {} - {}", name, path, kind, ext);
            let item = SearchItem {
                name,
                path,
                ext,
                kind,
                title,
                mime,
                itemtype,
                pathdisplay,
                namedisplay,
            };
            vec.push(item);
            let move_next_dispid = get_dispid(&recordset, w!("MoveNext"));
            invoke_method(&recordset, move_next_dispid, &mut []);
        }

        OleUninitialize();
        return vec;
    }
}

unsafe fn get_dispid(conn: &IDispatch, name: PCWSTR) -> i32 {
    let mut dispid = 0;
    let _ = conn.GetIDsOfNames(
        &CLSIDFromProgID(w!("ADODB.Connection")).unwrap(),
        &name,
        1,
        0,
        &mut dispid,
    );
    dispid
}

unsafe fn invoke_method(dispatch: &IDispatch, dispid: i32, args: &mut [VARIANT]) {
    let mut disp_params = DISPPARAMS {
        rgvarg: args.as_mut_ptr(),
        rgdispidNamedArgs: std::ptr::null_mut(),
        cArgs: args.len() as u32,
        cNamedArgs: 0,
    };
    let _ = dispatch.Invoke(
        dispid,
        &GUID::default(),
        0,
        DISPATCH_METHOD,
        &mut disp_params,
        Some(std::ptr::null_mut()),
        Some(std::ptr::null_mut()),
        Some(std::ptr::null_mut()),
    );
}

unsafe fn invoke_method_return_dispatch(
    dispatch: &IDispatch,
    dispid: i32,
    args: &mut [VARIANT],
) -> IDispatch {
    let mut result = VARIANT::default();
    let mut disp_params = DISPPARAMS {
        rgvarg: args.as_mut_ptr(),
        rgdispidNamedArgs: std::ptr::null_mut(),
        cArgs: args.len() as u32,
        cNamedArgs: 0,
    };
    let _ = dispatch.Invoke(
        dispid,
        &GUID::default(),
        0,
        DISPATCH_METHOD,
        &mut disp_params,
        Some(&mut result),
        Some(std::ptr::null_mut()),
        Some(std::ptr::null_mut()),
    );
    if result.vt()
        == windows::Win32::System::Variant::VARENUM(
            windows::Win32::System::Variant::VT_DISPATCH.0 as u16,
        )
    {
        result
            .Anonymous
            .Anonymous
            .Anonymous
            .pdispVal
            .as_ref()
            .unwrap()
            .clone()
    } else {
        panic!("Expected VARIANT to contain an IDispatch pointer");
    }
}

unsafe fn get_property_bool(dispatch: &IDispatch, name: PCWSTR) -> i32 {
    let dispid = get_dispid(dispatch, name);
    let mut result = VARIANT::default();
    let mut disp_params = DISPPARAMS {
        rgvarg: std::ptr::null_mut(),
        rgdispidNamedArgs: std::ptr::null_mut(),
        cArgs: 0,
        cNamedArgs: 0,
    };
    let _ = dispatch.Invoke(
        dispid,
        &GUID::default(),
        0,
        DISPATCH_PROPERTYGET,
        &mut disp_params,
        Some(&mut result),
        Some(std::ptr::null_mut()),
        Some(std::ptr::null_mut()),
    );
    let bool = VariantToBoolean(&result).unwrap().0;
    bool
}

unsafe fn get_property_dispatch(dispatch: &IDispatch, name: PCWSTR) -> IDispatch {
    let dispid = get_dispid(dispatch, name);
    let mut result = VARIANT::default();
    let mut disp_params = DISPPARAMS {
        rgvarg: std::ptr::null_mut(),
        rgdispidNamedArgs: std::ptr::null_mut(),
        cArgs: 0,
        cNamedArgs: 0,
    };
    let _ = dispatch.Invoke(
        dispid,
        &GUID::default(),
        0,
        DISPATCH_PROPERTYGET,
        &mut disp_params,
        Some(&mut result),
        Some(std::ptr::null_mut()),
        Some(std::ptr::null_mut()),
    );
    if result.vt()
        == windows::Win32::System::Variant::VARENUM(
            windows::Win32::System::Variant::VT_DISPATCH.0 as u16,
        )
    {
        result
            .Anonymous
            .Anonymous
            .Anonymous
            .pdispVal
            .as_ref()
            .unwrap()
            .clone()
    } else {
        panic!("Expected VARIANT to contain an IDispatch pointer");
    }
}

unsafe fn get_field_value(fields: &IDispatch, name: &str) -> String {
    let item_dispid = get_dispid(fields, w!("Item"));
    let name_var = VARIANT::from(name);
    let field = invoke_method_return_dispatch(fields, item_dispid, &mut [name_var]);

    let value_dispid = get_dispid(&field, w!("Value"));
    let mut result = VARIANT::default();
    let mut disp_params = DISPPARAMS {
        rgvarg: std::ptr::null_mut(),
        rgdispidNamedArgs: std::ptr::null_mut(),
        cArgs: 0,
        cNamedArgs: 0,
    };
    let _ = field.Invoke(
        value_dispid,
        &GUID::default(),
        0,
        DISPATCH_PROPERTYGET,
        &mut disp_params,
        Some(&mut result),
        Some(std::ptr::null_mut()),
        Some(std::ptr::null_mut()),
    );

    result.to_string()
}

// VT_EMPTY = 0,
//   VT_NULL = 1,
//   VT_I2 = 2,
//   VT_I4 = 3,
//   VT_R4 = 4,
//   VT_R8 = 5,
//   VT_CY = 6,
//   VT_DATE = 7,
//   VT_BSTR = 8,
//   VT_DISPATCH = 9,
//   VT_ERROR = 10,
//   VT_BOOL = 11,
//   VT_VARIANT = 12,
//   VT_UNKNOWN = 13,
//   VT_DECIMAL = 14,
//   VT_I1 = 16,
//   VT_UI1 = 17,
//   VT_UI2 = 18,
//   VT_UI4 = 19,
//   VT_I8 = 20,
//   VT_UI8 = 21,
//   VT_INT = 22,
//   VT_UINT = 23,
//   VT_VOID = 24,
//   VT_HRESULT = 25,
//   VT_PTR = 26,
//   VT_SAFEARRAY = 27,
//   VT_CARRAY = 28,
//   VT_USERDEFINED = 29,
//   VT_LPSTR = 30,
//   VT_LPWSTR = 31,
//   VT_RECORD = 36,
//   VT_INT_PTR = 37,
//   VT_UINT_PTR = 38,
//   VT_FILETIME = 64,
//   VT_BLOB = 65,
//   VT_STREAM = 66,
//   VT_STORAGE = 67,
//   VT_STREAMED_OBJECT = 68,
//   VT_STORED_OBJECT = 69,
//   VT_BLOB_OBJECT = 70,
//   VT_CF = 71,
//   VT_CLSID = 72,
//   VT_VERSIONED_STREAM = 73,
//   VT_BSTR_BLOB = 0xfff,
//   VT_VECTOR = 0x1000,
//   VT_ARRAY = 0x2000,
//   VT_BYREF = 0x4000,
//   VT_RESERVED = 0x8000,
//   VT_ILLEGAL = 0xffff,
//   VT_ILLEGALMASKED = 0xfff,
//   VT_TYPEMASK = 0xfff
