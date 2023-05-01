use std::fmt::Display;

use curl::easy::Easy;
use once_cell::unsync::Lazy;
use serde::Serialize;

pub static mut CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub struct Client {
    client: Easy,
    url: String,
    buf: Vec<u8>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: Easy::new(),
            url: String::new(),
            buf: vec![],
        }
    }

    pub fn get(&mut self, url: &str) -> &mut Self {
        self.client.get(true).unwrap();
        self.url = self.url.to_string() + url;
        self
    }

    pub fn basic_auth(&mut self, username: &str, password: &str) -> &mut Self {
        use curl::easy::Auth;
        self.client.username(username).unwrap();
        self.client.password(password).unwrap();
        self.client.http_auth(Auth::new().basic(true)).unwrap();
        self
    }
    pub fn query<T: Serialize + ?Sized>(&mut self, query: &T) -> &mut Self {
        let pairs = serde_urlencoded::to_string(query).unwrap();
        self.url = self.url.to_string() + "?" + &pairs;
        self
    }
    pub fn send(&mut self) -> Result<&mut Self, Error> {
        self.client.url(&self.url).unwrap();
        let result = {
            let mut transfer = self.client.transfer();
            transfer
                .write_function(|data| {
                    self.buf.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform()
        };
        match result {
            Ok(_) => Ok(self),
            Err(e) => Err(Error(e.to_string())),
        }
    }
    pub fn status(&mut self) -> StatusCode {
        StatusCode(self.client.response_code().unwrap())
    }
    pub fn text(&mut self) -> Result<String, Error> {
        let buf = self.buf.to_owned();
        self.buf = vec![];
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
