use std::str::FromStr;

use serde::de::{value, IntoDeserializer};
use serde::{Deserialize, Serialize};

use proxmox_lang::constnamedbitmap;
use proxmox_schema::{
    api, const_regex, ApiStringFormat, BooleanSchema, EnumEntry, Schema, StringSchema,
};

const_regex! {
    pub ACL_PATH_REGEX = concat!(r"^(?:/|", r"(?:/", PROXMOX_SAFE_ID_REGEX_STR!(), ")+", r")$");
}

// Define Privilege bitfield for Cloud Backup

constnamedbitmap! {
    /// Contains a list of privilege name to privilege value mappings for cloud backups.
    PRIVILEGES: u64 => {
        /// Cloud.Audit allows reading cloud backup configuration and status
        PRIV_CLOUD_AUDIT("Cloud.Audit");
        /// Cloud.Modify allows modifying cloud backup configuration
        PRIV_CLOUD_MODIFY("Cloud.Modify");
        /// Cloud.Backup allows creating new backups in the cloud
        PRIV_CLOUD_BACKUP("Cloud.Backup");
        /// Cloud.Restore allows restoring backups from the cloud
        PRIV_CLOUD_RESTORE("Cloud.Restore");
        /// Cloud.Delete allows deleting backups from the cloud
        PRIV_CLOUD_DELETE("Cloud.Delete");
    }
}

pub fn privs_to_priv_names(privs: u64) -> Vec<&'static str> {
    PRIVILEGES
        .iter()
        .fold(Vec::new(), |mut priv_names, (name, value)| {
            if value & privs != 0 {
                priv_names.push(name);
            }
            priv_names
        })
}

/// Admin always has all privileges.
pub const ROLE_ADMIN: u64 = u64::MAX;

/// NoAccess can be used to remove privileges from specific (sub-)paths
pub const ROLE_NO_ACCESS: u64 = 0;

#[rustfmt::skip]
#[allow(clippy::identity_op)]
/// Cloud.Audit can view cloud backup configuration and status information, but not modify it.
pub const ROLE_CLOUD_AUDIT: u64 = 0
    | PRIV_CLOUD_AUDIT;

#[rustfmt::skip]
#[allow(clippy::identity_op)]
/// Cloud.Admin can do anything on the cloud backup.
pub const ROLE_CLOUD_ADMIN: u64 = 0
    | PRIV_CLOUD_AUDIT
    | PRIV_CLOUD_MODIFY
    | PRIV_CLOUD_BACKUP
    | PRIV_CLOUD_RESTORE
    | PRIV_CLOUD_DELETE;

#[rustfmt::skip]
#[allow(clippy::identity_op)]
/// Cloud.User can perform backup and restore operations.
pub const ROLE_CLOUD_USER: u64 = 0
    | PRIV_CLOUD_BACKUP
    | PRIV_CLOUD_RESTORE;

#[api(
    type_text: "<role>",
)]
#[repr(u64)]
#[derive(Serialize, Deserialize)]
/// Enum representing roles via their [PRIVILEGES] combination for cloud backups.
pub enum Role {
    /// Administrator
    Admin = ROLE_ADMIN,
    /// Disable Access
    NoAccess = ROLE_NO_ACCESS,
    /// Cloud Auditor
    CloudAudit = ROLE_CLOUD_AUDIT,
    /// Cloud Administrator
    CloudAdmin = ROLE_CLOUD_ADMIN,
    /// Cloud User (perform backups and restores)
    CloudUser = ROLE_CLOUD_USER,
}

impl FromStr for Role {
    type Err = value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}

pub const ACL_PATH_FORMAT: ApiStringFormat = ApiStringFormat::Pattern(&ACL_PATH_REGEX);

pub const ACL_PATH_SCHEMA: Schema = StringSchema::new("Access control path.")
    .format(&ACL_PATH_FORMAT)
    .min_length(1)
    .max_length(128)
    .schema();

pub const ACL_PROPAGATE_SCHEMA: Schema =
    BooleanSchema::new("Allow to propagate (inherit) permissions.")
        .default(true)
        .schema();

pub const ACL_UGID_TYPE_SCHEMA: Schema = StringSchema::new("Type of 'ugid' property.")
    .format(&ApiStringFormat::Enum(&[
        EnumEntry::new("user", "User"),
        EnumEntry::new("group", "Group"),
    ]))
    .schema();

#[api(
    properties: {
        propagate: {
            schema: ACL_PROPAGATE_SCHEMA,
        },
        path: {
            schema: ACL_PATH_SCHEMA,
        },
        ugid_type: {
            schema: ACL_UGID_TYPE_SCHEMA,
        },
        ugid: {
            type: String,
            description: "User or Group ID.",
        },
        roleid: {
            type: Role,
        }
    }
)]
#[derive(Serialize, Deserialize, Clone, PartialEq)]
/// ACL list entry for cloud backup.
pub struct AclListItem {
    pub path: String,
    pub ugid: String,
    pub ugid_type: String,
    pub propagate: bool,
    pub roleid: String,
}
