use std::{net::TcpListener, io::Read, io::Write, fmt::Error};
use crate::{http::{Request, ParseError}, response::{StatusCode, Response}};
use std::convert::TryFrom;


pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response{
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server{
    addr: String,
}

impl Server {
    pub fn new(addr:String) -> Self{
        Server {
            addr
        }
    }

    pub fn run(self, mut handler: impl Handler){
        println!("Running on : [{}] !", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop{

            match listener.accept(){
                Ok((mut stream, addr)) => {
                    // todo: log success
                    println!("Successful connection from {}", addr.ip());
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(size) => {
                            // todo: read until end of all data
                            let response = match Request::try_from(&buffer[..]){
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream){
                                println!("Failed to send response: {}", e);
                            }
                        },
                        Err(e) => {
                            println!("Failed to read from connection: {}", e)
                        }
                    }
                    
                },
                Err(e) => {
                    // todo: log error
                    println!("Failed to establish a connection: {}", e);
                }
            }

            let connection = listener.accept();
            
            if connection.is_err(){
                continue;
                // todo : log error
            }

            // todo : log success
            let (stream, addr) = connection.unwrap();
        }
    }
}

