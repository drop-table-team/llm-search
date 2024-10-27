use anyhow::bail;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
struct RegisterPayload {
    name: String,
}

#[derive(Clone, Deserialize)]
pub struct RegisterResponse {
    #[serde(rename(deserialize = "mongoAddress"))]
    pub mongo_address: String,
    #[serde(rename(deserialize = "mongoDatabase"))]
    pub mongo_database: String,
    #[serde(rename(deserialize = "mongoCollection"))]
    pub mongo_collection: String,
    #[serde(rename(deserialize = "qdrantAddress"))]
    pub qdrant_address: String,
    #[serde(rename(deserialize = "qdrantCollection"))]
    pub qdrant_collection: String,
}

#[derive(Clone, Debug, Serialize)]
struct UnregisterPayload {}

pub struct Module {
    name: String,
    client: Client,
}

impl Module {
    pub fn new(name: String) -> Self {
        Module {
            name,
            client: Client::new(),
        }
    }

    pub async fn register<A: AsRef<str>>(&self, address: A) -> anyhow::Result<RegisterResponse> {
        let url = format!("{}/modules/output/register", address.as_ref().to_string());

        let payload = RegisterPayload {
            name: self.name.clone(),
        };

        let response = self
            .client
            .post(&url)
            .body(serde_json::to_string(&payload).unwrap())
            .send()
            .await?;

        if response.status() != 200 {
            bail!(
                "Couldn't register module '{}' on backend '{}', got status {}: {}",
                self.name,
                address.as_ref(),
                response.status(),
                String::from_utf8_lossy(&response.bytes().await?)
            );
        }

        Ok(response.json::<RegisterResponse>().await?)
    }

    pub async fn unregister<A: AsRef<str>>(&mut self, address: A) -> anyhow::Result<()> {
        let url = format!("{}/modules/output/unregister", address.as_ref().to_string());

        let payload = UnregisterPayload {};

        self.client
            .post(&url)
            .body(serde_json::to_string(&payload).unwrap())
            .send()
            .await?;

        Ok(())
    }
}
