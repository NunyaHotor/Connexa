//! Double Ratchet algorithm primitives for Connexa.

use x25519_dalek::{EphemeralSecret, PublicKey};
use hkdf::Hkdf;
use sha2::Sha256;

/// Represents the state of a Double Ratchet session.
pub struct RatchetState {
    pub root_key: [u8; 32],
    pub send_chain_key: [u8; 32],
    pub recv_chain_key: [u8; 32],
    pub dhs_private: EphemeralSecret,
    pub dhr_public: PublicKey,
    pub message_number: u32,
}

impl RatchetState {
    /// Initialize a new Double Ratchet session from a shared secret and DH keys.
    pub fn new(shared_secret: [u8; 32], dhs_private: EphemeralSecret, dhr_public: PublicKey) -> Self {
        // Derive initial root and chain keys from the shared secret
        let hk = Hkdf::<Sha256>::new(None, &shared_secret);
        let mut root_key = [0u8; 32];
        let mut chain_key = [0u8; 32];
        hk.expand(b"root", &mut root_key).unwrap();
        hk.expand(b"chain", &mut chain_key).unwrap();

        Self {
            root_key,
            send_chain_key: chain_key,
            recv_chain_key: chain_key,
            dhs_private,
            dhr_public,
            message_number: 0,
        }
    }

    /// Perform a DH ratchet step (when a new public key is received).
    pub fn ratchet(&mut self, new_dhr_public: PublicKey) {
        let dh_output = (&self.dhs_private).diffie_hellman(&new_dhr_public);
        let hk = Hkdf::<Sha256>::new(Some(&self.root_key), dh_output.as_bytes());
        let mut new_root = [0u8; 32];
        let mut new_chain = [0u8; 32];
        hk.expand(b"root", &mut new_root).unwrap();
        hk.expand(b"chain", &mut new_chain).unwrap();

        self.root_key = new_root;
        self.recv_chain_key = new_chain;
        self.dhr_public = new_dhr_public;
        self.message_number = 0;
    }

    /// Derive the next message key from the sending chain.
    pub fn next_send_message_key(&mut self) -> [u8; 32] {
        let hk = Hkdf::<Sha256>::new(None, &self.send_chain_key);
        let mut msg_key = [0u8; 32];
        hk.expand(b"msg", &mut msg_key).unwrap();
        // Advance the chain key
        hk.expand(b"chain", &mut self.send_chain_key).unwrap();
        self.message_number += 1;
        msg_key
    }

    /// Derive the next message key from the receiving chain.
    pub fn next_recv_message_key(&mut self) -> [u8; 32] {
        let hk = Hkdf::<Sha256>::new(None, &self.recv_chain_key);
        let mut msg_key = [0u8; 32];
        hk.expand(b"msg", &mut msg_key).unwrap();
        // Advance the chain key
        hk.expand(b"chain", &mut self.recv_chain_key).unwrap();
        msg_key
    }
}

// Add basic tests for ratchet state
#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::OsRng;

    #[test]
    fn test_ratchet_state_init() {
        let shared_secret = [1u8; 32];
        let dhs = EphemeralSecret::new(&mut OsRng);
        let dhr = PublicKey::from(&EphemeralSecret::new(&mut OsRng));
        let state = RatchetState::new(shared_secret, dhs, dhr);
        assert_eq!(state.root_key.len(), 32);
    }

    #[test]
    fn test_send_and_recv_message_keys() {
        let shared_secret = [2u8; 32];
        let dhs = EphemeralSecret::new(&mut OsRng);
        let dhr = PublicKey::from(&EphemeralSecret::new(&mut OsRng));
        let mut state = RatchetState::new(shared_secret, dhs, dhr);

        let send_key1 = state.next_send_message_key();
        let send_key2 = state.next_send_message_key();
        assert_ne!(send_key1, send_key2);

        let recv_key1 = state.next_recv_message_key();
        let recv_key2 = state.next_recv_message_key();
        assert_ne!(recv_key1, recv_key2);
    }

    #[test]
    fn test_ratchet_step_changes_keys() {
        let shared_secret = [3u8; 32];
        let dhs = EphemeralSecret::new(&mut OsRng);
        let dhr1 = PublicKey::from(&EphemeralSecret::new(&mut OsRng));
        let dhr2 = PublicKey::from(&EphemeralSecret::new(&mut OsRng));
        let mut state = RatchetState::new(shared_secret, dhs, dhr1);

        let old_root = state.root_key;
        state.ratchet(dhr2);
        assert_ne!(old_root, state.root_key);
    }
}