use rand::{thread_rng, RngCore};

mod aes128gcm;
mod aes256gcm;

use aes128gcm::Aes128GcmAlgorithm;
use aes256gcm::Aes256GcmAlgorithm;
use std::str::FromStr;

pub const AES256GCM_NAME: &str = "aes256gcm";
pub const AES128GCM_NAME: &str = "aes128gcm";
pub const DEFAULT_ALGO_NAME: &str = AES256GCM_NAME;
pub static ALGORITHM_NAMES: &'static [&'static str] = &[AES128GCM_NAME, AES256GCM_NAME];

pub enum AlgoType {
    Aes128Gcm,
    Aes256Gcm,
}

impl FromStr for AlgoType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            AES128GCM_NAME => Ok(AlgoType::Aes128Gcm),
            AES256GCM_NAME => Ok(AlgoType::Aes256Gcm),
            _ => Err("no match"),
        }
    }
}

#[derive(Debug)]
pub enum AlgoError {
    MalformedKey,
    EncryptionFailed,
    DecryptionFailed,
    InvalidAlgorithm(String),
}

pub trait Algorithm {
    fn generate_key_data(&self) -> Vec<u8>;
    fn encrypt_data(&self, key: &[u8], input: &[u8]) -> Result<Vec<u8>, AlgoError>;
    fn decrypt_data(&self, key: &[u8], input: &[u8]) -> Result<Vec<u8>, AlgoError>;
}

pub fn select_algorithm(name: AlgoType) -> Result<Box<dyn Algorithm>, AlgoError> {
    match name {
        AlgoType::Aes256Gcm => Ok(Box::new(Aes256GcmAlgorithm {})),
        AlgoType::Aes128Gcm => Ok(Box::new(Aes128GcmAlgorithm {})),
    }
}

fn generate_256_bit_key_data() -> Vec<u8> {
    let mut key_data = [0u8; 32];
    thread_rng().fill_bytes(&mut key_data);
    key_data.to_vec()
}

fn generate_128_bit_key_data() -> Vec<u8> {
    let mut key_data = [0u8; 16];
    thread_rng().fill_bytes(&mut key_data);
    key_data.to_vec()
}
