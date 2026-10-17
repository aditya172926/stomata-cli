//! Encrypted key management utilities
//!
//! Provides secure storage, retrieval, and management of encrypted keys
//! using password-based encryption. Keys are stored locally in encrypted
//! form and can only be decrypted with the correct password.

use std::process::exit;

use stomata_web3::providers::{delete_key, list_keys, retrieve_key, store_key};

use crate::features::web3::cli::OutputFormat;

/// Securely prompts the user for sensitive information without echoing to terminal.
///
/// Uses `rpassword` to read password or key input without displaying characters
/// on the screen. Exits the program if input reading fails.
///
/// # Arguments
///
/// * `ask_text` - The prompt text to display to the user
///
/// # Returns
///
/// The entered string, or exits the program on error
///
/// # Examples
///
/// ```ignore
/// let password = ask_sensitive_info("Enter password: ");
/// let key = ask_sensitive_info("Enter private key: ");
/// ```
///
/// # Panics
///
/// Calls `exit(0)` if reading from stdin fails
fn ask_sensitive_info(ask_text: &str) -> String {
    let info = match rpassword::prompt_password(ask_text) {
        Ok(pw) => pw,
        Err(_err) => {
            eprintln!("Error in reading entered data");
            exit(0)
        }
    };
    info
}

/// Encrypts and stores a key with password-based encryption.
///
/// Prompts the user for a password and the key to encrypt, then stores
/// the encrypted key under the given name. The key is encrypted using
/// the provided password and can only be retrieved with the same password.
///
/// # Arguments
///
/// * `name` - Identifier for the stored key (used for later retrieval)
///
/// # User Prompts
///
/// 1. "Password: " - Encryption password (hidden input)
/// 2. "Key to encrypt: " - The sensitive key data (hidden input)
///
/// # Errors
///
/// Prints an error message to stderr if encryption or storage fails,
/// but does not exit the program.
///
/// # Examples
///
/// ```ignore
/// use crate::features::web3::crypto::encrypt_key;
///
/// // User will be prompted for password and key
/// encrypt_key("my_wallet_key".to_string());
/// ```
///
/// # Security Notes
///
/// - Password is never stored, only used for encryption
/// - Key input is not echoed to terminal
/// - Encrypted data is stored locally by `stomata_web3`
pub fn encrypt_key(name: String) {
    let password = ask_sensitive_info("Password: ");
    let pk = ask_sensitive_info("Key to encrypt: ");
    let res = store_key(name.as_str(), pk.as_bytes(), password.as_str());
    if let Err(err) = res {
        eprintln!("Error in encrypting key {:?}", err);
    }
}

/// Decrypts and displays a stored encrypted key.
///
/// Prompts the user for the password used during encryption, then retrieves
/// and decrypts the key. The decrypted key is displayed in the requested format.
///
/// # Arguments
///
/// * `name` - Identifier of the stored key to decrypt
/// * `format` - Output format for the decrypted key (Hex or UTF-8)
///
/// # User Prompts
///
/// 1. "Password: " - Decryption password (hidden input)
///
/// # Output Formats
///
/// - `OutputFormat::Hex` - Displays key as hexadecimal string
/// - `OutputFormat::Utf8` - Displays key as UTF-8 string
///
/// # Errors
///
/// Silently fails if:
/// - Key name doesn't exist
/// - Password is incorrect
/// - Decryption fails
/// - UTF-8 conversion fails (for UTF-8 format)
///
/// # Examples
///
/// ```ignore
/// use crate::features::web3::crypto::{decrypt_key, OutputFormat};
///
/// // Display as hex
/// decrypt_key("my_wallet_key".to_string(), OutputFormat::Hex);
///
/// // Display as UTF-8
/// decrypt_key("my_api_key".to_string(), OutputFormat::Utf8);
/// ```
///
/// # Security Notes
///
/// - Password verification is implicit (wrong password = decryption failure)
/// - Decrypted data is printed to stdout (use with caution)
pub fn decrypt_key(name: String, format: OutputFormat) {
    let password = ask_sensitive_info("Password: ");
    let res = retrieve_key(name.as_str(), password.as_str());
    if let Ok(data) = res {
        match format {
            OutputFormat::Hex => println!("{:?}", hex::encode(&data)),
            OutputFormat::Utf8 => println!(
                "{:?}",
                String::from_utf8(data).expect("Failed to decrypt key to utf-8")
            ),
        }
    };
}

/// Lists all stored encrypted key names.
///
/// Displays the identifiers of all keys currently stored in the encrypted
/// key storage. Does not display the actual key data or require passwords.
///
/// # Output
///
/// Prints each key name on a separate line to stdout.
///
/// # Errors
///
/// Silently fails if unable to retrieve the key list.
///
/// # Examples
///
/// ```ignore
/// use crate::features::web3::crypto::list_all_keys;
///
/// list_all_keys();
/// // Output:
/// // my_wallet_key
/// // my_api_key
/// // backup_key
/// ```
pub fn list_all_keys() {
    let keys = list_keys();
    if let Ok(res) = keys {
        for key in res {
            println!("{key}");
        }
    }
}

/// Deletes a stored encrypted key.
///
/// Permanently removes the encrypted key with the given identifier from
/// storage. This operation does not require the password and cannot be undone.
///
/// # Arguments
///
/// * `name` - Identifier of the key to delete
///
/// # Errors
///
/// Prints an error message to stderr if:
/// - Key doesn't exist
/// - Deletion fails due to filesystem errors
///
/// # Examples
///
/// ```ignore
/// use crate::features::web3::crypto::delete_encrypted_key;
///
/// delete_encrypted_key("old_wallet_key".to_string());
/// ```
///
/// # Security Notes
///
/// - No password verification required (intentional for key rotation)
/// - Deletion is permanent
/// - Consider backing up important keys before deletion
pub fn delete_encrypted_key(name: String) {
    if let Err(err) = delete_key(&name) {
        eprintln!("Error in deleting key {name}: {:?}", err);
    }
}
