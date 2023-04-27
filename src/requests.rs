use std::fmt::Display;

use once_cell::sync::Lazy;
use serde::Serialize;

pub static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn get(&self, url: &str) -> RequestBuilder {
        RequestBuilder(self.client.get(url))
    }
}

pub struct RequestBuilder(reqwest::blocking::RequestBuilder);

impl RequestBuilder {
    pub fn basic_auth(self, username: &str, password: &str) -> RequestBuilder {
        RequestBuilder(self.0.basic_auth(username, Some(password)))
    }
    pub fn query<T: Serialize + ?Sized>(self, query: &T) -> Self {
        RequestBuilder(self.0.query(query))
    }
    pub fn send(self) -> Result<Response, Error> {
        match self.0.send() {
            Ok(res) => Ok(Response(res)),
            Err(e) => Err(Error(e.to_string())),
        }
    }
}

pub struct Response(reqwest::blocking::Response);

impl Response {
    pub fn status(&self) -> StatusCode {
        let status = self.0.status();
        StatusCode(status.as_u16(), status.to_string())
    }
    pub fn text(self) -> Result<String, Error> {
        match self.0.text() {
            Ok(text) => Ok(text),
            Err(e) => Err(Error(e.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct StatusCode(u16, String);

impl StatusCode {
    pub fn is_success(&self) -> bool {
        300 > self.0 && self.0 >= 200
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

#[derive(Debug)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
