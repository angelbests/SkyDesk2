use std::{fs::File, path::Path};
use tiny_http::{Response, Server};

#[tauri::command]
pub fn open_server(str: String) {
    tauri::async_runtime::spawn(async move {
        let server = Server::http("127.0.0.1:12345").unwrap();
        for request in server.incoming_requests() {
            println!(
                "received request! method: {:?}, url: {:?}",
                request.method(),
                request.url(),
            );
            let mut urlstr = request.url();
            if urlstr == "/" {
                urlstr = "/index.html"
            }
            // if urlstr == "/favicon.ico" {
            //     continue;
            // }
            let sstr = str.clone() + urlstr;
            println!("{:?}", sstr);
            let file = File::open(&Path::new(&sstr));
            match file {
                Ok(s) => {
                    let response = Response::from_file(s);
                    let _ = request.respond(response);
                }
                Err(_e) => {
                    let response = Response::empty(404);
                    let _ = request.respond(response);
                }
            }
        }
    });
}
