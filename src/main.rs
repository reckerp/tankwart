mod client;

use client::{FuelPrice, Tankerkoenig};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");

    let api_key = env::var("TANKERKOENIG_API_KEY").expect("TANKERKOENIG_API_KEY not set in .env");
    let station_ids = env::var("STATION_IDS")
        .expect("STATION_IDS not set in .env")
        .split(',')
        .map(|id| id.trim().to_string())
        .collect::<Vec<String>>();

    let client = Tankerkoenig::new(api_key)?;
    let response = client.get_prices(station_ids).await?;

    println!("Response: {:?}", response.ok);

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
    Ok(())
}
