#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookListCredentialsResult {
    #[serde(rename = "primaryAccessKey", skip_serializing_if = "Option::is_none")]
    pub primary_access_key: Option<String>,
    #[serde(rename = "secondaryAccessKey", skip_serializing_if = "Option::is_none")]
    pub secondary_access_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookResourceInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "notebookPreparationError", skip_serializing_if = "Option::is_none")]
    pub notebook_preparation_error: Option<NotebookPreparationError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookPreparationError {
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "statusCode", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceProperties {
    #[serde(rename = "workspaceId", skip_serializing)]
    pub workspace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "creationTime", skip_serializing)]
    pub creation_time: Option<String>,
    #[serde(rename = "keyVault", skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<String>,
    #[serde(rename = "applicationInsights", skip_serializing_if = "Option::is_none")]
    pub application_insights: Option<String>,
    #[serde(rename = "containerRegistry", skip_serializing_if = "Option::is_none")]
    pub container_registry: Option<String>,
    #[serde(rename = "storageAccount", skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<String>,
    #[serde(rename = "discoveryUrl", skip_serializing_if = "Option::is_none")]
    pub discovery_url: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<workspace_properties::ProvisioningState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionProperty>,
    #[serde(rename = "hbiWorkspace", skip_serializing_if = "Option::is_none")]
    pub hbi_workspace: Option<bool>,
    #[serde(rename = "serviceProvisionedResourceGroup", skip_serializing)]
    pub service_provisioned_resource_group: Option<String>,
    #[serde(rename = "privateLinkCount", skip_serializing)]
    pub private_link_count: Option<i32>,
    #[serde(rename = "imageBuildCompute", skip_serializing_if = "Option::is_none")]
    pub image_build_compute: Option<String>,
    #[serde(rename = "allowPublicAccessWhenBehindVnet", skip_serializing_if = "Option::is_none")]
    pub allow_public_access_when_behind_vnet: Option<bool>,
    #[serde(rename = "privateEndpointConnections", skip_serializing)]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[serde(rename = "sharedPrivateLinkResources", skip_serializing_if = "Vec::is_empty")]
    pub shared_private_link_resources: Vec<SharedPrivateLinkResource>,
    #[serde(rename = "notebookInfo", skip_serializing_if = "Option::is_none")]
    pub notebook_info: Option<NotebookResourceInfo>,
}
pub mod workspace_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Unknown,
        Updating,
        Creating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspacePropertiesUpdateParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePropertiesUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmlUserFeature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListAmlUserFeatureResult {
    #[serde(skip_serializing)]
    pub value: Vec<AmlUserFeature>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageName {
    #[serde(skip_serializing)]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", skip_serializing)]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub unit: Option<usage::Unit>,
    #[serde(rename = "currentValue", skip_serializing)]
    pub current_value: Option<i64>,
    #[serde(skip_serializing)]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<UsageName>,
}
pub mod usage {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListUsagesResult {
    #[serde(skip_serializing)]
    pub value: Vec<Usage>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineSize {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub family: Option<String>,
    #[serde(rename = "vCPUs", skip_serializing)]
    pub v_cp_us: Option<i32>,
    #[serde(skip_serializing)]
    pub gpus: Option<i32>,
    #[serde(rename = "osVhdSizeMB", skip_serializing)]
    pub os_vhd_size_mb: Option<i32>,
    #[serde(rename = "maxResourceVolumeMB", skip_serializing)]
    pub max_resource_volume_mb: Option<i32>,
    #[serde(rename = "memoryGB", skip_serializing)]
    pub memory_gb: Option<f64>,
    #[serde(rename = "lowPriorityCapable", skip_serializing)]
    pub low_priority_capable: Option<bool>,
    #[serde(rename = "premiumIO", skip_serializing)]
    pub premium_io: Option<bool>,
    #[serde(rename = "estimatedVMPrices", skip_serializing_if = "Option::is_none")]
    pub estimated_vm_prices: Option<EstimatedVmPrices>,
    #[serde(rename = "supportedComputeTypes", skip_serializing_if = "Vec::is_empty")]
    pub supported_compute_types: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EstimatedVmPrices {
    #[serde(rename = "billingCurrency")]
    pub billing_currency: estimated_vm_prices::BillingCurrency,
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: estimated_vm_prices::UnitOfMeasure,
    pub values: Vec<EstimatedVmPrice>,
}
pub mod estimated_vm_prices {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BillingCurrency {
        #[serde(rename = "USD")]
        Usd,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UnitOfMeasure {
        OneHour,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EstimatedVmPrice {
    #[serde(rename = "retailPrice")]
    pub retail_price: f64,
    #[serde(rename = "osType")]
    pub os_type: estimated_vm_price::OsType,
    #[serde(rename = "vmTier")]
    pub vm_tier: estimated_vm_price::VmTier,
}
pub mod estimated_vm_price {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Linux,
        Windows,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VmTier {
        Standard,
        LowPriority,
        Spot,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineSizeListResult {
    #[serde(rename = "amlCompute", skip_serializing_if = "Vec::is_empty")]
    pub aml_compute: Vec<VirtualMachineSize>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaBaseProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<quota_base_properties::Unit>,
}
pub mod quota_base_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaUpdateParameters {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<QuotaBaseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateWorkspaceQuotasResult {
    #[serde(skip_serializing)]
    pub value: Vec<UpdateWorkspaceQuotas>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateWorkspaceQuotas {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing)]
    pub unit: Option<update_workspace_quotas::Unit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<update_workspace_quotas::Status>,
}
pub mod update_workspace_quotas {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Undefined,
        Success,
        Failure,
        InvalidQuotaBelowClusterMinimum,
        InvalidQuotaExceedsSubscriptionLimit,
        #[serde(rename = "InvalidVMFamilyName")]
        InvalidVmFamilyName,
        OperationNotSupportedForSku,
        OperationNotEnabledForRegion,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceName {
    #[serde(skip_serializing)]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", skip_serializing)]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceQuota {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[serde(skip_serializing)]
    pub limit: Option<i64>,
    #[serde(skip_serializing)]
    pub unit: Option<resource_quota::Unit>,
}
pub mod resource_quota {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListWorkspaceQuotas {
    #[serde(skip_serializing)]
    pub value: Vec<ResourceQuota>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: identity::Type,
    #[serde(rename = "userAssignedIdentities", skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        #[serde(rename = "SystemAssigned,UserAssigned")]
        SystemAssignedUserAssigned,
        UserAssigned,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceId {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListWorkspaceKeysResult {
    #[serde(rename = "userStorageKey", skip_serializing)]
    pub user_storage_key: Option<String>,
    #[serde(rename = "userStorageResourceId", skip_serializing)]
    pub user_storage_resource_id: Option<String>,
    #[serde(rename = "appInsightsInstrumentationKey", skip_serializing)]
    pub app_insights_instrumentation_key: Option<String>,
    #[serde(rename = "containerRegistryCredentials", skip_serializing_if = "Option::is_none")]
    pub container_registry_credentials: Option<RegistryListCredentialsResult>,
    #[serde(rename = "notebookAccessKeys", skip_serializing_if = "Option::is_none")]
    pub notebook_access_keys: Option<NotebookListCredentialsResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryListCredentialsResult {
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub passwords: Vec<Password>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Password {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedComputeResourcesList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ComputeResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Compute {
    #[serde(rename = "computeType")]
    pub compute_type: ComputeType,
    #[serde(rename = "computeLocation", skip_serializing_if = "Option::is_none")]
    pub compute_location: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<compute::ProvisioningState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "createdOn", skip_serializing)]
    pub created_on: Option<String>,
    #[serde(rename = "modifiedOn", skip_serializing)]
    pub modified_on: Option<String>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "provisioningErrors", skip_serializing)]
    pub provisioning_errors: Vec<MachineLearningServiceError>,
    #[serde(rename = "isAttachedCompute", skip_serializing)]
    pub is_attached_compute: Option<bool>,
}
pub mod compute {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Unknown,
        Updating,
        Creating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Aks {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmlCompute {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeInstance {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachine {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HdInsight {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataFactory {
    #[serde(flatten)]
    pub compute: Compute,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Databricks {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeAnalytics {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalCredentials {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemService {
    #[serde(rename = "systemServiceType", skip_serializing)]
    pub system_service_type: Option<String>,
    #[serde(rename = "publicIpAddress", skip_serializing)]
    pub public_ip_address: Option<String>,
    #[serde(skip_serializing)]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SslConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ssl_configuration::Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
}
pub mod ssl_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Disabled,
        Enabled,
        Auto,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AksNetworkingConfiguration {
    #[serde(rename = "subnetId", skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "serviceCidr", skip_serializing_if = "Option::is_none")]
    pub service_cidr: Option<String>,
    #[serde(rename = "dnsServiceIP", skip_serializing_if = "Option::is_none")]
    pub dns_service_ip: Option<String>,
    #[serde(rename = "dockerBridgeCidr", skip_serializing_if = "Option::is_none")]
    pub docker_bridge_cidr: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAccountCredentials {
    #[serde(rename = "adminUserName")]
    pub admin_user_name: String,
    #[serde(rename = "adminUserSshPublicKey", skip_serializing_if = "Option::is_none")]
    pub admin_user_ssh_public_key: Option<String>,
    #[serde(rename = "adminUserPassword", skip_serializing_if = "Option::is_none")]
    pub admin_user_password: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleSettings {
    #[serde(rename = "maxNodeCount")]
    pub max_node_count: i64,
    #[serde(rename = "minNodeCount", skip_serializing_if = "Option::is_none")]
    pub min_node_count: Option<i64>,
    #[serde(rename = "nodeIdleTimeBeforeScaleDown", skip_serializing_if = "Option::is_none")]
    pub node_idle_time_before_scale_down: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeStateCounts {
    #[serde(rename = "idleNodeCount", skip_serializing)]
    pub idle_node_count: Option<i32>,
    #[serde(rename = "runningNodeCount", skip_serializing)]
    pub running_node_count: Option<i32>,
    #[serde(rename = "preparingNodeCount", skip_serializing)]
    pub preparing_node_count: Option<i32>,
    #[serde(rename = "unusableNodeCount", skip_serializing)]
    pub unusable_node_count: Option<i32>,
    #[serde(rename = "leavingNodeCount", skip_serializing)]
    pub leaving_node_count: Option<i32>,
    #[serde(rename = "preemptedNodeCount", skip_serializing)]
    pub preempted_node_count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterUpdateProperties {
    #[serde(rename = "scaleSettings", skip_serializing_if = "Option::is_none")]
    pub scale_settings: Option<ScaleSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeNodesInformation {
    #[serde(rename = "computeType")]
    pub compute_type: ComputeType,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmlComputeNodesInformation {
    #[serde(flatten)]
    pub compute_nodes_information: ComputeNodesInformation,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmlComputeNodeInformation {
    #[serde(rename = "nodeId", skip_serializing)]
    pub node_id: Option<String>,
    #[serde(rename = "privateIpAddress", skip_serializing)]
    pub private_ip_address: Option<String>,
    #[serde(rename = "publicIpAddress", skip_serializing)]
    pub public_ip_address: Option<String>,
    #[serde(skip_serializing)]
    pub port: Option<f64>,
    #[serde(rename = "nodeState", skip_serializing)]
    pub node_state: Option<aml_compute_node_information::NodeState>,
    #[serde(rename = "runId", skip_serializing)]
    pub run_id: Option<String>,
}
pub mod aml_compute_node_information {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NodeState {
        #[serde(rename = "idle")]
        Idle,
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "preparing")]
        Preparing,
        #[serde(rename = "unusable")]
        Unusable,
        #[serde(rename = "leaving")]
        Leaving,
        #[serde(rename = "preempted")]
        Preempted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineSshCredentials {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "publicKeyData", skip_serializing_if = "Option::is_none")]
    pub public_key_data: Option<String>,
    #[serde(rename = "privateKeyData", skip_serializing_if = "Option::is_none")]
    pub private_key_data: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeSecrets {
    #[serde(rename = "computeType")]
    pub compute_type: ComputeType,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AksComputeSecrets {
    #[serde(flatten)]
    pub compute_secrets: ComputeSecrets,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineSecrets {
    #[serde(flatten)]
    pub compute_secrets: ComputeSecrets,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabricksComputeSecrets {
    #[serde(flatten)]
    pub compute_secrets: ComputeSecrets,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ComputeType {
    #[serde(rename = "AKS")]
    Aks,
    AmlCompute,
    ComputeInstance,
    DataFactory,
    VirtualMachine,
    #[serde(rename = "HDInsight")]
    HdInsight,
    Databricks,
    DataLakeAnalytics,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineLearningServiceError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuLocationInfo {
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub zones: Vec<String>,
    #[serde(rename = "zoneDetails", skip_serializing)]
    pub zone_details: Vec<ResourceSkuZoneDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuZoneDetails {
    #[serde(skip_serializing)]
    pub name: Vec<String>,
    #[serde(skip_serializing)]
    pub capabilities: Vec<SkuCapability>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceSku {
    #[serde(rename = "resourceType", skip_serializing)]
    pub resource_type: Option<String>,
    #[serde(skip_serializing)]
    pub skus: Vec<SkuSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuSettings {
    #[serde(skip_serializing)]
    pub locations: Vec<String>,
    #[serde(rename = "locationInfo", skip_serializing)]
    pub location_info: Vec<ResourceSkuLocationInfo>,
    #[serde(skip_serializing)]
    pub tier: Option<String>,
    #[serde(rename = "resourceType", skip_serializing)]
    pub resource_type: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub capabilities: Vec<SkuCapability>,
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
        NotSpecified,
        NotAvailableForRegion,
        NotAvailableForSubscription,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkspaceSku>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpoint {
    #[serde(skip_serializing)]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "actionsRequired", skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    Disconnected,
    Timeout,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", skip_serializing)]
    pub group_id: Option<String>,
    #[serde(rename = "requiredMembers", skip_serializing)]
    pub required_members: Vec<String>,
    #[serde(rename = "requiredZoneNames", skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedPrivateLinkResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedPrivateLinkResourceProperty>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedPrivateLinkResourceProperty {
    #[serde(rename = "privateLinkResourceId", skip_serializing_if = "Option::is_none")]
    pub private_link_resource_id: Option<String>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "requestMessage", skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionProperty {
    pub status: encryption_property::Status,
    #[serde(rename = "keyVaultProperties")]
    pub key_vault_properties: KeyVaultProperties,
}
pub mod encryption_property {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultProperties {
    #[serde(rename = "keyVaultArmId")]
    pub key_vault_arm_id: String,
    #[serde(rename = "keyIdentifier")]
    pub key_identifier: String,
    #[serde(rename = "identityClientId", skip_serializing_if = "Option::is_none")]
    pub identity_client_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedWorkspaceConnectionsList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkspaceConnection>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceConnection {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceConnectionProps>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceConnectionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceConnectionProps>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceConnectionProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "authType", skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeInstanceSshSettings {
    #[serde(rename = "sshPublicAccess", skip_serializing_if = "Option::is_none")]
    pub ssh_public_access: Option<compute_instance_ssh_settings::SshPublicAccess>,
    #[serde(rename = "adminUserName", skip_serializing)]
    pub admin_user_name: Option<String>,
    #[serde(rename = "sshPort", skip_serializing)]
    pub ssh_port: Option<i32>,
    #[serde(rename = "adminPublicKey", skip_serializing_if = "Option::is_none")]
    pub admin_public_key: Option<String>,
}
pub mod compute_instance_ssh_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SshPublicAccess {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ComputeInstanceState {
    Creating,
    CreateFailed,
    Deleting,
    Running,
    Restarting,
    JobRunning,
    SettingUp,
    SetupFailed,
    Starting,
    Stopped,
    Stopping,
    UserSettingUp,
    UserSetupFailed,
    Unknown,
    Unusable,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeInstanceLastOperation {
    #[serde(rename = "operationName", skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<compute_instance_last_operation::OperationName>,
    #[serde(rename = "operationTime", skip_serializing_if = "Option::is_none")]
    pub operation_time: Option<String>,
    #[serde(rename = "operationStatus", skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<compute_instance_last_operation::OperationStatus>,
}
pub mod compute_instance_last_operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OperationName {
        Create,
        Start,
        Stop,
        Restart,
        Reimage,
        Delete,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OperationStatus {
        InProgress,
        Succeeded,
        CreateFailed,
        StartFailed,
        StopFailed,
        RestartFailed,
        ReimageFailed,
        DeleteFailed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeInstanceApplication {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "endpointUri", skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeInstanceConnectivityEndpoints {
    #[serde(rename = "publicIpAddress", skip_serializing)]
    pub public_ip_address: Option<String>,
    #[serde(rename = "privateIpAddress", skip_serializing)]
    pub private_ip_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeInstanceCreatedBy {
    #[serde(rename = "userName", skip_serializing)]
    pub user_name: Option<String>,
    #[serde(rename = "userOrgId", skip_serializing)]
    pub user_org_id: Option<String>,
    #[serde(rename = "userId", skip_serializing)]
    pub user_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersonalComputeInstanceSettings {
    #[serde(rename = "assignedUser", skip_serializing_if = "Option::is_none")]
    pub assigned_user: Option<AssignedUser>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignedUser {
    #[serde(rename = "objectId")]
    pub object_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}
