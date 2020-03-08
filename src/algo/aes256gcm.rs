use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes256Gcm;
use rand::{thread_rng, RngCore};

use super::{generate_256_bit_key_data, AlgoError, Algorithm};

pub struct Aes256GcmAlgorithm {}

pub const ALGO_NAME: &str = "aes256gcm";

impl Aes256GcmAlgorithm {
    fn create_cipher(key: &[u8]) -> Result<Aes256Gcm, AlgoError> {
        let mut key_data = [0u8; 32];
        key_data.copy_from_slice(key);
        Ok(Aes256Gcm::new(GenericArray::clone_from_slice(&key_data)))
    }
}

impl Algorithm for Aes256GcmAlgorithm {
    fn generate_key_data(&self) -> Vec<u8> {
        generate_256_bit_key_data()
    }

    fn encrypt_data(&self, key: &[u8], data: &mut Vec<u8>) -> Result<(), AlgoError> {
        let cipher = Self::create_cipher(key)?;
        let mut nonce = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce);

        cipher
            .encrypt_in_place(GenericArray::from_slice(&nonce), &[], data)
            .map_err(|_| AlgoError::EncryptionFailed)?;

        data.extend_from_slice(&nonce);

        Ok(())
    }

    fn decrypt_data(&self, key: &[u8], data: &mut Vec<u8>) -> Result<(), AlgoError> {
        let cipher = Self::create_cipher(&key)?;
        let body_len = data.len() - 12;
        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(&data[body_len..]);

        data.truncate(body_len);
        cipher
            .decrypt_in_place(GenericArray::from_slice(&nonce),&[], data)
            .map_err(|_| AlgoError::DecryptionFailed)?;

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    fn raw_data() -> Vec<u8> {
        vec![0xAB, 0x01, 0x02, 0x22, 0x23, 0x43]
    }

    #[test]
    fn can_round_trip() {
        let algo = Aes256GcmAlgorithm {};
        let key = algo.generate_key_data();
        let mut data = raw_data();

        algo.encrypt_data(&key, &mut data).unwrap();
        assert_ne!(raw_data(), data);
        algo.decrypt_data(&key, &mut data).unwrap();
        assert_eq!(raw_data(), data);
    }

    #[test]
    fn generates_different_keys() {
        let algo = Aes256GcmAlgorithm {};
        assert_ne!(
            algo.generate_key_data(),
            algo.generate_key_data(),
        )
    }

    #[test]
    fn encrypting_twice_generates_different_cipher_texts() {
        let algo = Aes256GcmAlgorithm {};
        let mut data1 = raw_data();
        let mut data2 = raw_data();
        let key = algo.generate_key_data();
        algo.encrypt_data(&key, &mut data1).unwrap();
        algo.encrypt_data(&key, &mut data2).unwrap();
        assert_ne!(data1, data2)
    }
}