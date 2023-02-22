use once_cell::sync::Lazy;
use reqwest::blocking::Client;
use std::net::IpAddr;

use local_ip_address::list_afinet_netifas;

use crate::config::CONFIG;

const IPV4_URL: &'static str = "https://api4.my-ip.io/ip";

pub static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub fn ipv6() -> Option<IpAddr> {
    let ifas = list_afinet_netifas().unwrap();
    for (name, ip) in ifas.iter() {
        if name == &CONFIG.interface {
            if let IpAddr::V6(v6) = ip {
                // ipv6 link-local // IpAddr is_unicast_link_local
                if (v6.segments()[0] & 0xffc0) != 0xfe80 {
                    return Some(*ip);
                }
            }
        }
    }
    None
}

fn fetch_ipv4() -> reqwest::Result<String> {
    let res = CLIENT.get(IPV4_URL).send()?;
    res.text()
}

pub fn ipv4() -> Option<IpAddr> {
    match fetch_ipv4() {
        Ok(ip_str) => {
            if let Ok(ip) = ip_str.parse::<IpAddr>() {
                return Some(ip);
            }
            error!("{ip_str}");
        }
        Err(err) => error!("{err}"),
    }
    None
}
