use crate::crypto::session::SecureSession;
use crate::proto::message::EncryptedMessage;
use sha2::{Sha256, Digest};
use crate::crypto::key::PublicKey;

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