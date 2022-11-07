use dotenvy::dotenv;

#[macro_use]
extern crate log;

mod config;
mod dynv6;
mod util;

use config::CONFIG;

fn main() {
    dotenv().ok();
    if CONFIG.no_ipv4 && CONFIG.no_ipv6 {
        panic!("no_ipv4 and no_ipv6 can't both be true !")
    }
    pretty_env_logger::formatted_timed_builder()
        .parse_filters(&CONFIG.log_level)
        .parse_write_style(&CONFIG.log_style)
        .init();
    debug!("{CONFIG:?}");
    dynv6::launch_task();
}
