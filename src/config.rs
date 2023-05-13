use once_cell::sync::Lazy;
use serde::Deserialize;
use std::net::IpAddr;
use std::{fmt::Display, fs};

use figment::{providers::Env, Figment};

pub const IPV4_FILE: &'static str = ".dynv6.addr4";
pub const IPV6_FILE: &'static str = ".dynv6.addr6";

const PREFIX: &'static str = "DYNV6_";

pub static CONFIG: Lazy<Config> = Lazy::new(|| init_config());

#[derive(Debug)]
pub enum LogStyle {
    Auto,
    Always,
    Never,
}

impl Default for LogStyle {
    fn default() -> Self {
        Self::Auto
    }
}

impl Display for LogStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LogStyle::Auto => "auto",
            LogStyle::Always => "always",
            LogStyle::Never => "never",
        };
        write!(f, "{}", s)
    }
}

impl<'de> Deserialize<'de> for LogStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        match s.as_str() {
            "auto" => Ok(LogStyle::Auto),
            "always" => Ok(LogStyle::Always),
            "never" => Ok(LogStyle::Never),
            _ => Err(serde::de::Error::unknown_field(
                &s,
                &["auto", "always", "never"],
            )),
        }
    }
}

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
#[serde(default)]
pub struct Log {
    pub level: String,
    pub style: LogStyle,
}

impl Default for Log {
    fn default() -> Self {
        Log {
            level: Log::level(),
            style: LogStyle::default(),
        }
    }
}

impl Log {
    fn level() -> String {
        "dynv6=info".to_string()
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
