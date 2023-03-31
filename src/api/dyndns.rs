use serde::{Serialize, Serializer};
use std::fs;
use std::time::Duration;

use crate::config::{CONFIG, IPV4_FILE, IPV6_FILE};
use crate::util::{self, CLIENT};

const DYNV6_URL: &'static str = "https://dynv6.com/nic/update";
const DYNDNS_GOOD: &'static str = "good";

pub fn launch_task() {
    debug!("DynDNS API");
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
struct Dynv6Params {
    hostname: String,
    #[serde(serialize_with = "as_myip")]
    myip: Vec<String>,
}

fn as_myip<T, S>(myip: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<Vec<String>>,
    S: Serializer,
{
    serializer.serialize_str(myip.as_ref().join(",").as_str())
}

impl Dynv6Params {
    fn new() -> Self {
        Dynv6Params {
            hostname: CONFIG.hostname.clone(),
            myip: Vec::new(),
        }
    }
}

struct Dynv6 {
    v4: String,
    v6: String,
    new_v4: Option<String>,
    new_v6: Option<String>,
    params: Dynv6Params,
    username: String,
    password: String,
}

impl Dynv6 {
    fn new() -> Self {
        Dynv6 {
            v4: CONFIG.current_ip.v4.clone(),
            v6: CONFIG.current_ip.v6.clone(),
            new_v4: None,
            new_v6: None,
            params: Dynv6Params::new(),
            username: "none".to_string(),
            password: CONFIG.token.clone(),
        }
    }
    fn check_v4(&mut self) {
        debug!("check v4");
        if let Some(new_v4) = util::ipv4() {
            let new_v4 = new_v4.to_string();
            if new_v4 != self.v4 {
                info!("old ipv4: {}, current ipv4: {}", self.v4, new_v4);
                self.params.myip.push(new_v4.clone());
                self.new_v4 = Some(new_v4);
            }
        }
    }
    fn check_v6(&mut self) {
        debug!("check v6");
        if let Some(new_v6) = util::ipv6() {
            let new_v6 = new_v6.to_string();
            if new_v6 != self.v6 {
                info!("old ipv6: {}, current ipv6: {}", self.v6, new_v6);
                self.params.myip.push(new_v6.clone());
                self.new_v6 = Some(new_v6)
            }
        }
    }
    fn update(&mut self) {
        if self.params.myip.is_empty() {
            return;
        }
        info!("ipv4/ipv6 address changed, start update");
        match CLIENT
            .get(DYNV6_URL)
            .basic_auth(&self.username, Some(&self.password))
            .query(&self.params)
            .send()
        {
            Ok(res) => {
                let status = res.status();
                let text = match res.text() {
                    Ok(text) => text.trim().to_string(),
                    Err(err) => format!("{err:?}"),
                };
                if status.is_success() && text == DYNDNS_GOOD {
                    info!("{DYNDNS_GOOD}");
                    if let Some(v4) = &self.new_v4 {
                        fs::write(IPV4_FILE, &v4).ok();
                        self.v4 = v4.to_string();
                    }
                    if let Some(v6) = &self.new_v6 {
                        fs::write(IPV6_FILE, &v6).ok();
                        self.v6 = v6.to_string();
                    }
                } else {
                    error!("code: {status:?}, msg: {text}");
                }
            }
            Err(err) => error!("{err}"),
        }
        self.params.myip = Vec::new();
        self.new_v4 = None;
        self.new_v6 = None;
    }
}
