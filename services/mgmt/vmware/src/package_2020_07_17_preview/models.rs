#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trial {
    #[serde(skip_serializing)]
    pub status: Option<trial::Status>,
    #[serde(rename = "availableHosts", skip_serializing)]
    pub available_hosts: Option<i32>,
}
pub mod trial {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        TrialAvailable,
        TrialUsed,
        TrialDisabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quota {
    #[serde(rename = "hostsRemaining", skip_serializing)]
    pub hosts_remaining: Option<serde_json::Value>,
    #[serde(rename = "quotaEnabled", skip_serializing)]
    pub quota_enabled: Option<quota::QuotaEnabled>,
}
pub mod quota {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QuotaEnabled {
        Enabled,
        Disabled,
    }
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
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceTags {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationList {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub display: Option<operation::Display>,
    #[serde(rename = "isDataAction", skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing)]
        pub provider: Option<String>,
        #[serde(skip_serializing)]
        pub resource: Option<String>,
        #[serde(skip_serializing)]
        pub operation: Option<String>,
        #[serde(skip_serializing)]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationProperties {
    #[serde(rename = "serviceSpecification", skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSpecification {
    #[serde(rename = "logSpecifications", skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
    #[serde(rename = "metricSpecifications", skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "blobDuration", skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "aggregationType", skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "supportedAggregationTypes", skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
    #[serde(rename = "supportedTimeGrainTypes", skip_serializing_if = "Vec::is_empty")]
    pub supported_time_grain_types: Vec<String>,
    #[serde(rename = "fillGapWithZero", skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<MetricDimension>,
    #[serde(rename = "enableRegionalMdmAccount", skip_serializing_if = "Option::is_none")]
    pub enable_regional_mdm_account: Option<String>,
    #[serde(rename = "sourceMdmAccount", skip_serializing_if = "Option::is_none")]
    pub source_mdm_account: Option<String>,
    #[serde(rename = "sourceMdmNamespace", skip_serializing_if = "Option::is_none")]
    pub source_mdm_namespace: Option<String>,
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
pub struct ExpressRouteAuthorization {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExpressRouteAuthorizationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpressRouteAuthorizationProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<express_route_authorization_properties::ProvisioningState>,
    #[serde(rename = "expressRouteAuthorizationId", skip_serializing)]
    pub express_route_authorization_id: Option<String>,
    #[serde(rename = "expressRouteAuthorizationKey", skip_serializing)]
    pub express_route_authorization_key: Option<String>,
}
pub mod express_route_authorization_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpressRouteAuthorizationList {
    #[serde(skip_serializing)]
    pub value: Vec<ExpressRouteAuthorization>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Circuit {
    #[serde(rename = "primarySubnet", skip_serializing)]
    pub primary_subnet: Option<String>,
    #[serde(rename = "secondarySubnet", skip_serializing)]
    pub secondary_subnet: Option<String>,
    #[serde(rename = "expressRouteID", skip_serializing)]
    pub express_route_id: Option<String>,
    #[serde(rename = "expressRoutePrivatePeeringID", skip_serializing)]
    pub express_route_private_peering_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Endpoints {
    #[serde(rename = "nsxtManager", skip_serializing)]
    pub nsxt_manager: Option<String>,
    #[serde(skip_serializing)]
    pub vcsa: Option<String>,
    #[serde(rename = "hcxCloudManager", skip_serializing)]
    pub hcx_cloud_manager: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentitySource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "baseUserDN", skip_serializing_if = "Option::is_none")]
    pub base_user_dn: Option<String>,
    #[serde(rename = "baseGroupDN", skip_serializing_if = "Option::is_none")]
    pub base_group_dn: Option<String>,
    #[serde(rename = "primaryServer", skip_serializing_if = "Option::is_none")]
    pub primary_server: Option<String>,
    #[serde(rename = "secondaryServer", skip_serializing_if = "Option::is_none")]
    pub secondary_server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<identity_source::Ssl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
pub mod identity_source {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Ssl {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloud {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub sku: Sku,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateCloudProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloudUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateCloudUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloudUpdateProperties {
    #[serde(rename = "managementCluster", skip_serializing_if = "Option::is_none")]
    pub management_cluster: Option<ManagementCluster>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet: Option<private_cloud_update_properties::Internet>,
    #[serde(rename = "identitySources", skip_serializing_if = "Vec::is_empty")]
    pub identity_sources: Vec<IdentitySource>,
}
pub mod private_cloud_update_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Internet {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloudProperties {
    #[serde(flatten)]
    pub private_cloud_update_properties: PrivateCloudUpdateProperties,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<private_cloud_properties::ProvisioningState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit: Option<Circuit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Endpoints>,
    #[serde(rename = "networkBlock")]
    pub network_block: String,
    #[serde(rename = "managementNetwork", skip_serializing)]
    pub management_network: Option<String>,
    #[serde(rename = "provisioningNetwork", skip_serializing)]
    pub provisioning_network: Option<String>,
    #[serde(rename = "vmotionNetwork", skip_serializing)]
    pub vmotion_network: Option<String>,
    #[serde(rename = "vcenterPassword", skip_serializing_if = "Option::is_none")]
    pub vcenter_password: Option<String>,
    #[serde(rename = "nsxtPassword", skip_serializing_if = "Option::is_none")]
    pub nsxt_password: Option<String>,
    #[serde(rename = "vcenterCertificateThumbprint", skip_serializing)]
    pub vcenter_certificate_thumbprint: Option<String>,
    #[serde(rename = "nsxtCertificateThumbprint", skip_serializing)]
    pub nsxt_certificate_thumbprint: Option<String>,
}
pub mod private_cloud_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Cancelled,
        Pending,
        Building,
        Deleting,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub resource: Resource,
    pub sku: Sku,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterUpdateProperties {
    #[serde(rename = "clusterSize", skip_serializing_if = "Option::is_none")]
    pub cluster_size: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommonClusterProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<common_cluster_properties::ProvisioningState>,
    #[serde(rename = "clusterSize", skip_serializing_if = "Option::is_none")]
    pub cluster_size: Option<i32>,
    #[serde(rename = "clusterId", skip_serializing)]
    pub cluster_id: Option<i32>,
    #[serde(skip_serializing)]
    pub hosts: Vec<String>,
}
pub mod common_cluster_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Cancelled,
        Deleting,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementCluster {
    #[serde(flatten)]
    pub common_cluster_properties: CommonClusterProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterProperties {
    #[serde(flatten)]
    pub common_cluster_properties: CommonClusterProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloudList {
    #[serde(skip_serializing)]
    pub value: Vec<PrivateCloud>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterList {
    #[serde(skip_serializing)]
    pub value: Vec<Cluster>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Addon {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AddonProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddonUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AddonUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddonUpdateProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AddonProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddonProperties {
    #[serde(flatten)]
    pub addon_srm_properties: AddonSrmProperties,
    #[serde(rename = "addonType", skip_serializing_if = "Option::is_none")]
    pub addon_type: Option<addon_properties::AddonType>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<addon_properties::ProvisioningState>,
}
pub mod addon_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AddonType {
        #[serde(rename = "SRM")]
        Srm,
        #[serde(rename = "VR")]
        Vr,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Cancelled,
        Deleting,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddonSrmProperties {
    #[serde(rename = "licenseKey", skip_serializing_if = "Option::is_none")]
    pub license_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddonList {
    #[serde(skip_serializing)]
    pub value: Vec<Addon>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminCredentials {
    #[serde(rename = "nsxtUsername", skip_serializing)]
    pub nsxt_username: Option<String>,
    #[serde(rename = "nsxtPassword", skip_serializing)]
    pub nsxt_password: Option<String>,
    #[serde(rename = "vcenterUsername", skip_serializing)]
    pub vcenter_username: Option<String>,
    #[serde(rename = "vcenterPassword", skip_serializing)]
    pub vcenter_password: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HcxEnterpriseSiteList {
    #[serde(skip_serializing)]
    pub value: Vec<HcxEnterpriseSite>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HcxEnterpriseSite {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HcxEnterpriseSiteProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HcxEnterpriseSiteProperties {
    #[serde(rename = "activationKey", skip_serializing)]
    pub activation_key: Option<String>,
    #[serde(skip_serializing)]
    pub status: Option<hcx_enterprise_site_properties::Status>,
}
pub mod hcx_enterprise_site_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Available,
        Consumed,
        Deactivated,
        Deleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalReachConnectionList {
    #[serde(skip_serializing)]
    pub value: Vec<GlobalReachConnection>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalReachConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GlobalReachConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalReachConnectionProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<global_reach_connection_properties::ProvisioningState>,
    #[serde(rename = "addressPrefix", skip_serializing)]
    pub address_prefix: Option<String>,
    #[serde(rename = "authorizationKey", skip_serializing_if = "Option::is_none")]
    pub authorization_key: Option<String>,
    #[serde(rename = "circuitConnectionStatus", skip_serializing)]
    pub circuit_connection_status: Option<global_reach_connection_properties::CircuitConnectionStatus>,
    #[serde(rename = "peerExpressRouteCircuit", skip_serializing_if = "Option::is_none")]
    pub peer_express_route_circuit: Option<String>,
}
pub mod global_reach_connection_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Updating,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CircuitConnectionStatus {
        Connected,
        Connecting,
        Disconnected,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkSegmentSubnet {
    #[serde(rename = "dhcpRanges", skip_serializing_if = "Vec::is_empty")]
    pub dhcp_ranges: Vec<String>,
    #[serde(rename = "gatewayAddress", skip_serializing_if = "Option::is_none")]
    pub gateway_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkSegmentPortVif {
    #[serde(rename = "portName", skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkSegmentProperties {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "connectedGateway", skip_serializing_if = "Option::is_none")]
    pub connected_gateway: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<WorkloadNetworkSegmentSubnet>,
    #[serde(rename = "portVif", skip_serializing)]
    pub port_vif: Vec<WorkloadNetworkSegmentPortVif>,
    #[serde(skip_serializing)]
    pub status: Option<workload_network_segment_properties::Status>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<workload_network_segment_properties::ProvisioningState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
pub mod workload_network_segment_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "SUCCESS, FAILURE")]
        SuccessFailure,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Building,
        Deleting,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkSegment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkSegmentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkSegmentsList {
    #[serde(skip_serializing)]
    pub value: Vec<WorkloadNetworkSegment>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDhcpEntity {
    #[serde(rename = "dhcpType")]
    pub dhcp_type: workload_network_dhcp_entity::DhcpType,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing)]
    pub segments: Vec<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<workload_network_dhcp_entity::ProvisioningState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
pub mod workload_network_dhcp_entity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DhcpType {
        #[serde(rename = "SERVER, RELAY")]
        ServerRelay,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Building,
        Deleting,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDhcpServer {
    #[serde(flatten)]
    pub workload_network_dhcp_entity: WorkloadNetworkDhcpEntity,
    #[serde(rename = "serverAddress", skip_serializing_if = "Option::is_none")]
    pub server_address: Option<String>,
    #[serde(rename = "leaseTime", skip_serializing_if = "Option::is_none")]
    pub lease_time: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDhcpRelay {
    #[serde(flatten)]
    pub workload_network_dhcp_entity: WorkloadNetworkDhcpEntity,
    #[serde(rename = "serverAddresses", skip_serializing_if = "Vec::is_empty")]
    pub server_addresses: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDhcp {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkDhcpEntity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDhcpList {
    #[serde(skip_serializing)]
    pub value: Vec<WorkloadNetworkDhcp>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkPortMirroringProperties {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<workload_network_port_mirroring_properties::Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(skip_serializing)]
    pub status: Option<workload_network_port_mirroring_properties::Status>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<workload_network_port_mirroring_properties::ProvisioningState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
pub mod workload_network_port_mirroring_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Direction {
        #[serde(rename = "INGRESS, EGRESS, BIDIRECTIONAL")]
        IngressEgressBidirectional,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "SUCCESS, FAILURE")]
        SuccessFailure,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Building,
        Deleting,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkPortMirroring {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkPortMirroringProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkPortMirroringList {
    #[serde(skip_serializing)]
    pub value: Vec<WorkloadNetworkPortMirroring>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkVmGroupProperties {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<String>,
    #[serde(skip_serializing)]
    pub status: Option<workload_network_vm_group_properties::Status>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<workload_network_vm_group_properties::ProvisioningState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
pub mod workload_network_vm_group_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "SUCCESS, FAILURE")]
        SuccessFailure,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Building,
        Deleting,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkVmGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkVmGroupProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkVmGroupsList {
    #[serde(skip_serializing)]
    pub value: Vec<WorkloadNetworkVmGroup>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkVirtualMachineProperties {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "vmType", skip_serializing)]
    pub vm_type: Option<workload_network_virtual_machine_properties::VmType>,
}
pub mod workload_network_virtual_machine_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VmType {
        #[serde(rename = "REGULAR, EDGE, SERVICE")]
        RegularEdgeService,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkVirtualMachine {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkVirtualMachineProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkVirtualMachinesList {
    #[serde(skip_serializing)]
    pub value: Vec<WorkloadNetworkVirtualMachine>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkGatewayProperties {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing)]
    pub path: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkGateway {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkGatewayProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkGatewayList {
    #[serde(skip_serializing)]
    pub value: Vec<WorkloadNetworkGateway>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDnsServiceProperties {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "dnsServiceIp", skip_serializing_if = "Option::is_none")]
    pub dns_service_ip: Option<String>,
    #[serde(rename = "defaultDnsZone", skip_serializing_if = "Option::is_none")]
    pub default_dns_zone: Option<String>,
    #[serde(rename = "fqdnZones", skip_serializing_if = "Vec::is_empty")]
    pub fqdn_zones: Vec<String>,
    #[serde(rename = "logLevel", skip_serializing_if = "Option::is_none")]
    pub log_level: Option<workload_network_dns_service_properties::LogLevel>,
    #[serde(skip_serializing)]
    pub status: Option<workload_network_dns_service_properties::Status>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<workload_network_dns_service_properties::ProvisioningState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
pub mod workload_network_dns_service_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LogLevel {
        #[serde(rename = "DEBUG")]
        Debug,
        #[serde(rename = "INFO")]
        Info,
        #[serde(rename = "WARNING")]
        Warning,
        #[serde(rename = "ERROR")]
        Error,
        #[serde(rename = "FATAL")]
        Fatal,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "SUCCESS")]
        Success,
        #[serde(rename = "FAILURE")]
        Failure,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Building,
        Deleting,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDnsService {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkDnsServiceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDnsServicesList {
    #[serde(skip_serializing)]
    pub value: Vec<WorkloadNetworkDnsService>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDnsZoneProperties {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub domain: Vec<String>,
    #[serde(rename = "dnsServerIps", skip_serializing_if = "Vec::is_empty")]
    pub dns_server_ips: Vec<String>,
    #[serde(rename = "sourceIp", skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
    #[serde(rename = "dnsServices", skip_serializing_if = "Option::is_none")]
    pub dns_services: Option<i64>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<workload_network_dns_zone_properties::ProvisioningState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
pub mod workload_network_dns_zone_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Building,
        Deleting,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDnsZone {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkDnsZoneProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDnsZonesList {
    #[serde(skip_serializing)]
    pub value: Vec<WorkloadNetworkDnsZone>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorResponse>,
    #[serde(rename = "additionalInfo", skip_serializing)]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub info: Option<serde_json::Value>,
}
