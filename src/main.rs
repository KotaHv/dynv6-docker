use dotenvy::dotenv;

#[macro_use]
extern crate log;

mod api;
mod config;
mod error;
mod logger;
mod requests;
mod util;

pub use error::Error;
pub use requests::CLIENT;

fn main() -> std::io::Result<()> {
    launch_info();
    dotenv().ok();
    logger::init_logger();
    api::launch()
}

fn launch_info() {
    println!();
    println!(
        "=================== Starting Dynv6 DDNS {} ===================",
        env!("CARGO_PKG_VERSION")
    );
    println!();
}
