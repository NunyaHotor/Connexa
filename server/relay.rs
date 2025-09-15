use crate::proto::message::EncryptedMessage;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use thiserror::Error;

/// Custom error type for relay operations.
#[derive(Debug, Error)]
pub enum RelayError {
    #[error("Recipient not found")]
    RecipientNotFound,
    #[error("Internal error")]
    InternalError,
}

/// In-memory message store for demonstration.
/// In production, use a persistent database.
pub struct MessageStore {
    // recipient_id -> Vec<EncryptedMessage>
    store: Arc<Mutex<HashMap<String, Vec<EncryptedMessage>>>>,
}

impl MessageStore {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Store a message for a recipient (async).
    pub async fn store_message(
        &self,
        recipient_id: &str,
        msg: EncryptedMessage,
        authenticated: bool,
    ) -> Result<(), RelayError> {
        if !authenticated {
            return Err(RelayError::InternalError);
        }
        let mut store = self.store.lock().await;
        store.entry(recipient_id.to_string())
            .or_default()
            .push(msg);
        Ok(())
    }

    /// Retrieve and remove all messages for a recipient (async).
    pub async fn fetch_messages(
        &self,
        recipient_id: &str,
        authenticated: bool,
    ) -> Result<Vec<EncryptedMessage>, RelayError> {
        if !authenticated {
            return Err(RelayError::InternalError);
        }
        let mut store = self.store.lock().await;
        Ok(store.remove(recipient_id).unwrap_or_default())
    }
}

/// Example push notification trigger (stub, async).
pub async fn trigger_push_notification(recipient_id: &str) {
    // Integrate with FCM, APNs, etc.
    println!("Push notification triggered for recipient: {}", recipient_id);
}

/// Store message and trigger push notification (async, with authentication).
pub async fn store_message_for_recipient(
    store: &MessageStore,
    recipient_id: &str,
    msg: EncryptedMessage,
    authenticated: bool,
) -> Result<(), RelayError> {
    store.store_message(recipient_id, msg, authenticated).await?;
    if authenticated {
        trigger_push_notification(recipient_id).await;
    }
    Ok(())
}