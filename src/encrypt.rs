use argon2::{Algorithm, Argon2, Params, Version};

use chacha20poly1305::aead::Aead;
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce};

use crate::custom_result::{CustomError, CustomResult};

pub struct Encrypt {
    master_pass: &'static str,
    salt: &'static str,
}

impl Encrypt {
    pub fn new(master_pass: &'static str, salt: &'static str) -> Self {
        Encrypt { master_pass, salt }
    }
    pub fn encrypt(&self, plaintext: &str) -> CustomResult<String> {
        let key = self.derive_key()?;
        let nonce = hex::decode(self.salt)
            .map_err(|err| CustomError::WrongSecretParams(err.to_string()))?;

        if nonce.len() != 12 || key.len() != 32 {
            return Err(CustomError::WrongSecretParams(
                "Invalid key or nonce length".to_string(),
            ));
        }

        // Create a ChaCha20-Poly1305 cipher instance
        let key = Key::from_slice(&key);
        let nonce = Nonce::from_slice(&nonce);
        let cipher = ChaCha20Poly1305::new(key);

        // Encrypt the password
        let cipher_text = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|err| CustomError::CipherError(err.to_string()))?;

        Ok(hex::encode(cipher_text))
    }

    pub fn decrypt(&self, ciphertext: &str) -> CustomResult<String> {
        let key = self.derive_key()?;
        // Ensure the key and IV are the correct length
        let nonce = hex::decode(self.salt)
            .map_err(|err| CustomError::WrongSecretParams(err.to_string()))?;

        if nonce.len() != 12 || key.len() != 32 {
            return Err(CustomError::WrongSecretParams(
                "Invalid key or nonce length".to_string(),
            ));
        }

        // Create a ChaCha20-Poly1305 cipher instance
        let key = Key::from_slice(&key);
        let nonce = Nonce::from_slice(&nonce);
        let cipher = ChaCha20Poly1305::new(key);

        // Decrypt the password
        let ciphertext =
            hex::decode(ciphertext).map_err(|err| CustomError::CipherError(err.to_string()))?;

        let plaintext_encoded = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|err| CustomError::CipherError(err.to_string()))?;
        let plaintext = String::from_utf8(plaintext_encoded)
            .map_err(|err| CustomError::CipherError(err.to_string()))?;
        Ok(plaintext)
    }

    pub fn derive_key(&self) -> CustomResult<Vec<u8>> {
        let salt = self.salt.as_bytes();
        let params = Params::new(65536, 10, 4, Some(32))
            .map_err(|err| CustomError::DerivingKeyError(err.to_string()))?;
        // Configure Argon2
        let argon2 = Argon2::new(
            Algorithm::Argon2id, // Use Argon2id for a balance of security and performance
            Version::V0x13,      // Use the latest version (0x13)
            params,              // Memory cost, time cost, parallelism, output length
        );

        // Derive the key
        let mut output_key_material = vec![0u8; 32]; // 32 bytes = 256 bits
        argon2
            .hash_password_into(self.master_pass.as_bytes(), salt, &mut output_key_material)
            .map_err(|err| CustomError::DerivingKeyError(err.to_string()))?;

        Ok(output_key_material)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let secret_key = "hello";
        let salt = "000102030405060708090a0b"; // 12 bytes
        let crypter = Encrypt::new(secret_key, salt);
        let encrypted = crypter.encrypt("Hello, Rust!");
        assert_eq!(
            encrypted,
            Ok("5957d9925131b10b1cdaeeac176c0b89ea43f379e8942ef4fb95bb91".to_string())
        );
    }

    #[test]
    fn test_decrypt() {
        let secret_key = "hello";
        let salt = "000102030405060708090a0b"; // 12 bytes
        let crypter = Encrypt::new(secret_key, salt);
        let encrypted = crypter.decrypt("5957d9925131b10b1cdaeeac176c0b89ea43f379e8942ef4fb95bb91");
        assert_eq!(encrypted, Ok("Hello, Rust!".to_string()));
    }

    #[test]
    fn test_encrypt_wrong_salt_len() {
        let secret_key = "hello";
        let salt = "000102030405060708090a0b11";
        let crypter = Encrypt::new(secret_key, salt);
        let encrypted = crypter.encrypt("Hello, Rust!");
        assert_eq!(
            encrypted,
            Err(CustomError::WrongSecretParams(
                "Invalid key or nonce length".to_string()
            ))
        );
    }

    #[test]
    fn test_decrypt_wrong_salt_len() {
        let secret_key = "hello";
        let salt = "000102030405060708090a0b11";
        let crypter = Encrypt::new(secret_key, salt);
        let encrypted = crypter.decrypt("5957d9925131b10b1cdaeeac176c0b89ea43f379e8942ef4fb95bb91");
        assert_eq!(
            encrypted,
            Err(CustomError::WrongSecretParams(
                "Invalid key or nonce length".to_string()
            ))
        );
    }

    #[test]
    fn test_encrypt_odd_salt_len() {
        let secret_key = "hello";
        let salt = "000102030405060708090a0b1";
        let crypter = Encrypt::new(secret_key, salt);
        let encrypted = crypter.encrypt("Hello, Rust!");
        assert_eq!(
            encrypted,
            Err(CustomError::WrongSecretParams(
                "Odd number of digits".to_string()
            ))
        );
    }

    #[test]
    fn test_decrypt_odd_salt_len() {
        let secret_key = "hello";
        let salt = "000102030405060708090a0b1";
        let crypter = Encrypt::new(secret_key, salt);
        let encrypted = crypter.decrypt("5957d9925131b10b1cdaeeac176c0b89ea43f379e8942ef4fb95bb91");
        assert_eq!(
            encrypted,
            Err(CustomError::WrongSecretParams(
                "Odd number of digits".to_string()
            ))
        );
    }
}
