use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes128Gcm;
use rand::{thread_rng, RngCore};

use super::{
    generate_128_bit_key_data, Algorithm, CoreError,
};

pub struct Aes128GcmAlgorithm {}

impl Aes128GcmAlgorithm {
    fn create_cipher(key: &[u8]) -> Result<Aes128Gcm, CoreError> {
        let mut key_data = [0u8; 16];
        key_data.copy_from_slice(key);
        Ok(Aes128Gcm::new(GenericArray::clone_from_slice(&key_data)))
    }
}

impl Algorithm for Aes128GcmAlgorithm {
    fn generate_key_data(&self) -> Vec<u8> {
        generate_128_bit_key_data()
    }

    fn encrypt_data(&self, key: &[u8], input: &[u8]) -> Result<Vec<u8>, CoreError> {
        let cipher = Self::create_cipher(key)?;
        let mut nonce = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce);

        let mut output = Vec::with_capacity(input.len() + 16);
        output.extend_from_slice(input);

        cipher
            .encrypt_in_place(GenericArray::from_slice(&nonce), &[], &mut output)
            .map_err(|_| CoreError::EncryptionFailed)?;

        output.extend_from_slice(&nonce);

        Ok(output)
    }

    fn decrypt_data(&self, key: &[u8], input: &[u8]) -> Result<Vec<u8>, CoreError> {
        let cipher = Self::create_cipher(&key)?;
        let body_len = input.len() - 12;
        let nonce = &input[body_len..];
        let body = &input[..body_len];
        cipher
            .decrypt(GenericArray::from_slice(&nonce), body)
            .map_err(|_| CoreError::DecryptionFailed)
    }
}
