use ::serde::{Deserialize, Serialize};

use proxmox_schema::api;

#[api()]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Optional Device Identification Attributes
pub struct OptionalDeviceIdentification {
    /// Vendor (autodetected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    /// Model (autodetected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Serial number (autodetected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
}

#[api()]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Kind of device
pub enum DeviceKind {
    /// Tape changer (Autoloader, Robot)
    Changer,
    /// Normal SCSI tape device
    Tape,
}

#[api(
    properties: {
        kind: {
            type: DeviceKind,
        },
    },
)]
#[derive(Debug, Serialize, Deserialize)]
/// Cloud backup device information
pub struct CloudBackupDeviceInfo {
    pub kind: DeviceKind,
    /// Path to the cloud backup service
    pub service_path: String,
    /// Access key for the cloud backup service
    pub access_key: String,
    /// Secret key for the cloud backup service
    pub secret_key: String,
    /// Bucket name for the cloud backup service
    pub bucket_name: String,
    /// Region for the cloud backup service
    pub region: String,
}