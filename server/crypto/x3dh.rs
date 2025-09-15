//! X3DH key exchange protocol primitives for Connexa.

use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};

/// Represents a user's long-term identity key pair.
pub struct IdentityKeyPair {
    pub private: StaticSecret,
    pub public: PublicKey,
}

/// Represents a signed prekey pair.
pub struct SignedPreKeyPair {
    pub private: StaticSecret,
    pub public: PublicKey,
}

/// Represents a one-time prekey pair.
pub struct OneTimePreKeyPair {
    pub private: StaticSecret,
    pub public: PublicKey,
}

/// Generates a new X25519 key pair.
pub fn generate_keypair() -> (StaticSecret, PublicKey) {
    let private = StaticSecret::new(&mut OsRng);
    let public = PublicKey::from(&private);
    (private, public)
}

/// Computes the shared secret using X3DH.
/// Returns the derived shared secret bytes.
pub fn x3dh_agree(
    alice_identity_priv: &StaticSecret,
    alice_ephemeral_priv: &EphemeralSecret,
    bob_identity_pub: &PublicKey,
    bob_signed_prekey_pub: &PublicKey,
    bob_onetime_prekey_pub: Option<&PublicKey>,
) -> [u8; 32] {
    // DH1: Alice's ephemeral key & Bob's signed prekey
    let dh1 = alice_ephemeral_priv.diffie_hellman(bob_signed_prekey_pub);
    // DH2: Alice's identity key & Bob's signed prekey
    let dh2 = alice_identity_priv.diffie_hellman(bob_signed_prekey_pub);
    // DH3: Alice's ephemeral key & Bob's identity key
    let dh3 = alice_ephemeral_priv.diffie_hellman(bob_identity_pub);
    // DH4: Alice's ephemeral key & Bob's one-time prekey (if present)
    let dh4 = if let Some(otk) = bob_onetime_prekey_pub {
        alice_ephemeral_priv.diffie_hellman(otk).to_bytes()
    } else {
        [0u8; 32]
    };

    // Concatenate DH outputs and hash to derive the shared secret
    let mut hasher = sha2::Sha256::new();
    hasher.update(dh1.to_bytes());
    hasher.update(dh2.to_bytes());
    hasher.update(dh3.to_bytes());
    hasher.update(dh4);
    let result = hasher.finalize();
    let mut shared_secret = [0u8; 32];
    shared_secret.copy_from_slice(&result[..32]);
    shared_secret
}

// Add tests for key generation and agreement
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let (privk, pubk) = generate_keypair();
        assert_eq!(pubk.as_bytes().len(), 32);
        assert_eq!(privk.to_bytes().len(), 32);
    }
}