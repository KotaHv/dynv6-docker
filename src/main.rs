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
    logger::init_logger();
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
