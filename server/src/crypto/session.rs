//! Integration of X3DH and Double Ratchet for secure session setup.

use crate::crypto::x3dh;
use crate::crypto::double_ratchet::{RatchetState};
use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};
use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng as ChaChaOsRng, AeadCore},
    ChaCha20Poly1305, Key, Nonce,
};

/// Represents a secure session between two parties.
pub struct SecureSession {
    pub ratchet: RatchetState,
}

impl SecureSession {
    /// Initiates a session as Alice (the sender).
    pub fn initiate_as_alice(
        alice_identity_priv: &StaticSecret,
        bob_identity_pub: &PublicKey,
        bob_signed_prekey_pub: &PublicKey,
        bob_onetime_prekey_pub: Option<&PublicKey>,
    ) -> Self {
        // Generate Alice's ephemeral key
        let alice_ephemeral = EphemeralSecret::random_from_rng(rand_core::OsRng);
        // Compute shared secret using X3DH
        let shared_secret = x3dh::x3dh_agree(
            alice_identity_priv,
            &alice_ephemeral,
            bob_identity_pub,
            bob_signed_prekey_pub,
            bob_onetime_prekey_pub,
        );
        // Initialize Double Ratchet state
        let ratchet = RatchetState::new(
            shared_secret,
            alice_ephemeral,
            *bob_signed_prekey_pub,
        );
        Self { ratchet }
    }

    /// Initiates a session as Bob (the receiver).
    pub fn initiate_as_bob(
        shared_secret: [u8; 32],
        bob_ephemeral_priv: EphemeralSecret,
        alice_ephemeral_pub: PublicKey,
    ) -> Self {
        let ratchet = RatchetState::new(
            shared_secret,
            bob_ephemeral_priv,
            alice_ephemeral_pub,
        );
        Self { ratchet }
    }

    /// Encrypts a plaintext message using the next send message key from the ratchet.
    pub fn encrypt_message(&mut self, plaintext: &[u8], associated_data: &[u8]) -> Vec<u8> {
        let msg_key = self.ratchet.next_send_message_key();
        let cipher = ChaCha20Poly1305::new(Key::from_slice(&msg_key));
        let nonce = ChaCha20Poly1305::generate_nonce(&mut ChaChaOsRng); // 12-bytes; unique per message
        let mut ciphertext = cipher
            .encrypt(&nonce, chacha20poly1305::aead::Payload {
                msg: plaintext,
                aad: associated_data,
            })
            .expect("encryption failure!");
        // Prepend nonce to ciphertext for transport
        let mut result = nonce.to_vec();
        result.append(&mut ciphertext);
        result
    }

    /// Decrypts a ciphertext message using the next receive message key from the ratchet.
    pub fn decrypt_message(&mut self, ciphertext: &[u8], associated_data: &[u8]) -> Option<Vec<u8>> {
        if ciphertext.len() < 12 {
            return None;
        }
        let msg_key = self.ratchet.next_recv_message_key();
        let cipher = ChaCha20Poly1305::new(Key::from_slice(&msg_key));
        let (nonce_bytes, ct) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        cipher
            .decrypt(nonce, chacha20poly1305::aead::Payload {
                msg: ct,
                aad: associated_data,
            })
            .ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::x3dh::{generate_keypair};
    use x25519_dalek::EphemeralSecret;

    #[test]
    fn test_session_initiation() {
        // Generate Alice and Bob identity keys
        let (alice_id_priv, alice_id_pub) = generate_keypair();
        let (bob_id_priv, bob_id_pub) = generate_keypair();
        let (bob_signed_priv, bob_signed_pub) = generate_keypair();

        // Alice initiates session
        let session = SecureSession::initiate_as_alice(
            &alice_id_priv,
            &bob_id_pub,
            &bob_signed_pub,
            None,
        );
        assert_eq!(session.ratchet.root_key.len(), 32);

        // Bob computes shared secret using X3DH (simulate)
        let alice_ephemeral = EphemeralSecret::random_from_rng(rand_core::OsRng);
        let shared_secret = crate::crypto::x3dh::x3dh_agree(
            &alice_id_priv,
            &alice_ephemeral,
            &bob_id_pub,
            &bob_signed_pub,
            None,
        );
        let bob_session = SecureSession::initiate_as_bob(
            shared_secret,
            EphemeralSecret::random_from_rng(rand_core::OsRng),
            PublicKey::from(&alice_ephemeral),
        );
        assert_eq!(bob_session.ratchet.root_key.len(), 32);
    }
}