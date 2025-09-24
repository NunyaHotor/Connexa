use crate::connexa::EncryptedMessage;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::time::Duration;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Server {
    pub name: String,
    pub url: String,
}

#[derive(Clone)]
pub struct FederationState {
    pub servers: RwLock<Vec<Server>>,
}

impl FederationState {
    pub fn new() -> Self {
        Self {
            servers: RwLock::new(Vec::new()),
        }
    }
}

pub async fn send_federated_message(server: &Server, message: &EncryptedMessage) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;
    let url = format!("{}/federation/message", server.url.trim_end_matches('/'));
    client.post(url).json(message).send().await?;
    Ok(())
}