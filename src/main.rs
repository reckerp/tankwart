mod ntfy;
mod tankerkoenig;
mod threshold;

use dotenvy::dotenv;
use ntfy::Ntfy;
use std::collections::HashMap;
use std::env;
use tankerkoenig::{FuelPrice, Tankerkoenig};
use threshold::Threshold;
use tokio::time::{Duration, interval};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");

    let api_key = env::var("TANKERKOENIG_API_KEY").expect("TANKERKOENIG_API_KEY not set in .env");
    let ntfy_topic = env::var("NTFY_TOPIC").expect("NTFY_TOPIC not set in .env");
    let station_ids = env::var("STATION_IDS")
        .expect("STATION_IDS not set in .env")
        .split(',')
        .map(|id| id.trim().to_string())
        .collect::<Vec<String>>();

    let threshold = Threshold::from_env();
    let mut interval = interval(Duration::from_secs(300)); // 5 minutes
    let client = Tankerkoenig::new(api_key)?;
    let ntfy = Ntfy::new(ntfy_topic);
    let mut last_prices: HashMap<String, HashMap<String, f64>> = HashMap::new();

    loop {
        interval.tick().await;
        let response = client.get_prices(&station_ids).await?;

        if !response.ok {
            println!("API request failed");
            continue;
        }

        for (station_id, station) in response.prices {
            if station.status != "open" {
                println!("Station is closed!");
                continue;
            }

            last_prices.entry(station_id.clone()).or_default();

            process_fuel(
                &ntfy,
                "Diesel",
                station.diesel,
                threshold.diesel,
                station_id.as_str(),
                "diesel",
                &mut last_prices,
            )
            .await?;
            process_fuel(
                &ntfy,
                "E5",
                station.e5,
                threshold.e5,
                station_id.as_str(),
                "e5",
                &mut last_prices,
            )
            .await?;
            process_fuel(
                &ntfy,
                "E10",
                station.e10,
                threshold.e10,
                station_id.as_str(),
                "e10",
                &mut last_prices,
            )
            .await?;
        }
    }
}

async fn process_fuel(
    ntfy: &Ntfy,
    fuel_name: &str,
    fuel_price: FuelPrice,
    limit: Option<f64>,
    station_id: &str,
    fuel_key: &str,
    last_prices: &mut HashMap<String, HashMap<String, f64>>,
) -> Result<(), Box<dyn std::error::Error>> {
    match fuel_price {
        FuelPrice::Available(price) => {
            println!("{}: {}", fuel_name, price);

            if let Some(limit_val) = limit {
                let should_notify = if let Some(last_price) =
                    last_prices.get(station_id).and_then(|m| m.get(fuel_key))
                {
                    price <= limit_val && price < *last_price
                } else {
                    price <= limit_val
                };

                if should_notify {
                    let msg = format!("{} price dropped! Current price: {:.3}", fuel_name, price);
                    ntfy.send(msg).await?;
                }
            }

            last_prices
                .entry(station_id.to_string())
                .or_default()
                .insert(fuel_key.to_string(), price);
        }
        FuelPrice::NotAvailable() => {
            println!("{} not available", fuel_name);
        }
    }

    Ok(())
}
