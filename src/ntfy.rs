const ENDPOINT: &str = "https://ntfy.sh";

pub struct Ntfy {
    endpoint: String,
    topic: String,
}

impl Ntfy {
    pub fn new(topic: String) -> Self {
        Ntfy {
            endpoint: ENDPOINT.to_string(),
            topic: topic,
        }
    }

    pub async fn send(&self, message: String) -> Result<(), reqwest::Error> {
        let url = format!("{}/{}", self.endpoint, self.topic);
        reqwest::Client::new()
            .post(url)
            .body(message.to_string())
            .send()
            .await?;
        Ok(())
    }
}
