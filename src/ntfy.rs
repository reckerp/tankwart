use crate::config::FuelType;

const ENDPOINT: &str = "https://ntfy.sh";

pub struct Ntfy {
    endpoint: String,
    base_topic: String,
}

impl Ntfy {
    pub fn new(base_topic: String) -> Self {
        Ntfy {
            endpoint: ENDPOINT.to_string(),
            base_topic,
        }
    }

    pub async fn send(&self, message: String, fuel_type: FuelType) -> Result<(), reqwest::Error> {
        let topic = format!("{}_{}", self.base_topic, fuel_type.key());
        let url = format!("{}/{}", self.endpoint, topic);
        reqwest::Client::new()
            .post(url)
            .body(message.to_string())
            .send()
            .await?;
        Ok(())
    }
}
