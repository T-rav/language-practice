pub struct Request{
    path: String,
    query_string: Option<String>,
    method: Method,
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
