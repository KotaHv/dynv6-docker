use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use curl::easy::Easy;
use once_cell::sync::Lazy;
use serde::Serialize;

pub static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub struct Client {
    client: Arc<Mutex<Easy>>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Mutex::new(Easy::new())),
        }
    }

    pub fn get(&self, url: &str) -> Request {
        let mut client = self.client.lock().unwrap();
        client.get(true).unwrap();
        Request::new(self.client.clone(), url.into())
    }
}

pub struct Request {
    client: Arc<Mutex<Easy>>,
    url: String,
}

impl Request {
    fn new(client: Arc<Mutex<Easy>>, url: String) -> Self {
        Self { client, url }
    }
    pub fn basic_auth(self, username: &str, password: &str) -> Self {
        use curl::easy::Auth;
        {
            let mut client = self.client.lock().unwrap();
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
        let mut client = self.client.lock().unwrap();
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
    client: Arc<Mutex<Easy>>,
    buf: Vec<u8>,
}

impl Response {
    fn new(client: Arc<Mutex<Easy>>, buf: Vec<u8>) -> Self {
        Self { client, buf }
    }
    pub fn status(&self) -> StatusCode {
        let mut client = self.client.lock().unwrap();
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
