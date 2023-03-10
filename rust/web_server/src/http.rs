use std::convert::TryFrom;
use std::str::Utf8Error;
use std::error::Error;
use std::str;
use std::str::FromStr;
use splitty::*;
use std::fmt::{Result as FmtResult, Display, Debug};

use crate::query_string::{QueryString, self};
#[derive(Debug)]
pub struct Request<'buf>{
    path: String,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self)-> &str{
        &&self.path
    }

    pub fn method(&self)-> &Method{
        &&self.method
    }

    pub fn query_string(&self)-> Option<&QueryString>{
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>{
    type Error = ParseError;

    fn try_from(value: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(value)?;
        let mut tokens = split_unquoted_char(request, ' ');
        
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol.trim() != "HTTP/1.1" {
            print!("[{}]",protocol);
            return Err(ParseError::InvalidProtocol);
        }

        let method : Method = method.parse()?;
        let mut query_string = None;

        if let Some(idx) = path.find('?'){
            query_string = Some(QueryString::from(&path[idx+1..]));
            path = &path[..idx];
        }

        Ok(Self{
            path : path.to_string(), 
            query_string,
            method
            
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

impl From<Utf8Error> for ParseError{
    fn from(value: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError{
    fn message(&self) -> &str {
        match self{
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl Display for ParseError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError{

}

#[derive(Debug)]
pub enum Method{
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET) ,
            "HEAD" => Ok(Self::HEAD),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError)
        }
    }
    
}

pub struct MethodError;

impl From<MethodError> for ParseError{
    fn from(value: MethodError) -> Self {
        Self::InvalidMethod
    }
}