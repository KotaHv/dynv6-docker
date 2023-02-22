use dotenvy::dotenv;

#[macro_use]
extern crate log;

mod api;
mod config;
mod util;

use config::CONFIG;

fn main() {
    launch_info();
    dotenv().ok();
    if CONFIG.no_ipv4 && CONFIG.no_ipv6 {
        panic!("no_ipv4 and no_ipv6 can't both be true !")
    }
    pretty_env_logger::formatted_timed_builder()
        .parse_filters(&CONFIG.log_level)
        .parse_write_style(&CONFIG.log_style)
        .init();
    debug!("{CONFIG:?}");
    api::update::launch_task();
}

fn launch_info() {
    println!();
    println!("=================== Starting Dynv6 DDNS ===================");
    println!();
}
