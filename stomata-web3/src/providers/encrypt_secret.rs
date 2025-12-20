use aes_gcm::{
    Aes256Gcm, KeyInit, Nonce,
    aead::{Aead, OsRng},
};
use argon2::{Argon2, password_hash::SaltString};
use rand::random;

pub struct EncryptPrivateKey {
    pub crypto_key: CryptoData,
}

pub struct CryptoData {
    pub cipher: String,
    pub salt: String,
    pub nonce: String,
    pub ciphertext: String,
}

fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .unwrap();
    key
}

pub fn encrpt_private_key(pk: &[u8], password: &str) -> Option<EncryptPrivateKey> {
    let salt = SaltString::generate(&mut OsRng);
    let nonce = random::<[u8; 12]>();
    let key = derive_key(password, salt.as_str().as_bytes());
    let cipher = match Aes256Gcm::new_from_slice(&key) {
        Ok(res) => res,
        Err(_err) => {
            return None;
        }
    };
    let ciphertext = match cipher.encrypt(Nonce::from_slice(&nonce), pk) {
        Ok(c_text) => c_text,
        Err(_err) => {
            return None;
        }
    };

    Some(EncryptPrivateKey {
        crypto_key: CryptoData {
            cipher: "aes-256-gcm".to_string(),
            salt: hex::encode(salt.as_str().as_bytes()),
            nonce: hex::encode(nonce),
            ciphertext: hex::encode(ciphertext),
        },
    })
}
