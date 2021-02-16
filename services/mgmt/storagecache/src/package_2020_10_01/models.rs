#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiOperation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<api_operation::Display>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(rename = "isDataAction", skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<api_operation::Properties>,
}
pub mod api_operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "serviceSpecification", skip_serializing_if = "Option::is_none")]
        pub service_specification: Option<properties::ServiceSpecification>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct ServiceSpecification {
            #[serde(rename = "metricSpecifications", skip_serializing_if = "Vec::is_empty")]
            pub metric_specifications: Vec<MetricSpecification>,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiOperationListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApiOperation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AscOperation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AscOperationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AscOperationProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cache {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<UrlString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<CacheIdentity>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<cache::Properties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<cache::Sku>,
}
pub mod cache {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "cacheSizeGB", skip_serializing_if = "Option::is_none")]
        pub cache_size_gb: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub health: Option<CacheHealth>,
        #[serde(rename = "mountAddresses", skip_serializing)]
        pub mount_addresses: Vec<String>,
        #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subnet: Option<UrlString>,
        #[serde(rename = "upgradeStatus", skip_serializing_if = "Option::is_none")]
        pub upgrade_status: Option<CacheUpgradeStatus>,
        #[serde(rename = "networkSettings", skip_serializing_if = "Option::is_none")]
        pub network_settings: Option<CacheNetworkSettings>,
        #[serde(rename = "encryptionSettings", skip_serializing_if = "Option::is_none")]
        pub encryption_settings: Option<CacheEncryptionSettings>,
        #[serde(rename = "securitySettings", skip_serializing_if = "Option::is_none")]
        pub security_settings: Option<CacheSecuritySettings>,
        #[serde(rename = "directoryServicesSettings", skip_serializing_if = "Option::is_none")]
        pub directory_services_settings: Option<CacheDirectorySettings>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Cancelled,
            Creating,
            Deleting,
            Updating,
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Sku {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheIdentity {
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<cache_identity::Type>,
}
pub mod cache_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheNetworkSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[serde(rename = "utilityAddresses", skip_serializing)]
    pub utility_addresses: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheEncryptionSettings {
    #[serde(rename = "keyEncryptionKey", skip_serializing_if = "Option::is_none")]
    pub key_encryption_key: Option<KeyVaultKeyReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheSecuritySettings {
    #[serde(rename = "accessPolicies", skip_serializing_if = "Vec::is_empty")]
    pub access_policies: Vec<NfsAccessPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheDirectorySettings {
    #[serde(rename = "activeDirectory", skip_serializing_if = "Option::is_none")]
    pub active_directory: Option<CacheActiveDirectorySettings>,
    #[serde(rename = "usernameDownload", skip_serializing_if = "Option::is_none")]
    pub username_download: Option<CacheUsernameDownloadSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheActiveDirectorySettings {
    #[serde(rename = "primaryDnsIpAddress")]
    pub primary_dns_ip_address: String,
    #[serde(rename = "secondaryDnsIpAddress", skip_serializing_if = "Option::is_none")]
    pub secondary_dns_ip_address: Option<String>,
    #[serde(rename = "domainName")]
    pub domain_name: String,
    #[serde(rename = "domainNetBiosName")]
    pub domain_net_bios_name: String,
    #[serde(rename = "cacheNetBiosName")]
    pub cache_net_bios_name: String,
    #[serde(rename = "domainJoined", skip_serializing)]
    pub domain_joined: Option<cache_active_directory_settings::DomainJoined>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<cache_active_directory_settings::Credentials>,
}
pub mod cache_active_directory_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DomainJoined {
        Yes,
        No,
        Error,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Credentials {
        pub username: String,
        pub password: String,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheUsernameDownloadSettings {
    #[serde(rename = "extendedGroups", skip_serializing_if = "Option::is_none")]
    pub extended_groups: Option<bool>,
    #[serde(rename = "usernameSource", skip_serializing_if = "Option::is_none")]
    pub username_source: Option<cache_username_download_settings::UsernameSource>,
    #[serde(rename = "groupFileURI", skip_serializing_if = "Option::is_none")]
    pub group_file_uri: Option<String>,
    #[serde(rename = "userFileURI", skip_serializing_if = "Option::is_none")]
    pub user_file_uri: Option<String>,
    #[serde(rename = "ldapServer", skip_serializing_if = "Option::is_none")]
    pub ldap_server: Option<String>,
    #[serde(rename = "ldapBaseDN", skip_serializing_if = "Option::is_none")]
    pub ldap_base_dn: Option<String>,
    #[serde(rename = "encryptLdapConnection", skip_serializing_if = "Option::is_none")]
    pub encrypt_ldap_connection: Option<bool>,
    #[serde(rename = "requireValidCertificate", skip_serializing_if = "Option::is_none")]
    pub require_valid_certificate: Option<bool>,
    #[serde(rename = "autoDownloadCertificate", skip_serializing_if = "Option::is_none")]
    pub auto_download_certificate: Option<bool>,
    #[serde(rename = "caCertificateURI", skip_serializing_if = "Option::is_none")]
    pub ca_certificate_uri: Option<String>,
    #[serde(rename = "usernameDownloaded", skip_serializing)]
    pub username_downloaded: Option<cache_username_download_settings::UsernameDownloaded>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<cache_username_download_settings::Credentials>,
}
pub mod cache_username_download_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UsernameSource {
        #[serde(rename = "AD")]
        Ad,
        #[serde(rename = "LDAP")]
        Ldap,
        File,
        None,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UsernameDownloaded {
        Yes,
        No,
        Error,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Credentials {
        #[serde(rename = "bindDn", skip_serializing_if = "Option::is_none")]
        pub bind_dn: Option<String>,
        #[serde(rename = "bindPassword", skip_serializing_if = "Option::is_none")]
        pub bind_password: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NfsAccessPolicy {
    pub name: String,
    #[serde(rename = "accessRules")]
    pub access_rules: Vec<NfsAccessRule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NfsAccessRule {
    pub scope: nfs_access_rule::Scope,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    pub access: nfs_access_rule::Access,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suid: Option<bool>,
    #[serde(rename = "submountAccess", skip_serializing_if = "Option::is_none")]
    pub submount_access: Option<bool>,
    #[serde(rename = "rootSquash", skip_serializing_if = "Option::is_none")]
    pub root_squash: Option<bool>,
    #[serde(rename = "anonymousUID", skip_serializing_if = "Option::is_none")]
    pub anonymous_uid: Option<String>,
    #[serde(rename = "anonymousGID", skip_serializing_if = "Option::is_none")]
    pub anonymous_gid: Option<String>,
}
pub mod nfs_access_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Scope {
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "network")]
        Network,
        #[serde(rename = "host")]
        Host,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Access {
        #[serde(rename = "no")]
        No,
        #[serde(rename = "ro")]
        Ro,
        #[serde(rename = "rw")]
        Rw,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultKeyReference {
    #[serde(rename = "keyUrl")]
    pub key_url: String,
    #[serde(rename = "sourceVault")]
    pub source_vault: key_vault_key_reference::SourceVault,
}
pub mod key_vault_key_reference {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SourceVault {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CachesListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cache>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheHealth {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<cache_health::State>,
    #[serde(rename = "statusDescription", skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
}
pub mod cache_health {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Unknown,
        Healthy,
        Degraded,
        Down,
        Transitioning,
        Stopping,
        Stopped,
        Upgrading,
        Flushing,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheUpgradeStatus {
    #[serde(rename = "currentFirmwareVersion", skip_serializing)]
    pub current_firmware_version: Option<String>,
    #[serde(rename = "firmwareUpdateStatus", skip_serializing)]
    pub firmware_update_status: Option<cache_upgrade_status::FirmwareUpdateStatus>,
    #[serde(rename = "firmwareUpdateDeadline", skip_serializing)]
    pub firmware_update_deadline: Option<String>,
    #[serde(rename = "lastFirmwareUpdate", skip_serializing)]
    pub last_firmware_update: Option<String>,
    #[serde(rename = "pendingFirmwareVersion", skip_serializing)]
    pub pending_firmware_version: Option<String>,
}
pub mod cache_upgrade_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirmwareUpdateStatus {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnknownProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTarget {
    #[serde(flatten)]
    pub storage_target_resource: StorageTargetResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageTargetProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTargetResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTargetProperties {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub junctions: Vec<NamespaceJunction>,
    #[serde(rename = "targetType")]
    pub target_type: storage_target_properties::TargetType,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<storage_target_properties::ProvisioningState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs3: Option<Nfs3Target>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clfs: Option<ClfsTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<UnknownTarget>,
}
pub mod storage_target_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TargetType {
        #[serde(rename = "nfs3")]
        Nfs3,
        #[serde(rename = "clfs")]
        Clfs,
        #[serde(rename = "unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Cancelled,
        Creating,
        Deleting,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Nfs3TargetProperties {
    #[serde(flatten)]
    pub storage_target_properties: StorageTargetProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Nfs3Target {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "usageModel", skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClfsTargetProperties {
    #[serde(flatten)]
    pub storage_target_properties: StorageTargetProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClfsTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<UrlString>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnknownTargetProperties {
    #[serde(flatten)]
    pub storage_target_properties: StorageTargetProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnknownTarget {
    #[serde(rename = "unknownMap", skip_serializing_if = "Option::is_none")]
    pub unknown_map: Option<UnknownProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceName {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    #[serde(rename = "resourceType", skip_serializing)]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<ResourceSkuCapabilities>,
    #[serde(skip_serializing)]
    pub locations: Vec<String>,
    #[serde(rename = "locationInfo", skip_serializing_if = "Vec::is_empty")]
    pub location_info: Vec<ResourceSkuLocationInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<Restriction>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Restriction {
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub values: Vec<String>,
    #[serde(rename = "reasonCode", skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<restriction::ReasonCode>,
}
pub mod restriction {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuLocationInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkusResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing)]
    pub value: Vec<ResourceSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceJunction {
    #[serde(rename = "namespacePath", skip_serializing_if = "Option::is_none")]
    pub namespace_path: Option<String>,
    #[serde(rename = "targetPath", skip_serializing_if = "Option::is_none")]
    pub target_path: Option<String>,
    #[serde(rename = "nfsExport", skip_serializing_if = "Option::is_none")]
    pub nfs_export: Option<String>,
    #[serde(rename = "nfsAccessPolicy", skip_serializing_if = "Option::is_none")]
    pub nfs_access_policy: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTargetsResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageTarget>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlString {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<usage_model::Display>,
    #[serde(rename = "modelName", skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[serde(rename = "targetType", skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}
pub mod usage_model {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageModelsResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UsageModel>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayDescription", skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "aggregationType", skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "supportedAggregationTypes", skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
    #[serde(rename = "metricClass", skip_serializing_if = "Option::is_none")]
    pub metric_class: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<MetricDimension>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDimension {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "internalName", skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[serde(rename = "toBeExportedForShoebox", skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemData {
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
pub mod system_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
