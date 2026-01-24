use crate::config::{Config, FuelType};
use crate::ntfy::Ntfy;
use crate::tankerkoenig::Tankerkoenig;

pub struct PriceTracker {
    prices: std::collections::HashMap<String, std::collections::HashMap<String, f64>>,
}

impl PriceTracker {
    pub fn new() -> Self {
        Self {
            prices: std::collections::HashMap::new(),
        }
    }

    pub fn get_last_price(&self, station_id: &str, fuel_key: &str) -> Option<f64> {
        self.prices.get(station_id)?.get(fuel_key).copied()
    }

    pub fn update_price(&mut self, station_id: &str, fuel_key: &str, price: f64) {
        self.prices
            .entry(station_id.to_string())
            .or_default()
            .insert(fuel_key.to_string(), price);
    }
}

pub struct PriceMonitor {
    client: Tankerkoenig,
    ntfy: Ntfy,
    config: Config,
    price_tracker: PriceTracker,
}

impl PriceMonitor {
    pub fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            client: Tankerkoenig::new(config.api_key.clone())?,
            ntfy: Ntfy::new(config.ntfy_topic.clone()),
            config,
            price_tracker: PriceTracker::new(),
        })
    }

    pub async fn run(&mut self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300));

        loop {
            interval.tick().await;

            if let Err(e) = self.check_and_notify().await {
                eprintln!("Error checking prices: {e}");
            }
        }
    }

    async fn check_and_notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client.get_prices(&self.config.station_ids).await?;

        if !response.ok {
            return Ok(());
        }

        for (station_id, station) in response.prices {
            if station.status != "open" {
                println!("station {} closed", station_id);
                continue;
            }

            for fuel_type in FuelType::all() {
                self.process_fuel(&fuel_type, &station, &station_id).await?;
            }
        }

        Ok(())
    }

    async fn process_fuel(
        &mut self,
        fuel_type: &FuelType,
        station: &crate::tankerkoenig::StationPrice,
        station_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fuel_price = fuel_type.get_price(station);

        match fuel_price {
            Some(crate::tankerkoenig::FuelPrice::Available(price)) => {
                println!("{}: {}", fuel_type.name(), price);

                if let Some(limit_val) = fuel_type.threshold() {
                    let last_price = self.price_tracker.get_last_price(station_id, fuel_type.key());
                    let should_notify = match last_price {
                        Some(last) => price <= limit_val && price < last,
                        None => price <= limit_val,
                    };

                    if should_notify {
                        let msg = format!(
                            "{} price dropped! Current price: {:.3}",
                            fuel_type.name(),
                            price
                        );
                        self.ntfy.send(msg, *fuel_type).await?;
                    }
                }

                self.price_tracker
                    .update_price(station_id, fuel_type.key(), price);
            }
            Some(crate::tankerkoenig::FuelPrice::NotAvailable()) => {
                println!("{} not available", fuel_type.name());
            }
            None => {}
        }

        Ok(())
    }
}
