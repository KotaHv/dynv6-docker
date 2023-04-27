use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs;
use std::net::IpAddr;

use figment::{providers::Env, Figment};

pub const IPV4_FILE: &'static str = ".dynv6.addr4";
pub const IPV6_FILE: &'static str = ".dynv6.addr6";

const PREFIX: &'static str = "DYNV6_";

pub static CONFIG: Lazy<Config> = Lazy::new(|| init_config());

#[derive(Deserialize, Debug)]
pub struct CurrentIpAddr {
    pub v4: String,
    pub v6: String,
}

impl Default for CurrentIpAddr {
    fn default() -> Self {
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
}

#[derive(Deserialize, Debug)]
pub enum API {
    Update,
    DynDNS,
}

impl Default for API {
    fn default() -> Self {
        API::DynDNS
    }
}

#[derive(Deserialize, Debug)]
pub struct Log {
    #[serde(default = "Log::level")]
    pub level: String,
    #[serde(default = "Log::style")]
    pub style: String,
}

impl Default for Log {
    fn default() -> Self {
        Log {
            level: Log::level(),
            style: Log::style(),
        }
    }
}

impl Log {
    fn level() -> String {
        "dynv6=info".to_string()
    }

    fn style() -> String {
        "always".to_string()
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub hostname: String,
    pub token: String,
    #[serde(default)]
    pub no_ipv4: bool,
    #[serde(default)]
    pub no_ipv6: bool,
    #[serde(default = "Config::interface")]
    pub interface: String,
    #[serde(default = "Config::interval")]
    pub interval: f64,
    #[serde(default)]
    pub api: API,
    #[serde(default)]
    pub current_ip: CurrentIpAddr,
    #[serde(default)]
    pub log: Log,
}

impl Config {
    fn interface() -> String {
        "eth0".to_string()
    }

    fn interval() -> f64 {
        10.0
    }
}

pub fn init_config() -> Config {
    let config = Figment::from(Env::prefixed(PREFIX))
        .merge(Env::prefixed(PREFIX).split("_"))
        .extract::<Config>();
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
