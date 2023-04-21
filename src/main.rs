use dotenvy::dotenv;

#[macro_use]
extern crate log;

mod api;
mod config;
mod logger;
mod util;

fn main() -> std::io::Result<()> {
    launch_info();
    dotenv().ok();
    logger::init_logger();
    api::launch()
}

fn launch_info() {
    println!();
    println!("=================== Starting Dynv6 DDNS ===================");
    println!();
}
