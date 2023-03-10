use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod http;
mod server;
mod query_string;
mod response;
mod website_handler;

fn main() {

    //let addr = String::from("127.0.0.1:8080");

    let default_path = format!("{}/www", env!("CARGO_MANIFEST_DIR"));
    let default_addr = format!("{}:8080", "127.0.0.1");

    let www_root = env::var("WWW_ROOT").unwrap_or(default_path);
    let addr = env::var("WWW_IP_PORT").unwrap_or(default_addr);

    //--- Print Vars ---//
    println!("************ [ Configuration ] ************");
    println!("WWW IP bound to [{}]", addr);
    println!("WWW root directory bound to [{}]", www_root);
    println!("*******************************************");

    let server = Server::new(addr);
    server.run(WebsiteHandler::new(www_root));
}