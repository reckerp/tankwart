use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;
use url::Url;

const ENDPOINT: &str = "https://creativecommons.tankerkoenig.de/json/";

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("URL parsing failed: {0}")]
    Url(#[from] url::ParseError),
    #[error("API returned error status: {0}")]
    Status(StatusCode),
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(untagged)]
pub enum FuelPrice {
    Available(f64),
    NotAvailable(),
}

#[derive(Debug, Deserialize)]
pub struct StationPrice {
    pub status: String,
    #[serde(default)]
    pub e5: Option<FuelPrice>,
    #[serde(default)]
    pub e10: Option<FuelPrice>,
    #[serde(default)]
    pub diesel: Option<FuelPrice>,
}

#[derive(Debug, Deserialize)]
pub struct PriceResponse {
    pub ok: bool,
    pub prices: HashMap<String, StationPrice>,
}

pub struct Tankerkoenig {
    client: Client,
    api_key: String,
    base_url: Url,
}

impl Tankerkoenig {
    pub fn new(api_key: String) -> Result<Self, ApiError> {
        Ok(Self {
            client: Client::new(),
            api_key,
            base_url: Url::parse(ENDPOINT)?,
        })
    }

    pub async fn get_prices(&self, ids: &[String]) -> Result<PriceResponse, ApiError> {
        let mut url = self.base_url.join("prices.php")?;
        url.query_pairs_mut()
            .append_pair("ids", &ids.join(","))
            .append_pair("apikey", &self.api_key);

        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            Ok(response.json::<PriceResponse>().await?)
        } else {
            Err(ApiError::Status(response.status()))
        }
    }
}
