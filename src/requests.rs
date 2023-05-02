use std::{cell::RefCell, fmt::Display, rc::Rc};

use curl::easy::Easy;
use once_cell::unsync::Lazy;
use serde::Serialize;

pub const CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

#[derive(Clone)]
pub struct Client {
    inner: Rc<RefCell<Easy>>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(Easy::new())),
        }
    }

    pub fn get(&self, url: &str) -> Request {
        let mut inner = self.inner.borrow_mut();
        inner.get(true).unwrap();
        Request::new(self.clone(), url.into())
    }
}

pub struct Request {
    client: Client,
    url: String,
}

impl Request {
    fn new(client: Client, url: String) -> Self {
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
    pub fn send(&mut self) -> Result<Response, Error> {
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
            Ok(_) => Ok(Response::new(self.client.clone(), buf)),
            Err(e) => Err(Error(e.to_string())),
        }
    }
}

pub struct Response {
    client: Client,
    buf: Vec<u8>,
}

impl Response {
    fn new(client: Client, buf: Vec<u8>) -> Self {
        Self { client, buf }
    }
    pub fn status(&self) -> StatusCode {
        let mut client = self.client.inner.borrow_mut();
        StatusCode(client.response_code().unwrap())
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
