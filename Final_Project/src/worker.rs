use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use ureq;

use crate::config::Config;
use crate::report::WebsiteStatus;
use crate::error::MonitorError;

pub fn check_websites(
    websites: Vec<String>,
    config: &Config,
    results: Arc<Mutex<Vec<WebsiteStatus>>>,
) {
    let chunk_size = websites.len() / config.worker_threads + 1;
    let chunks: Vec<_> = websites.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect();

    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| {
            let results = Arc::clone(&results);
            let timeout = config.timeout;
            let max_retries = config.max_retries;

            thread::spawn(move || {
                for website in chunk {
                    let mut retries = 0;

                    while retries <= max_retries {
                        let start = Instant::now();
                        let result = match ureq::get(&website)
                            .timeout(Duration::from_secs(timeout))
                            .call()
                        {
                            Ok(response) => Ok(WebsiteStatus {
                                url: website.clone(),
                                status: Ok(response.status()),
                                response_time: start.elapsed(),
                                timestamp: chrono::Utc::now(),
                            }),
                            Err(e) => Err(MonitorError::from(e)),
                        };

                        if result.is_ok() || retries == max_retries {
                            let mut results = results.lock().unwrap();
                            results.push(result.unwrap_or_else(|err| WebsiteStatus {
                                url: website.clone(),
                                status: Err(err.to_string()),
                                response_time: Duration::new(0, 0),
                                timestamp: chrono::Utc::now(),
                            }));
                            break;
                        }

                        retries += 1;
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
