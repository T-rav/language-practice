use web::Server;

mod http;
mod web;

fn main() {

    let addr = String::from("127.0.0.1:8080");

    let server = Server::new(addr);
    server.run();
}