use std::time::Duration;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}

pub struct Report;

impl Report {
    pub fn print(results: Vec<WebsiteStatus>) {
        println!("{:<30} {:<10} {:<15} {:<20}", "URL", "Status", "Response Time", "Timestamp");
        println!("{}", "-".repeat(80));
        for result in results {
            println!(
                "{:<30} {:<10} {:<15} {:<20}",
                result.url,
                result
                    .status
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|e| e.to_string()),
                format!("{:.2?}", result.response_time),
                result.timestamp
            );
        }
    }
}
