use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes128Gcm;
use rand::{thread_rng, RngCore};

use super::{generate_128_bit_key_data, AlgoError, Algorithm};

pub struct Aes128GcmAlgorithm {}

pub const ALGO_NAME: &str = "aes128gcm";

impl Aes128GcmAlgorithm {
    fn create_cipher(key: &[u8]) -> Result<Aes128Gcm, AlgoError> {
        let mut key_data = [0u8; 16];
        key_data.copy_from_slice(key);
        Ok(Aes128Gcm::new(GenericArray::clone_from_slice(&key_data)))
    }
}

impl Algorithm for Aes128GcmAlgorithm {
    fn generate_key_data(&self) -> Vec<u8> {
        generate_128_bit_key_data()
    }

    fn encrypt_data(&self, key: &[u8], input: &[u8]) -> Result<Vec<u8>, AlgoError> {
        let cipher = Self::create_cipher(key)?;
        let mut nonce = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce);

        let mut output = Vec::with_capacity(input.len() + 16);
        output.extend_from_slice(input);

        cipher
            .encrypt_in_place(GenericArray::from_slice(&nonce), &[], &mut output)
            .map_err(|_| AlgoError::EncryptionFailed)?;

        output.extend_from_slice(&nonce);

        Ok(output)
    }

    fn decrypt_data(&self, key: &[u8], input: &[u8]) -> Result<Vec<u8>, AlgoError> {
        let cipher = Self::create_cipher(&key)?;
        let body_len = input.len() - 12;
        let nonce = &input[body_len..];
        let body = &input[..body_len];
        cipher
            .decrypt(GenericArray::from_slice(&nonce), body)
            .map_err(|_| AlgoError::DecryptionFailed)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_round_trip() {
        let algo = Aes128GcmAlgorithm {};
        let input = vec![0xAB, 0x01, 0x02, 0x22, 0x23, 0x43];
        let key = algo.generate_key_data();
        let output = algo.encrypt_data(&key, &input).unwrap();
        assert_ne!(input, output);
        assert_eq!(input, algo.decrypt_data(&key, &output).unwrap());
    }

    #[test]
    fn generates_different_keys() {
        let algo = Aes128GcmAlgorithm {};
        assert_ne!(
            algo.generate_key_data(),
            algo.generate_key_data(),
        )
    }

    #[test]
    fn encrypting_twice_generates_different_cipher_texts() {
        let algo = Aes128GcmAlgorithm {};
        let input = vec![0xAB, 0x01, 0x02, 0x22, 0x23, 0x43];
        let key = algo.generate_key_data();
        assert_ne!(
            algo.encrypt_data(&key, &input).unwrap(),
            algo.encrypt_data(&key, &input).unwrap(),
        )
    }
}


