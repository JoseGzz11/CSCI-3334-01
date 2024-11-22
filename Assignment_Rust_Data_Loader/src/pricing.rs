use serde::Deserialize;
use std::fs::File;
use std::io::Write;

pub trait Pricing {
    fn fetch_price(&self) -> Result<f64, String>;
    fn save_to_file(&self, price: f64) -> Result<(), String>;
}

#[derive(Deserialize, Debug)]
pub struct Bitcoin {}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64, String> {
        let response = ureq::get("https://api.coindesk.com/v1/bpi/currentprice/BTC.json")
            .call()
            .map_err(|e| format!("Request failed: {}", e))?
            .into_string()
            .map_err(|e| format!("Failed to read response: {}", e))?;

        let parsed: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        parsed["bpi"]["USD"]["rate_float"]
            .as_f64()
            .ok_or_else(|| "Failed to extract Bitcoin price".to_string())
    }

    fn save_to_file(&self, price: f64) -> Result<(), String> {
        let mut file = File::create("bitcoin_price.txt").map_err(|e| e.to_string())?;
        writeln!(file, "Bitcoin Price: ${:.2}", price).map_err(|e| e.to_string())
    }
}

#[derive(Deserialize, Debug)]
pub struct Ethereum {}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64, String> {
        let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd")
            .call()
            .map_err(|e| format!("Request failed: {}", e))?
            .into_string()
            .map_err(|e| format!("Failed to read response: {}", e))?;

        let parsed: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        parsed["ethereum"]["usd"]
            .as_f64()
            .ok_or_else(|| "Failed to extract Ethereum price".to_string())
    }

    fn save_to_file(&self, price: f64) -> Result<(), String> {
        let mut file = File::create("ethereum_price.txt").map_err(|e| e.to_string())?;
        writeln!(file, "Ethereum Price: ${:.2}", price).map_err(|e| e.to_string())
    }
}

#[derive(Deserialize, Debug)]
pub struct SP500 {}


impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64, String> {

        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d";
        
        let response = ureq::get(url)
            .call()
            .map_err(|e| format!("Request failed: {}", e))?
            .into_string()
            .map_err(|e| format!("Failed to read response: {}", e))?;
        
        let parsed: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;
        
        let price = parsed["chart"]["result"][0]["meta"]["regularMarketPrice"]
            .as_f64()
            .ok_or_else(|| "Failed to extract SP500 price".to_string())?;
        
        Ok(price)
    }
    

    fn save_to_file(&self, price: f64) -> Result<(), String> {
        let mut file = File::create("sp500_price.txt").map_err(|e| e.to_string())?;
        writeln!(file, "S&P 500 Price: ${:.2}", price).map_err(|e| e.to_string())
    }
}
