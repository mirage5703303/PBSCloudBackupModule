use anyhow::{bail, Error};

use proxmox_schema::{ApiStringFormat, Schema, StringSchema};

use crate::{CLOUD_STORAGE_NAME_SCHEMA, PROXMOX_SAFE_ID_FORMAT};

pub const BUCKET_NAME_SCHEMA: Schema = StringSchema::new("Bucket name.")
    .format(&PROXMOX_SAFE_ID_FORMAT)
    .min_length(3)
    .max_length(63)
    .schema();

#[derive(Debug, PartialEq, Eq, Clone)]
/// Cloud backup location
pub enum BackupLocation {
    /// Data is available in a cloud storage bucket
    Cloud(String),
    /// Local storage, available for upload
    Local,
    /// Archived in a specific storage vault
    Vault(String),
}

proxmox_serde::forward_deserialize_to_from_str!(BackupLocation);
proxmox_serde::forward_serialize_to_display!(BackupLocation);

impl proxmox_schema::ApiType for BackupLocation {
    const API_SCHEMA: Schema = StringSchema::new(
        "Backup location (e.g. 'local', 'cloud-<bucket_name>', 'vault-<vault_name>')",
    )
    .format(&ApiStringFormat::VerifyFn(|text| {
        let location: BackupLocation = text.parse()?;
        match location {
            BackupLocation::Cloud(ref bucket) => {
                BUCKET_NAME_SCHEMA.parse_simple_value(bucket)?;
            }
            BackupLocation::Vault(ref vault) => {
                VAULT_NAME_SCHEMA.parse_simple_value(vault)?;
            }
            BackupLocation::Local => { /* OK */ }
        }
        Ok(())
    }))
    .schema();
}

impl std::fmt::Display for BackupLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupLocation::Local => {
                write!(f, "local")
            }
            BackupLocation::Cloud(bucket) => {
                write!(f, "cloud-{}", bucket)
            }
            BackupLocation::Vault(vault) => {
                write!(f, "vault-{}", vault)
            }
        }
    }
}

impl std::str::FromStr for BackupLocation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "local" {
            return Ok(BackupLocation::Local);
        }
        if let Some(bucket) = s.strip_prefix("cloud-") {
            return Ok(BackupLocation::Cloud(bucket.to_string()));
        }
        if let Some(vault) = s.strip_prefix("vault-") {
            return Ok(BackupLocation::Vault(vault.to_string()));
        }

        bail!("BackupLocation parse error");
    }
}
