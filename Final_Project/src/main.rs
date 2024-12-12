mod config;
mod worker;
mod report;
mod error;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use config::Config;
use report::{WebsiteStatus, Report};
use worker::check_websites;

fn main() {
    let config = Config::new();
    let websites = config.websites.clone();

    let results: Arc<Mutex<Vec<WebsiteStatus>>> = Arc::new(Mutex::new(Vec::new()));

    let start_time = Instant::now();
    check_websites(websites, &config, results.clone());

    let elapsed = start_time.elapsed();
    println!("\nAll checks completed in {:.2?}\n", elapsed);

    // Print report
    let results = results.lock().unwrap();
    Report::print(results.clone());
}
