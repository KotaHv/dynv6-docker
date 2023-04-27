use std::fmt::Display;

use once_cell::sync::Lazy;
use serde::Serialize;

pub static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub struct Client {
    client: ureq::Agent,
}

impl Client {
    #[cfg(not(feature = "native"))]
    pub fn new() -> Self {
        Self {
            client: ureq::Agent::new(),
        }
    }
    #[cfg(feature = "native")]
    pub fn new() -> Self {
        use std::sync::Arc;
        Self {
            client: ureq::AgentBuilder::new()
                .tls_connector(Arc::new(native_tls::TlsConnector::new().unwrap()))
                .build(),
        }
    }

    pub fn get(&self, url: &str) -> RequestBuilder {
        RequestBuilder(self.client.get(url))
    }
}

pub struct RequestBuilder(ureq::Request);

impl RequestBuilder {
    pub fn basic_auth(self, username: &str, password: &str) -> RequestBuilder {
        use base64::{engine::general_purpose, Engine as _};
        let basic_auth = String::from(username) + ":" + password;
        let basic_auth =
            String::from("Basic ") + &general_purpose::STANDARD.encode(basic_auth.as_bytes());
        RequestBuilder(self.0.set("Authorization", &basic_auth))
    }
    pub fn query<T: Serialize + ?Sized>(self, query: &T) -> Self {
        use std::collections::BTreeMap;
        let pairs = serde_json::to_value(query).unwrap();
        let pairs: BTreeMap<String, String> = serde_json::from_value(pairs).unwrap();
        let pairs = pairs
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect::<Vec<(&str, &str)>>();
        RequestBuilder(self.0.query_pairs(pairs))
    }
    pub fn send(self) -> Result<Response, Error> {
        match self.0.call() {
            Ok(res) => Ok(Response(res)),
            Err(e) => Err(Error(e.to_string())),
        }
    }
}

pub struct Response(ureq::Response);

impl Response {
    pub fn status(&self) -> StatusCode {
        StatusCode(self.0.status(), self.0.status_text().to_string())
    }
    pub fn text(self) -> Result<String, Error> {
        match self.0.into_string() {
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
