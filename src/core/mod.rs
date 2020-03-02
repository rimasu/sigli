use rand::{thread_rng, RngCore};

mod aes128gcm;
mod aes256gcm;

use aes128gcm::Aes128GcmAlgorithm;
use aes256gcm::Aes256GcmAlgorithm;

#[derive(Debug)]
pub enum CoreError {
    MalformedKey,
    EncryptionFailed,
    DecryptionFailed,
    InvalidAlgorithm(String),
}

const AES256GCM_NAME: &str = "aes256gcm";
const AES128GCM_NAME: &str = "aes128gcm";
pub const DEFAULT_NAME: &str = AES256GCM_NAME;
pub static ALGORITHM_NAMES: &'static [&'static str] = &[AES128GCM_NAME, AES256GCM_NAME];

pub trait Algorithm {
    fn generate_key_text(&self) -> Vec<u8>;
    fn encrypt_data(&self, key: &[u8], input: &[u8]) -> Result<Vec<u8>, CoreError>;
    fn decrypt_data(&self, key: &[u8], input: &[u8]) -> Result<Vec<u8>, CoreError>;
}

pub fn select_algorithm(name: &str) -> Result<Box<dyn Algorithm>, CoreError> {
    match name {
        AES256GCM_NAME => Ok(Box::new(Aes256GcmAlgorithm {})),
        AES128GCM_NAME => Ok(Box::new(Aes128GcmAlgorithm {})),
        _ => Err(CoreError::InvalidAlgorithm(name.to_owned())),
    }
}

fn parse_hex_key(key: &[u8], out: &mut [u8]) -> Result<(), CoreError> {
    let key_text = std::str::from_utf8(key).map_err(|_| CoreError::MalformedKey)?;

    let clean_key = key_text.replace("-", "").replace(" ", "");

    hex::decode_to_slice(clean_key, out).map_err(|_| CoreError::MalformedKey)
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

fn pretty_hex_encode_key_data(key_data: &[u8]) -> String {
    let mut key_text = String::new();

    for (idx, c) in hex::encode_upper(&key_data).chars().enumerate() {
        key_text.push(c);
        if idx % 4 == 3 {
            key_text.push('-')
        }
    }

    if key_text.chars().last() == Some('-') {
        key_text.remove(key_text.len() - 1);
    }

    key_text
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_encrypt_and_decrypt_binary_data() {
        let input_data = b"some input data".to_vec();
        for algo_name in ALGORITHM_NAMES {
            let algo = select_algorithm(algo_name).unwrap();
            let key_text = algo.generate_key_text();
            let cipher_data = algo.encrypt_data(&key_text, &input_data).unwrap();
            let output_data = algo.decrypt_data(&key_text, &cipher_data).unwrap();
            assert_eq!(input_data, output_data);
        }
    }

    #[test]
    fn encrypting_same_input_twice_creates_different_cipher_texts() {
        let input_data = b"some input data".to_vec();
        for algo_name in ALGORITHM_NAMES {
            let algo = select_algorithm(algo_name).unwrap();
            let key_text = algo.generate_key_text();
            let cipher_data1 = algo.encrypt_data(&key_text, &input_data).unwrap();
            let cipher_data2 = algo.encrypt_data(&key_text, &input_data).unwrap();
            assert_ne!(cipher_data1, cipher_data2);
        }
    }

    #[test]
    fn generating_key_twice_creates_different_results() {
        for algo_name in ALGORITHM_NAMES {
            let algo = select_algorithm(algo_name).unwrap();
            let key_text1 = algo.generate_key_text();
            let key_text2 = algo.generate_key_text();
            assert_ne!(key_text1, key_text2)
        }
    }
}
