use pqcrypto_kyber::kyber512::{keypair, encapsulate, decapsulate};

pub struct Handshake;

impl Handshake {
    pub fn perform_handshake() -> (Vec<u8>, Vec<u8>) {
        let (public_key, private_key) = keypair();
        let (ciphertext, shared_secret) = encapsulate(&public_key);
        let decrypted_secret = decapsulate(&ciphertext, &private_key);
        (shared_secret.to_vec(), decrypted_secret.to_vec())
    }
}
