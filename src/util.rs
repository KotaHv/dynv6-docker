use std::net::IpAddr;

use local_ip_address::list_afinet_netifas;

use crate::config::CONFIG;
use crate::Error;
use crate::CLIENT;

const IPV4_URL: &'static str = "https://api4.my-ip.io/ip";

pub fn ipv6() -> Option<IpAddr> {
    let ifas = list_afinet_netifas().unwrap();
    #[cfg(not(target_os = "macos"))]
    {
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
    }
    #[cfg(target_os = "macos")]
    {
        let mut ipv6_list = vec![];
        for (name, ip) in ifas.iter() {
            if name == &CONFIG.interface {
                if let IpAddr::V6(v6) = ip {
                    // ipv6 link-local // IpAddr is_unicast_link_local
                    if (v6.segments()[0] & 0xffc0) != 0xfe80 {
                        ipv6_list.push(ip);
                    }
                }
            }
        }
        if !ipv6_list.is_empty() {
            if ipv6_list.len() == 1 {
                return Some(*ipv6_list[0]);
            }
            return Some(*ipv6_list[ipv6_list.len() - 2]);
        }
    }
    None
}

unsafe fn fetch_ipv4() -> Result<String, Error> {
    CLIENT.get(IPV4_URL).send()?.text()
}

pub fn ipv4() -> Option<IpAddr> {
    unsafe {
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
}
