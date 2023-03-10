use web::Server;
use website_handler::WebsiteHandler;

mod http;
mod web;
mod query_string;
mod response;
mod website_handler;

fn main() {

    let addr = String::from("127.0.0.1:8080");

    let server = Server::new(addr);
    server.run(WebsiteHandler);
}