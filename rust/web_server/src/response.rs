use std::{fmt::{Display, Formatter, Result as FmtResult}};
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode{
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}

#[derive(Debug)]
pub struct Response {
    status_code : StatusCode,
    body: Option<String>,

}

impl Response{
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response{status_code, body}
    }

    // TcpStream to impl Write to make this more test friendly
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()>{
        let body = match &self.body{
            Some(_body) => _body,
            None => "",
        };

        write!(stream, 
                "HTTP/1.1 {} {}\r\n\r\n{}", 
                self.status_code, 
                self.status_code.reason_phrase(),
                body)   
    }
}