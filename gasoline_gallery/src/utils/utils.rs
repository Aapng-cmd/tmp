use rand::Rng;
use rand::distributions::Alphanumeric;
use sha2::{Sha256, Digest};
use hex;
use base64::{Engine, engine::general_purpose};

pub fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric) 
        .take(length)
        .map(char::from)
        .collect()
}

pub fn random_bytes(length: usize) -> Vec<u8> {
    (0..length).map(|_| rand::thread_rng().gen()).collect()
}

pub fn hash_string(s: String, salt: String) -> String {
    let mut hasher = Sha256::new();

    hasher.update(s.as_bytes());
    hasher.update(salt.as_bytes());

    let result = hasher.finalize();

    hex::encode(result)
}

pub fn encrypt_data(data: &mut Vec<u8>, key: Vec<u8>) {
    for (i, byte) in data.iter_mut().enumerate() {
        *byte ^= key[i % key.len()];
    }
}

pub fn decrypt_data(data: &mut Vec<u8>, key: Vec<u8>) {
    for (i, byte) in data.iter_mut().enumerate() {
        *byte ^= key[i % key.len()];
    }
}

pub fn vec_to_b64(data: Vec<u8>) -> String {
    general_purpose::STANDARD.encode(data)
}

pub fn b64_to_vec(b64: String) -> Vec<u8> {
    general_purpose::STANDARD.decode(b64).expect("failed to decode data")
}