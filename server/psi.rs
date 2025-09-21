use x25519_dalek::{
    EphemeralSecret,
    PublicKey,
};
use sha2::{Sha256, Digest};
use rand_core::OsRng;

pub fn generate_server_oprf_key() -> EphemeralSecret {
    EphemeralSecret::new(OsRng)
}

pub fn blind(input: &[u8]) -> (EphemeralSecret, PublicKey) {
    let client_secret = EphemeralSecret::new(OsRng);
    let client_public = PublicKey::from(&client_secret);
    (client_secret, client_public)
}

pub fn evaluate(server_secret: &EphemeralSecret, blinded_element: &PublicKey) -> PublicKey {
    (server_secret * blinded_element).to_bytes().into()
}

pub fn unblind(client_secret: &EphemeralSecret, evaluated_element: &PublicKey) -> PublicKey {
    (client_secret * evaluated_element).to_bytes().into()
}