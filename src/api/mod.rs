pub mod dyndns;
pub mod update;

use signal_hook::{consts::TERM_SIGNALS, flag};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::config::CONFIG;

pub fn launch() -> std::io::Result<()> {
    match CONFIG.api {
        crate::config::API::Update => update::Update::new().run(),
        crate::config::API::DynDNS => dyndns::DynDNS::new().run(),
    }
}

pub trait API {
    fn new() -> Self;
    fn check_v4(&mut self);
    fn check_v6(&mut self);
    fn update(&mut self);
    fn run(&mut self) -> std::io::Result<()> {
        let term_now = Arc::new(AtomicBool::new(false));
        for sig in TERM_SIGNALS {
            flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
            flag::register(*sig, Arc::clone(&term_now))?;
        }

        while !term_now.load(Ordering::Relaxed) {
            if !CONFIG.no_ipv4 {
                self.check_v4();
            }
            if !CONFIG.no_ipv6 {
                self.check_v6();
            }
            self.update();
            std::thread::sleep(Duration::from_secs_f64(CONFIG.interval));
        }
        info!("gracefully shutting down");
        Ok(())
    }
}
