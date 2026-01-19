mod client;

use client::{FuelPrice, Tankerkoenig};
use dotenvy::dotenv;
use std::env;
use tokio::time::{Duration, interval};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");

    let api_key = env::var("TANKERKOENIG_API_KEY").expect("TANKERKOENIG_API_KEY not set in .env");
    let station_ids = env::var("STATION_IDS")
        .expect("STATION_IDS not set in .env")
        .split(',')
        .map(|id| id.trim().to_string())
        .collect::<Vec<String>>();

    let mut interval = interval(Duration::from_secs(300)); // 5 minutes
    let client = Tankerkoenig::new(api_key)?;

    loop {
        interval.tick().await;
        let response = client.get_prices(&station_ids).await?;

        if !response.ok {
            println!("API request failed");
            continue;
        }

        for (_, station) in response.prices {
            println!("Station: {}", station.status);
            match station.diesel {
                FuelPrice::Available(price) => println!("Diesel: {}", price),
                FuelPrice::NotAvailable() => println!("Diesel not available"),
            }

            match station.e5 {
                FuelPrice::Available(price) => println!("E5: {}", price),
                FuelPrice::NotAvailable() => println!("E5 not available"),
            }

            match station.e10 {
                FuelPrice::Available(price) => println!("E10: {}", price),
                FuelPrice::NotAvailable() => println!("E10 not available"),
            }
        }
    }
}
