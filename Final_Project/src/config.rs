use std::env;

pub struct Config {
    pub websites: Vec<String>,
    pub timeout: u64,
    pub max_retries: usize,
    pub worker_threads: usize,
}

impl Config {
    pub fn new() -> Self {
        Self {
            websites: vec![
                "https://www.google.com".to_string(),
                "https://www.youtube.com".to_string(),
                "https://www.rust-lang.org".to_string(),
                "https://www.ubs.com".to_string(),
                "https://www.spotify.com".to_string(),

            ],
            timeout: 5,             // Default timeout in seconds
            max_retries: 2,         // Max retries per website
            worker_threads: 10,     // Number of worker threads
        }
    }
}
