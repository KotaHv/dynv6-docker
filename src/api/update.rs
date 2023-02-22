use serde::Serialize;
use std::fs;
use std::time::Duration;

use crate::config::{CONFIG, IPV4_FILE, IPV6_FILE};
use crate::util::{self, CLIENT};

const DYNV6_URL: &'static str = "https://dynv6.com/api/update";

pub fn launch_task() {
    debug!("update API");
    let mut dynv6 = Dynv6::new();
    loop {
        if !CONFIG.no_ipv4 {
            dynv6.check_v4();
        }
        if !CONFIG.no_ipv6 {
            dynv6.check_v6();
        }
        dynv6.update();
        std::thread::sleep(Duration::from_secs_f64(CONFIG.interval));
    }
}

#[derive(Serialize)]
pub struct Dynv6Params {
    hostname: String,
    token: String,
    #[serde(rename = "ipv4")]
    v4: Option<String>,
    #[serde(rename = "ipv6")]
    v6: Option<String>,
}

impl Dynv6Params {
    fn new() -> Self {
        Dynv6Params {
            hostname: CONFIG.hostname.clone(),
            token: CONFIG.token.clone(),
            v4: None,
            v6: None,
        }
    }
}

pub struct Dynv6 {
    v4: String,
    v6: String,
    params: Dynv6Params,
}

impl Dynv6 {
    fn new() -> Self {
        Dynv6 {
            v4: CONFIG.current_ip.v4.clone(),
            v6: CONFIG.current_ip.v6.clone(),
            params: Dynv6Params::new(),
        }
    }
    fn check_v4(&mut self) {
        debug!("check v4");
        if let Some(new_v4) = util::ipv4() {
            let new_v4 = new_v4.to_string();
            if new_v4 != self.v4 {
                info!("old ipv4: {}, current ipv4: {}", self.v4, new_v4);
                self.params.v4 = Some(new_v4);
            }
        }
    }
    fn check_v6(&mut self) {
        debug!("check v6");
        if let Some(new_v6) = util::ipv6() {
            let new_v6 = new_v6.to_string();
            if new_v6 != self.v6 {
                info!("old ipv6: {}, current ipv6: {}", self.v6, new_v6);
                self.params.v6 = Some(new_v6);
            }
        }
    }
    fn update(&mut self) {
        if self.params.v4.is_none() && self.params.v6.is_none() {
            return;
        }
        info!("ipv4/ipv6 address changed, start update");
        match CLIENT.get(DYNV6_URL).query(&self.params).send() {
            Ok(res) => {
                if res.status().is_success() {
                    info!("{:?}", res.text());
                    if let Some(v4) = &self.params.v4 {
                        fs::write(IPV4_FILE, v4).ok();
                        self.v4 = v4.to_owned();
                    }
                    if let Some(v6) = &self.params.v6 {
                        fs::write(IPV6_FILE, v6).ok();
                        self.v6 = v6.to_owned();
                    }
                } else {
                    error!("code: {:?}, msg: {:?}", res.status(), res.text());
                }
            }
            Err(err) => error!("{err}"),
        }
        self.params.v4 = None;
        self.params.v6 = None;
    }
}
