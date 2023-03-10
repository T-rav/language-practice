use std::fs;

use crate::{server::Handler, http::Request, response::{Response, StatusCode}};


pub struct WebsiteHandler{
    www_root: String
}

impl WebsiteHandler{
    pub fn new(www_root:String) -> Self{
        Self{www_root}
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.www_root, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.www_root) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler{
    fn handle_request(&mut self, request: &Request) -> Response{
        //Response::new(StatusCode::Ok, Some("<h1>test!</h1>".to_string()))
        match request.method(){
            crate::http::Method::GET => match request.path(){
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                _ => Response::new(StatusCode::NotFound, None)
            },

            _ => Response::new(StatusCode::NotFound, None)
            /*
            crate::http::Method::HEAD => todo!(),
            crate::http::Method::POST => todo!(),
            crate::http::Method::PUT => todo!(),
            crate::http::Method::DELETE => todo!(),
            crate::http::Method::CONNECT => todo!(),
            crate::http::Method::OPTIONS => todo!(),
            crate::http::Method::TRACE => todo!(),
            crate::http::Method::PATCH => todo!(),
             */
        }
    }
}