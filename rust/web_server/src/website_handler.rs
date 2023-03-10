use crate::{web::Handler, http::Request, response::{Response, StatusCode}};


pub struct WebsiteHandler;

impl Handler for WebsiteHandler{
    fn handle_request(&mut self, request: &Request) -> Response{
        Response::new(StatusCode::Ok, Some("<h1>test!</h1>".to_string()))
    }
}