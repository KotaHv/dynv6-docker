use std::{cell::RefCell, fmt::Display};

use curl::easy::Easy;
use once_cell::unsync::Lazy;
use serde::Serialize;

pub const CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub struct Client {
    inner: RefCell<Easy>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            inner: RefCell::new(Easy::new()),
        }
    }

    pub fn get(&self, url: &str) -> Request {
        self.inner.borrow_mut().get(true).unwrap();

        Request::new(self, url.into())
    }
}

pub struct Request<'a> {
    client: &'a Client,
    url: String,
}

impl<'a> Request<'a> {
    fn new(client: &'a Client, url: String) -> Self {
        Self { client, url }
    }
    pub fn basic_auth(self, username: &str, password: &str) -> Self {
        use curl::easy::Auth;
        {
            let mut client = self.client.inner.borrow_mut();
            client.username(username).unwrap();
            client.password(password).unwrap();
            client.http_auth(Auth::new().basic(true)).unwrap();
        }
        self
    }
    pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> Self {
        let pairs = serde_urlencoded::to_string(query).unwrap();
        self.url = self.url.to_string() + "?" + &pairs;
        self
    }
    pub fn send(&self) -> Result<Response, Error> {
        let mut client = self.client.inner.borrow_mut();
        client.url(&self.url).unwrap();
        let mut buf = Vec::new();
        let result = {
            let mut transfer = client.transfer();
            transfer
                .write_function(|data| {
                    buf.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform()
        };
        match result {
            Ok(_) => Ok(Response::new(self.client, buf)),
            Err(e) => Err(Error(e.to_string())),
        }
    }
}

pub struct Response<'a> {
    client: &'a Client,
    buf: Vec<u8>,
}

impl<'a> Response<'a> {
    fn new(client: &'a Client, buf: Vec<u8>) -> Self {
        Self { client, buf }
    }
    pub fn status(&self) -> StatusCode {
        StatusCode(self.client.inner.borrow_mut().response_code().unwrap())
    }
    pub fn text(&self) -> Result<String, Error> {
        let buf = self.buf.to_owned();
        match String::from_utf8(buf) {
            Ok(text) => Ok(text),
            Err(e) => Err(Error(e.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct StatusCode(u32);

impl StatusCode {
    pub fn is_success(&self) -> bool {
        300 > self.0 && self.0 >= 200
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
