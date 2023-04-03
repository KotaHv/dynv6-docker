use dotenvy::dotenv;

#[macro_use]
extern crate log;

mod api;
mod config;
mod logger;
mod util;

use config::CONFIG;

fn main() {
    launch_info();
    dotenv().ok();
    if CONFIG.no_ipv4 && CONFIG.no_ipv6 {
        panic!("no_ipv4 and no_ipv6 can't both be true !")
    }
    logger::init_logger();
    debug!("{CONFIG:?}");
    match CONFIG.api {
        config::API::Update => api::update::launch_task(),
        config::API::DynDNS => api::dyndns::launch_task(),
    }
}

fn launch_info() {
    println!();
    println!("=================== Starting Dynv6 DDNS ===================");
    println!();
}
