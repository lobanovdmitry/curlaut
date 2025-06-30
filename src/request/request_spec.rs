use crate::auth::HttpAuthorization;
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Duration;
use url::Url;

pub struct HttpRequestSpec<'a> {
    pub url: Url,
    pub method: HttpRequestMethod,
    pub headers: HttpRequestHeaders<'a>,
    pub body: HttpRequestBody<'a>,
    pub authorization: Box<dyn HttpAuthorization>,
    pub http1: bool,
    pub timeout: Duration,
    pub insecure: bool,
}

#[derive(Debug)]
pub enum HttpRequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl Display for HttpRequestMethod {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

pub struct HttpRequestHeaders<'a>(pub HashMap<&'a str, &'a str>);

pub enum HttpRequestBody<'a> {
    Empty,
    Json(&'a str),
}
