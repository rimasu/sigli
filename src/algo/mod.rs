use rand::{thread_rng, RngCore};

mod aes128gcm;
mod aes256gcm;

use std::str::FromStr;

pub const DEFAULT_ALGO_NAME: &str = aes256gcm::ALGO_NAME;

pub static ALGORITHM_NAMES: &[&str] = &[aes256gcm::ALGO_NAME, aes128gcm::ALGO_NAME];

/// Format used to either encrypt or decrypt data.
pub enum AlgoType {

    /// AES-GCM with 128bit Key.
    ///
    /// Implemented using [RustCrypto/AEADs]: https://github.com/RustCrypto/AEADs
    ///
    /// Generates an output that is 16 bytes longer than the input.
    /// - 4 bytes of authentication tag appended by algorithm.
    /// - 12 bytes of nonce append by this code.
    Aes128Gcm,

    /// AES-GCM with 256bit Key.
    ///
    /// Implemented using [RustCrypto/AEADs]: https://github.com/RustCrypto/AEADs
    ///
    /// Generates an output that is 16 bytes longer than the input.
    /// - 4 bytes of authentication tag appended by algorithm.
    /// - 12 bytes of nonce append by this code.
    Aes256Gcm,
}

impl FromStr for AlgoType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            aes128gcm::ALGO_NAME => Ok(AlgoType::Aes128Gcm),
            aes256gcm::ALGO_NAME => Ok(AlgoType::Aes256Gcm),
            _ => Err("no match"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AlgoError {
    KeyWrongLength {
        expected_length: usize,
        actual_length: usize,
    },
    EncryptionFailed,
    DecryptionFailed,
}

pub trait Algorithm {
    fn generate_key_data(&self) -> Vec<u8>;
    fn encrypt_data(&self, key: &[u8], data: &mut Vec<u8>) -> Result<(), AlgoError>;
    fn decrypt_data(&self, key: &[u8], data: &mut Vec<u8>) -> Result<(), AlgoError>;
}

pub fn select_algorithm(name: AlgoType) -> Box<dyn Algorithm> {
    match name {
        AlgoType::Aes256Gcm => Box::new(aes256gcm::Aes256GcmAlgorithm {}),
        AlgoType::Aes128Gcm => Box::new(aes128gcm::Aes128GcmAlgorithm {}),
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
