use serde::{Deserialize, Serialize};

use proxmox_schema::api;

use crate::CLOUD_CERT_FINGERPRINT_SHA256_SCHEMA;

#[api(default: "scrypt")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/// Key derivation function for password-protected encryption keys in cloud backups.
pub enum Kdf {
    /// Do not encrypt the key.
    None,
    /// Encrypt the key with a password using SCrypt.
    Scrypt,
    /// Encrypt the key with a password using PBKDF2.
    PBKDF2,
}

impl Default for Kdf {
    #[inline]
    fn default() -> Self {
        Kdf::Scrypt
    }
}

#[api(
    properties: {
        kdf: {
            type: Kdf,
        },
        fingerprint: {
            schema: CLOUD_CERT_FINGERPRINT_SHA256_SCHEMA,
            optional: true,
        },
    },
)]
#[derive(Deserialize, Serialize)]
/// Cloud Encryption Key Information
pub struct CloudKeyInfo {
    /// Path to the key file (if stored in a file)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub kdf: Kdf,
    /// Key creation time
    pub created: i64,
    /// Key modification time
    pub modified: i64,
    /// Key fingerprint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Password hint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}
