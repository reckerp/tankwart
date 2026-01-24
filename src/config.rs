use std::env;

pub struct Config {
    pub api_key: String,
    pub ntfy_topic: String,
    pub station_ids: Vec<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            api_key: get_string("TANKERKOENIG_API_KEY"),
            ntfy_topic: get_string("NTFY_TOPIC"),
            station_ids: get_string_vec("STATION_IDS"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum FuelType {
    E5,
    E10,
    Diesel,
}

impl FuelType {
    pub fn name(&self) -> &str {
        match self {
            FuelType::E5 => "E5",
            FuelType::E10 => "E10",
            FuelType::Diesel => "Diesel",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            FuelType::E5 => "e5",
            FuelType::E10 => "e10",
            FuelType::Diesel => "diesel",
        }
    }

    pub fn threshold(&self) -> Option<f64> {
        match self {
            FuelType::E5 => get_f64("THRESHOLD_E5"),
            FuelType::E10 => get_f64("THRESHOLD_E10"),
            FuelType::Diesel => get_f64("THRESHOLD_DIESEL"),
        }
    }

    pub fn all() -> [FuelType; 3] {
        [FuelType::E5, FuelType::E10, FuelType::Diesel]
    }

    pub fn get_price(
        &self,
        station: &crate::tankerkoenig::StationPrice,
    ) -> Option<crate::tankerkoenig::FuelPrice> {
        match self {
            FuelType::E5 => station.e5,
            FuelType::E10 => station.e10,
            FuelType::Diesel => station.diesel,
        }
    }
}

fn get_f64(key: &str) -> Option<f64> {
    env::var(key).ok()?.parse().ok()
}

fn get_string(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Missing environment variable: {}", key))
}

fn get_string_vec(key: &str) -> Vec<String> {
    let value = env::var(key).unwrap_or_else(|_| panic!("Missing environment variable: {}", key));

    value
        .split(',')
        .map(|id| id.trim())
        .filter(|id| !id.is_empty())
        .map(|id| id.to_string())
        .collect()
}
