use std::convert::TryFrom;
use std::str::Utf8Error;
use std::error::Error;
use std::str;
use std::str::FromStr;
use splitty::*;
use std::fmt::{Result as FmtResult, Formatter, Display, Debug};

pub struct Request{
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    fn from_byte_array(bug: &[u8]) -> Result<Self, String> {
        unimplemented!()
    }
}

impl TryFrom<&[u8]> for Request{
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(value)?;
        let mut tokens = split_unquoted_char(request, ' ');
        
        let method = tokens.next().ok_or(ParseError::InvalidRequest)?;
        let mut path = tokens.next().ok_or(ParseError::InvalidRequest)?;
        let protocol = tokens.next().ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method : Method = method.parse()?;

        // todo : I now need to ensure there are three tokens and return an error if not

        unimplemented!()
    }
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