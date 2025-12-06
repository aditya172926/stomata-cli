use sha3::{Digest, Keccak256};

use crate::constants::EVM_ADDRESS_HEX_LENGTH;

pub struct AddressValidator;

#[derive(Debug, PartialEq)]
pub enum ValidationResult {
    Valid { checksummed: String },
    InvalidLength,
    InvalidPrefix,
    InvalidCharacters,
}

impl AddressValidator {
    pub fn validate(address: &str) -> ValidationResult {
        // checking length 0x + 40 hex characters
        if address.len() != EVM_ADDRESS_HEX_LENGTH {
            return ValidationResult::InvalidLength;
        }

        // check prefix
        if !address.starts_with("0x") {
            return ValidationResult::InvalidPrefix;
        }

        let addr_without_prefix = &address[2..];

        // check all chars are valid hex digits
        if !addr_without_prefix.chars().all(|c| c.is_ascii_hexdigit()) {
            return ValidationResult::InvalidCharacters;
        }

        // checksum
        let checksummed = Self::checksum_encode(addr_without_prefix);
        return ValidationResult::Valid {
            checksummed: format!("0x{checksummed}"),
        };
    }

    fn checksum_encode(address: &str) -> String {
        let address_lower = address.to_lowercase();
        let hash = Self::keccak256(address_lower.as_bytes());
        let hash_hex = hex::encode(hash);

        address_lower
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_ascii_digit() {
                    c
                } else {
                    let hash_char = hash_hex.chars().nth(i).unwrap();
                    let hash_value = hash_char.to_digit(16).unwrap();
                    if hash_value >= 8 {
                        c.to_ascii_uppercase()
                    } else {
                        c
                    }
                }
            })
            .collect()
    }

    fn keccak256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_checksummed_address() {
        let addr = "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed";
        let result = AddressValidator::validate(addr);
        assert!(matches!(result, ValidationResult::Valid { .. }))
    }

    #[test]
    fn test_valid_lowercase_address() {
        let addr = "0x5aaeb6053f3e94c9b9a09f33669435e7ef1beaed";
        let result = AddressValidator::validate(addr);
        assert!(matches!(result, ValidationResult::Valid { .. }));
    }

    #[test]
    fn test_invalid_length() {
        let addr = "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAe";
        let result = AddressValidator::validate(addr);
        assert_eq!(result, ValidationResult::InvalidLength);
    }

    #[test]
    fn test_invalid_prefix() {
        let addr = "5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed12";
        let result = AddressValidator::validate(addr);
        assert_eq!(result, ValidationResult::InvalidPrefix);
    }

    #[test]
    fn test_invalid_characters() {
        let addr = "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAeZ";
        let result = AddressValidator::validate(addr);
        assert_eq!(result, ValidationResult::InvalidCharacters);
    }
}
