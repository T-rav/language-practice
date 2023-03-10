use std::{net::TcpListener, io::Read, io::Write};
use crate::{http::Request, response::{StatusCode, Response}};
use std::convert::TryFrom;


pub struct Server{
    addr: String,
}

impl Server {
    pub fn new(addr:String) -> Self{
        Server {
            addr
        }
    }

    pub fn run(self){
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
                                Ok(_request) => {
                                    dbg!(_request);
                                     Response::new(StatusCode::Ok, Some("<h1>It Works!!!</h1>".to_string()))
                                },
                                Err(e) => {
                                    println!("Failed to parse request: {}", e);
                                    Response::new(StatusCode::BadRequest,None)
                                },
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

