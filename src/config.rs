use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs;
use std::net::IpAddr;

use figment::{providers::Env, Figment};

pub const IPV4_FILE: &'static str = ".dynv6.addr4";
pub const IPV6_FILE: &'static str = ".dynv6.addr6";

pub static CONFIG: Lazy<Config> = Lazy::new(|| init_config());

#[derive(Deserialize, Debug)]
pub struct CurrentIpAddr {
    pub v4: String,
    pub v6: String,
}

#[derive(Deserialize, Debug)]
pub enum API {
    Update,
    DynDNS,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub hostname: String,
    pub token: String,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_log_style")]
    pub log_style: String,
    #[serde(default)]
    pub no_ipv4: bool,
    #[serde(default)]
    pub no_ipv6: bool,
    #[serde(default = "default_interface")]
    pub interface: String,
    #[serde(default = "default_current_ip")]
    pub current_ip: CurrentIpAddr,
    #[serde(default = "default_interval")]
    pub interval: f64,
    #[serde(default = "default_api")]
    pub api: API,
}

fn default_interface() -> String {
    "eth0".to_string()
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_style() -> String {
    "auto".to_string()
}

fn default_current_ip() -> CurrentIpAddr {
    let mut v4 = "".to_string();
    if let Ok(v4_str) = fs::read_to_string(IPV4_FILE) {
        if v4_str.parse::<IpAddr>().is_ok() {
            v4 = v4_str;
        }
    }
    let mut v6 = "".to_string();
    if let Ok(v6_str) = fs::read_to_string(IPV6_FILE) {
        if v6_str.parse::<IpAddr>().is_ok() {
            v6 = v6_str;
        }
    }
    CurrentIpAddr { v4, v6 }
}

fn default_interval() -> f64 {
    10.0
}

fn default_api() -> API {
    API::DynDNS
}

pub fn init_config() -> Config {
    let config = Figment::from(Env::prefixed("DYNV6_")).extract::<Config>();
    match config {
        Ok(config) => {
            if config.no_ipv4 && config.no_ipv6 {
                panic!("no_ipv4 and no_ipv6 can't both be true !")
            }
            println!("{:#?}", config);
            config
        }
        Err(err) => {
            panic!("{}", err);
        }
    }
}
