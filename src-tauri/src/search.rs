use windows::{
    core::{w, GUID, PCWSTR},
    Win32::System::{
        Com::{
            CLSIDFromProgID, CoCreateInstance, IDispatch, CLSCTX_ALL, DISPATCH_METHOD,
            DISPATCH_PROPERTYGET, DISPPARAMS,
        },
        Ole::{OleInitialize, OleUninitialize},
        Variant::VARIANT,
    },
};

#[derive(Clone, serde::Serialize)]
pub struct SearchItem {
    kind: String,
    name: String,
    path: String,
    ext: String,
}

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
        let sql = format!("SELECT System.ItemName,System.ItemPathDisplay,System.Kind,System.FileExtension FROM SYSTEMINDEX WHERE System.ItemName like '%{}%'",str);
        println!("{:?}", sql);
        let sql = VARIANT::from(sql.as_str());
        let mut execute_args = [sql];
        let recordset = invoke_method_return_dispatch(&conn, execute_dispid, &mut execute_args);
        let mut vec: Vec<SearchItem> = vec![];
        // 遍历 Recordset
        loop {
            if get_property_bool(&recordset, w!("EOF")) {
                break;
            }

            let fields = get_property_dispatch(&recordset, w!("Fields"));
            let name = get_field_value(&fields, "System.ItemName");
            let path = get_field_value(&fields, "System.ItemPathDisplay");
            let ext = get_field_value(&fields, "System.FileExtension");
            let kind = get_field_value(&fields, "System.Kind");

            if name.is_empty() {
                break;
            }
            println!("{} - {} - {} - {}", name, path, kind, ext);
            let item = SearchItem {
                name,
                path,
                ext,
                kind,
            };
            vec.push(item);
            // println!("{}", kind);
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
    // if result.vt()
    //     == windows::Win32::System::Variant::VARENUM(
    //         windows::Win32::System::Variant::VT_DISPATCH.0 as u16,
    //     )
    // {
    //     result
    //         .Anonymous
    //         .Anonymous
    //         .Anonymous
    //         .pdispVal
    //         .as_ref()
    //         .unwrap()
    //         .clone()
    // } else {
    //     panic!("Expected VARIANT to contain an IDispatch pointer");
    // }
    result
        .Anonymous
        .Anonymous
        .Anonymous
        .pdispVal
        .as_ref()
        .unwrap()
        .clone()
}

unsafe fn get_property_bool(dispatch: &IDispatch, name: PCWSTR) -> bool {
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
    result.is_empty()
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
    // if result.vt()
    //     == windows::Win32::System::Variant::VARENUM(
    //         windows::Win32::System::Variant::VT_DISPATCH.0 as u16,
    //     )
    // {
    //     result
    //         .Anonymous
    //         .Anonymous
    //         .Anonymous
    //         .pdispVal
    //         .as_ref()
    //         .unwrap()
    //         .clone()
    // } else {
    //     panic!("Expected VARIANT to contain an IDispatch pointer");
    // }
    result
        .Anonymous
        .Anonymous
        .Anonymous
        .pdispVal
        .as_ref()
        .unwrap()
        .clone()
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
