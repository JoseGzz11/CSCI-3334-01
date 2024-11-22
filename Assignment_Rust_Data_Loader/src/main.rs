mod pricing;

use pricing::{Bitcoin, Ethereum, SP500, Pricing};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {

    let bitcoin = Bitcoin {};
    let ethereum = Ethereum {};
    let sp500 = SP500 {};

    let assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(bitcoin),
        Box::new(ethereum),
        Box::new(sp500),
    ];


    loop {
        for asset in &assets {
            match asset.fetch_price() {
                Ok(price) => {
                    println!("Fetched price: ${:.2}", price);
                    if let Err(e) = asset.save_to_file(price) {
                        eprintln!("Error saving data: {}", e);
                    }
                }
                Err(e) => eprintln!("Error fetching data: {}", e),
            }
        }
        sleep(Duration::from_secs(10)).await;
    }
}
