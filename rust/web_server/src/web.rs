use std::{net::TcpListener, io::Read};
use crate::http::Request;
use std::convert::TryFrom;
use std::str::from_utf8;

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
                            //println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            
                            let parsedRequest = Request::try_from(&buffer[..]);
                            match parsedRequest{
                                Ok(request) => {
                                    unimplemented!()
                                },
                                Err(e) => {
                                    println!("Failed to parse request: {}", e);
                                },
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

