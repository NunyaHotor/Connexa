use crate::crypto::session::SecureSession;
use crate::connexa::EncryptedMessage;

use sha2::{Sha256, Digest};
use x25519_dalek::PublicKey;

pub struct Messenger {
    pub session: SecureSession,
}

impl Messenger {
    pub fn send_message(&mut self, plaintext: &[u8], aad: &[u8]) -> EncryptedMessage {
        let ciphertext = self.session.encrypt_message(plaintext, aad);
        // Sealed sender: hash the sender's public key
        let sender_pub = self.session.ratchet.dhs_private.public_key().as_bytes();
        let sender_blind = Sha256::digest(sender_pub).to_vec();

        EncryptedMessage {
            ciphertext: ciphertext.into(),
            sender_blind,
            dh_ratchet_pub: sender_pub.to_vec(),
            message_number: self.session.ratchet.message_number,
            timestamp: chrono::Utc::now().timestamp(),
            target_device_id: "".to_string(),
            ttl: 0,
        }
    }

    pub fn receive_message(&mut self, msg: &EncryptedMessage, aad: &[u8]) -> Option<Vec<u8>> {
        // If the sender's DH ratchet pubkey is new, perform a ratchet step
        if msg.dh_ratchet_pub != self.session.ratchet.dhr_public.as_bytes() {
                        if let Ok(new_pub) = PublicKey::from_slice(&msg.dh_ratchet_pub) {
                self.session.ratchet.ratchet(new_pub);
            }
        }
        self.session.decrypt_message(&msg.ciphertext, aad)
    }
}

/*
pub async fn send_message_to_all_devices(
    sender_session: &SecureSession,
    recipient_devices: &[Device],
    plaintext: &[u8],
    aad: &[u8],
    store: &MessageStore,
) {
    for device in recipient_devices {
        // Assume you have a way to get or create a session for each device
        let mut session = get_or_create_session_for_device(device).await;
        let ciphertext = session.encrypt_message(plaintext, aad);
        let msg = EncryptedMessage {
            ciphertext: ciphertext.into(),
            sender_blind: vec![], // sealed sender logic
            dh_ratchet_pub: session.ratchet.dhs_private.public_key().as_bytes().to_vec(),
            message_number: session.ratchet.message_number,
            timestamp: chrono::Utc::now().timestamp(),
            target_device_id: Some(device.id), // Add this field to your proto
        };
        store.store_message(&device.id.to_string(), msg).await;
    }
}
*/