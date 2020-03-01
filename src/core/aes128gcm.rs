use aes_gcm::Aes128Gcm;
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};
use rand::{thread_rng, RngCore};

use super::{Algorithm, CoreError, pretty_hex_encode_key_data, generate_128_bit_key_data};

pub struct Aes128GcmAlgorithm {}

impl Aes128GcmAlgorithm {
    fn create_cipher(key_text: &str) -> Result<Aes128Gcm, CoreError> {
        let clean_key = key_text
            .replace("-", "")
            .replace(" ", "");

        let mut key = [0u8; 16];

        hex::decode_to_slice(clean_key, &mut key)
            .map_err(|_| CoreError::MalformedKey)?;

        Ok(Aes128Gcm::new(GenericArray::clone_from_slice(&key)))
    }
}

impl Algorithm for Aes128GcmAlgorithm {
    fn generate_key_text(&self) -> String {
        pretty_hex_encode_key_data(
            generate_128_bit_key_data().as_slice()
        )
    }

    fn encrypt_data(&self, key_text: &str, input: &[u8]) -> Result<Vec<u8>, CoreError> {
        let cipher = Self::create_cipher(&key_text)?;
        let mut nonce = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce);

        let mut output = Vec::with_capacity(input.len() + 16);
        output.extend_from_slice(input);

        cipher.encrypt_in_place(
            GenericArray::from_slice(&nonce),
            &[],
            &mut output,
        ).map_err(|_| CoreError::EncryptionFailed)?;

        output.extend_from_slice(&nonce);

        Ok(output)
    }

    fn decrypt_data(&self, key_text: &str, input: &[u8]) -> Result<Vec<u8>, CoreError> {
        let cipher = Self::create_cipher(&key_text)?;
        let body_len = input.len() - 12;
        let nonce = &input[body_len..];
        let body = &input[..body_len];
        cipher.decrypt(
            GenericArray::from_slice(&nonce),
            body,
        ).map_err(|_| CoreError::DecryptionFailed)
    }
}
