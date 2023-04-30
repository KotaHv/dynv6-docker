use std::fmt::Display;
use std::process::Command;

use once_cell::sync::Lazy;
use serde::Serialize;

use crate::Error;

pub static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub struct Client;

impl Client {
    pub fn new() -> Self {
        Self
    }

    pub fn get(&self, url: &str) -> RequestBuilder {
        RequestBuilder::new(url)
    }
}

pub struct RequestBuilder {
    url: String,
    basic_auth: Option<String>,
}

impl RequestBuilder {
    fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            basic_auth: None,
        }
    }
    pub fn basic_auth(mut self, username: &str, password: &str) -> Self {
        let basic_auth = String::from(username) + ":" + password;
        self.basic_auth = Some(basic_auth);
        self
    }
    pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> Self {
        self.url += "?";
        self.url += &serde_urlencoded::to_string(query).unwrap();
        self
    }
    pub fn send(&self) -> Result<Response, Error> {
        let mut args = vec!["-w", "status: %{http_code}\nerr: %{errormsg}"];
        if let Some(basic_auth) = &self.basic_auth {
            args.push("-u");
            args.push(basic_auth)
        }
        args.push(&self.url);
        let mut curl = Command::new("/usr/bin/curl");
        let output = curl.args(args).output();
        match output {
            Ok(output) => {
                let stdout = String::from_utf8(output.stdout).unwrap();
                let stderr = String::from_utf8(output.stderr).unwrap();
                match output.status.success() {
                    true => Ok(Response::new(&stdout)),
                    false => Err(Error(stderr)),
                }
            }
            Err(e) => Err(Error(e.to_string())),
        }
    }
}

pub struct Response {
    status: u16,
    err: String,
    text: String,
}

impl Response {
    fn new(dst: &str) -> Self {
        let (text, dst) = dst.split_once("\nstatus: ").unwrap();
        let (status, err) = dst.split_once("\nerr: ").unwrap();
        Self {
            status: status.parse().unwrap(),
            err: err.to_string(),
            text: text.to_string(),
        }
    }
    pub fn status(&self) -> StatusCode {
        StatusCode(self.status, self.status.to_string())
    }
    pub fn text(self) -> Result<String, Error> {
        if self.err.is_empty() {
            Ok(self.text)
        } else {
            Err(Error(self.err))
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
