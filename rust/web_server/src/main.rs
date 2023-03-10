use server::Server;
use website_handler::WebsiteHandler;
use std::{env, fmt::format};

mod http;
mod server;
mod query_string;
mod response;
mod website_handler;

fn main() {

    let default_path = format!("{}/www", env!("CARGO_MANIFEST_DIR"));
    let default_addr = format!("{}:8080", "127.0.0.1");
    let default_doc = format!("{}","index.html");

    let www_root = env::var("WWW_ROOT").unwrap_or(default_path);
    let addr = env::var("WWW_IP_PORT").unwrap_or(default_addr);
    let base_doc = env::var("WWW_DEFAULT_DOC").unwrap_or(default_doc);

    //--- Print Vars ---//
    println!("************ [ TGINX Configuration ] ************");
    println!("IP bound to [{}]", addr);
    println!("root directory bound to [{}]", www_root);
    println!("default doc for / bound to [{}]", base_doc);
    println!("*******************************************");

    let server = Server::new(addr);
    server.run(WebsiteHandler::new(www_root, base_doc));
}