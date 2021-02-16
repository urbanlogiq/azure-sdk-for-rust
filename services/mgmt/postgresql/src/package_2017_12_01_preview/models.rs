#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerVersion {
    #[serde(rename = "9.5")]
    _9_5,
    #[serde(rename = "9.6")]
    _9_6,
    #[serde(rename = "10")]
    _10,
    #[serde(rename = "10.0")]
    _10_0,
    #[serde(rename = "10.2")]
    _10_2,
    #[serde(rename = "11")]
    _11,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SslEnforcement {
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MinimalTlsVersion {
    #[serde(rename = "TLS1_0")]
    Tls10,
    #[serde(rename = "TLS1_1")]
    Tls11,
    #[serde(rename = "TLS1_2")]
    Tls12,
    #[serde(rename = "TLSEnforcementDisabled")]
    TlsEnforcementDisabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPrivateEndpointConnection {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpointProperty>,
    #[serde(rename = "privateLinkServiceConnectionState", skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionStateProperty>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<private_endpoint_connection_properties::ProvisioningState>,
}
pub mod private_endpoint_connection_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Approving,
        Ready,
        Dropping,
        Failed,
        Rejecting,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionStateProperty {
    pub status: private_link_service_connection_state_property::Status,
    pub description: String,
    #[serde(rename = "actionsRequired", skip_serializing)]
    pub actions_required: Option<private_link_service_connection_state_property::ActionsRequired>,
}
pub mod private_link_service_connection_state_property {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Approved,
        Pending,
        Rejected,
        Disconnected,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionsRequired {
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerProperties {
    #[serde(rename = "administratorLogin", skip_serializing_if = "Option::is_none")]
    pub administrator_login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ServerVersion>,
    #[serde(rename = "sslEnforcement", skip_serializing_if = "Option::is_none")]
    pub ssl_enforcement: Option<SslEnforcement>,
    #[serde(rename = "minimalTlsVersion", skip_serializing_if = "Option::is_none")]
    pub minimal_tls_version: Option<MinimalTlsVersion>,
    #[serde(rename = "byokEnforcement", skip_serializing)]
    pub byok_enforcement: Option<String>,
    #[serde(rename = "infrastructureEncryption", skip_serializing_if = "Option::is_none")]
    pub infrastructure_encryption: Option<server_properties::InfrastructureEncryption>,
    #[serde(rename = "userVisibleState", skip_serializing_if = "Option::is_none")]
    pub user_visible_state: Option<server_properties::UserVisibleState>,
    #[serde(rename = "fullyQualifiedDomainName", skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<String>,
    #[serde(rename = "earliestRestoreDate", skip_serializing_if = "Option::is_none")]
    pub earliest_restore_date: Option<String>,
    #[serde(rename = "storageProfile", skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[serde(rename = "replicationRole", skip_serializing_if = "Option::is_none")]
    pub replication_role: Option<String>,
    #[serde(rename = "masterServerId", skip_serializing_if = "Option::is_none")]
    pub master_server_id: Option<String>,
    #[serde(rename = "replicaCapacity", skip_serializing_if = "Option::is_none")]
    pub replica_capacity: Option<i32>,
    #[serde(rename = "publicNetworkAccess", skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<server_properties::PublicNetworkAccess>,
    #[serde(rename = "privateEndpointConnections", skip_serializing)]
    pub private_endpoint_connections: Vec<ServerPrivateEndpointConnection>,
}
pub mod server_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum InfrastructureEncryption {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UserVisibleState {
        Ready,
        Dropping,
        Disabled,
        Inaccessible,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageProfile {
    #[serde(rename = "backupRetentionDays", skip_serializing_if = "Option::is_none")]
    pub backup_retention_days: Option<i32>,
    #[serde(rename = "geoRedundantBackup", skip_serializing_if = "Option::is_none")]
    pub geo_redundant_backup: Option<storage_profile::GeoRedundantBackup>,
    #[serde(rename = "storageMB", skip_serializing_if = "Option::is_none")]
    pub storage_mb: Option<i32>,
    #[serde(rename = "storageAutogrow", skip_serializing_if = "Option::is_none")]
    pub storage_autogrow: Option<storage_profile::StorageAutogrow>,
}
pub mod storage_profile {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum GeoRedundantBackup {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StorageAutogrow {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ServerVersion>,
    #[serde(rename = "sslEnforcement", skip_serializing_if = "Option::is_none")]
    pub ssl_enforcement: Option<SslEnforcement>,
    #[serde(rename = "minimalTlsVersion", skip_serializing_if = "Option::is_none")]
    pub minimal_tls_version: Option<MinimalTlsVersion>,
    #[serde(rename = "storageProfile", skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[serde(rename = "createMode")]
    pub create_mode: server_properties_for_create::CreateMode,
}
pub mod server_properties_for_create {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateMode {
        Default,
        PointInTimeRestore,
        GeoRestore,
        Replica,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForDefaultCreate {
    #[serde(flatten)]
    pub server_properties_for_create: ServerPropertiesForCreate,
    #[serde(rename = "administratorLogin")]
    pub administrator_login: String,
    #[serde(rename = "administratorLoginPassword")]
    pub administrator_login_password: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForRestore {
    #[serde(flatten)]
    pub server_properties_for_create: ServerPropertiesForCreate,
    #[serde(rename = "sourceServerId")]
    pub source_server_id: String,
    #[serde(rename = "restorePointInTime")]
    pub restore_point_in_time: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForGeoRestore {
    #[serde(flatten)]
    pub server_properties_for_create: ServerPropertiesForCreate,
    #[serde(rename = "sourceServerId")]
    pub source_server_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForReplica {
    #[serde(flatten)]
    pub server_properties_for_create: ServerPropertiesForCreate,
    #[serde(rename = "sourceServerId")]
    pub source_server_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Basic,
        GeneralPurpose,
        MemoryOptimized,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceIdentity {
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<resource_identity::Type>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
}
pub mod resource_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerForCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    pub properties: ServerPropertiesForCreate,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<server_update_parameters::Properties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
pub mod server_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "storageProfile", skip_serializing_if = "Option::is_none")]
        pub storage_profile: Option<StorageProfile>,
        #[serde(rename = "administratorLoginPassword", skip_serializing_if = "Option::is_none")]
        pub administrator_login_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub version: Option<ServerVersion>,
        #[serde(rename = "sslEnforcement", skip_serializing_if = "Option::is_none")]
        pub ssl_enforcement: Option<SslEnforcement>,
        #[serde(rename = "minimalTlsVersion", skip_serializing_if = "Option::is_none")]
        pub minimal_tls_version: Option<MinimalTlsVersion>,
        #[serde(rename = "replicationRole", skip_serializing_if = "Option::is_none")]
        pub replication_role: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Server>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRuleProperties {
    #[serde(rename = "startIpAddress")]
    pub start_ip_address: String,
    #[serde(rename = "endIpAddress")]
    pub end_ip_address: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: FirewallRuleProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRuleListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FirewallRule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRuleProperties {
    #[serde(rename = "virtualNetworkSubnetId")]
    pub virtual_network_subnet_id: String,
    #[serde(rename = "ignoreMissingVnetServiceEndpoint", skip_serializing_if = "Option::is_none")]
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
    #[serde(skip_serializing)]
    pub state: Option<virtual_network_rule_properties::State>,
}
pub mod virtual_network_rule_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Initializing,
        InProgress,
        Ready,
        Deleting,
        Unknown,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRuleListResult {
    #[serde(skip_serializing)]
    pub value: Vec<VirtualNetworkRule>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Database {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Database>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(rename = "defaultValue", skip_serializing)]
    pub default_value: Option<String>,
    #[serde(rename = "dataType", skip_serializing)]
    pub data_type: Option<String>,
    #[serde(rename = "allowedValues", skip_serializing)]
    pub allowed_values: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Configuration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
    #[serde(skip_serializing)]
    pub provider: Option<String>,
    #[serde(skip_serializing)]
    pub resource: Option<String>,
    #[serde(skip_serializing)]
    pub operation: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(skip_serializing)]
    pub origin: Option<operation::Origin>,
    #[serde(skip_serializing)]
    pub properties: Option<serde_json::Value>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        NotSpecified,
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogFileProperties {
    #[serde(rename = "sizeInKB", skip_serializing_if = "Option::is_none")]
    pub size_in_kb: Option<i64>,
    #[serde(rename = "createdTime", skip_serializing)]
    pub created_time: Option<String>,
    #[serde(rename = "lastModifiedTime", skip_serializing)]
    pub last_modified_time: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogFile {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<LogFileProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogFileListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LogFile>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PerformanceTierServiceLevelObjectives {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(rename = "vCore", skip_serializing_if = "Option::is_none")]
    pub v_core: Option<i32>,
    #[serde(rename = "hardwareGeneration", skip_serializing_if = "Option::is_none")]
    pub hardware_generation: Option<String>,
    #[serde(rename = "maxBackupRetentionDays", skip_serializing_if = "Option::is_none")]
    pub max_backup_retention_days: Option<i32>,
    #[serde(rename = "minBackupRetentionDays", skip_serializing_if = "Option::is_none")]
    pub min_backup_retention_days: Option<i32>,
    #[serde(rename = "maxStorageMB", skip_serializing_if = "Option::is_none")]
    pub max_storage_mb: Option<i32>,
    #[serde(rename = "minStorageMB", skip_serializing_if = "Option::is_none")]
    pub min_storage_mb: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PerformanceTierProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "maxBackupRetentionDays", skip_serializing_if = "Option::is_none")]
    pub max_backup_retention_days: Option<i32>,
    #[serde(rename = "minBackupRetentionDays", skip_serializing_if = "Option::is_none")]
    pub min_backup_retention_days: Option<i32>,
    #[serde(rename = "maxStorageMB", skip_serializing_if = "Option::is_none")]
    pub max_storage_mb: Option<i32>,
    #[serde(rename = "minLargeStorageMB", skip_serializing_if = "Option::is_none")]
    pub min_large_storage_mb: Option<i32>,
    #[serde(rename = "maxLargeStorageMB", skip_serializing_if = "Option::is_none")]
    pub max_large_storage_mb: Option<i32>,
    #[serde(rename = "minStorageMB", skip_serializing_if = "Option::is_none")]
    pub min_storage_mb: Option<i32>,
    #[serde(rename = "serviceLevelObjectives", skip_serializing_if = "Vec::is_empty")]
    pub service_level_objectives: Vec<PerformanceTierServiceLevelObjectives>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PerformanceTierListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PerformanceTierProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailabilityRequest {
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "nameAvailable", skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAlertPolicyProperties {
    pub state: security_alert_policy_properties::State,
    #[serde(rename = "disabledAlerts", skip_serializing_if = "Vec::is_empty")]
    pub disabled_alerts: Vec<String>,
    #[serde(rename = "emailAddresses", skip_serializing_if = "Vec::is_empty")]
    pub email_addresses: Vec<String>,
    #[serde(rename = "emailAccountAdmins", skip_serializing_if = "Option::is_none")]
    pub email_account_admins: Option<bool>,
    #[serde(rename = "storageEndpoint", skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[serde(rename = "storageAccountAccessKey", skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[serde(rename = "retentionDays", skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
}
pub mod security_alert_policy_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerSecurityAlertPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAlertPolicyProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerSecurityAlertPolicyListResult {
    #[serde(skip_serializing)]
    pub value: Vec<ServerSecurityAlertPolicy>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerAdministratorProperties {
    #[serde(rename = "administratorType")]
    pub administrator_type: server_administrator_properties::AdministratorType,
    pub login: String,
    pub sid: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}
pub mod server_administrator_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AdministratorType {
        ActiveDirectory,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerAdministratorResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerAdministratorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerAdministratorResourceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerAdministratorResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoverableServerProperties {
    #[serde(rename = "lastAvailableBackupDateTime", skip_serializing)]
    pub last_available_backup_date_time: Option<String>,
    #[serde(rename = "serviceLevelObjective", skip_serializing)]
    pub service_level_objective: Option<String>,
    #[serde(skip_serializing)]
    pub edition: Option<String>,
    #[serde(rename = "vCore", skip_serializing)]
    pub v_core: Option<i32>,
    #[serde(rename = "hardwareGeneration", skip_serializing)]
    pub hardware_generation: Option<String>,
    #[serde(skip_serializing)]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoverableServerResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecoverableServerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
