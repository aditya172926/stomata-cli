//! Address validation utilities
//!
//! Provides functions for validating blockchain addresses using the
//! stomata_web3 address validation system. Used for verifying address
//! format and checksums across different blockchain networks.

use stomata_web3::providers::address::AddressValidator;

/// Validates a blockchain address and prints the validation result.
///
/// Performs comprehensive validation on the provided address string,
/// checking format, checksum, and network compatibility. The validation
/// result is printed to stdout for debugging purposes.
///
/// # Arguments
///
/// * `address` - The blockchain address string to validate (e.g., Ethereum address)
///
/// # Validation Checks
///
/// The validator typically checks:
/// - Address format (length, character set)
/// - Checksum validity (if applicable)
/// - Network-specific requirements
///
/// # Examples
///
/// ```ignore
/// use crate::validate_address;
///
/// // Valid Ethereum address
/// validate_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb");
/// // Output: ValidationResult::Valid
///
/// // Invalid address
/// validate_address("0xinvalid");
/// // Output: ValidationResult::Invalid { reason: ... }
/// ```
///
/// # Output Format
///
/// The function prints the `ValidationResult` enum using debug formatting,
/// which includes detailed information about validation success or failure.
///
/// # Notes
///
/// - This function is primarily for debugging and CLI utilities
/// - For production use, consider using `AddressValidator::validate()` directly
///   and handling the `ValidationResult` programmatically
/// - The validation logic is provided by the `stomata_web3` crate
pub fn validate_address(address: &str) {
    let result = AddressValidator::validate(address);
    println!("{:?}", result);
}
