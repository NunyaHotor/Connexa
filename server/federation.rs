use crate::connexa::EncryptedMessage;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Server {
    pub name: String,
    pub url: String,
}

#[derive(Clone)]
pub struct FederationState {
    pub servers: Vec<Server>,
}

impl FederationState {
    pub fn new() -> Self {
        Self {
            servers: Vec::new(),
        }
    }
}

pub async fn send_federated_message(server: &Server, message: &EncryptedMessage) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("{}/federation/message", server.url);
    client.post(&url).json(message).send().await?;
    Ok(())
}
