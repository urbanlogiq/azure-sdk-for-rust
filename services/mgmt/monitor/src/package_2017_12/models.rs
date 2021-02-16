#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleCapacity {
    pub minimum: String,
    pub maximum: String,
    pub default: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricTrigger {
    #[serde(rename = "metricName")]
    pub metric_name: String,
    #[serde(rename = "metricNamespace", skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    #[serde(rename = "metricResourceUri")]
    pub metric_resource_uri: String,
    #[serde(rename = "timeGrain")]
    pub time_grain: String,
    pub statistic: metric_trigger::Statistic,
    #[serde(rename = "timeWindow")]
    pub time_window: String,
    #[serde(rename = "timeAggregation")]
    pub time_aggregation: metric_trigger::TimeAggregation,
    pub operator: metric_trigger::Operator,
    pub threshold: f64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<ScaleRuleMetricDimension>,
}
pub mod metric_trigger {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Statistic {
        Average,
        Min,
        Max,
        Sum,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TimeAggregation {
        Average,
        Minimum,
        Maximum,
        Total,
        Count,
        Last,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        Equals,
        NotEquals,
        GreaterThan,
        GreaterThanOrEqual,
        LessThan,
        LessThanOrEqual,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleAction {
    pub direction: scale_action::Direction,
    #[serde(rename = "type")]
    pub type_: scale_action::Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub cooldown: String,
}
pub mod scale_action {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Direction {
        None,
        Increase,
        Decrease,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        ChangeCount,
        PercentChangeCount,
        ExactCount,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleRule {
    #[serde(rename = "metricTrigger")]
    pub metric_trigger: MetricTrigger,
    #[serde(rename = "scaleAction")]
    pub scale_action: ScaleAction,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeWindow {
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    pub start: String,
    pub end: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurrentSchedule {
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    pub days: Vec<String>,
    pub hours: Vec<i32>,
    pub minutes: Vec<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recurrence {
    pub frequency: recurrence::Frequency,
    pub schedule: RecurrentSchedule,
}
pub mod recurrence {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Frequency {
        None,
        Second,
        Minute,
        Hour,
        Day,
        Week,
        Month,
        Year,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleProfile {
    pub name: String,
    pub capacity: ScaleCapacity,
    pub rules: Vec<ScaleRule>,
    #[serde(rename = "fixedDate", skip_serializing_if = "Option::is_none")]
    pub fixed_date: Option<TimeWindow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Recurrence>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailNotification {
    #[serde(rename = "sendToSubscriptionAdministrator", skip_serializing_if = "Option::is_none")]
    pub send_to_subscription_administrator: Option<bool>,
    #[serde(rename = "sendToSubscriptionCoAdministrators", skip_serializing_if = "Option::is_none")]
    pub send_to_subscription_co_administrators: Option<bool>,
    #[serde(rename = "customEmails", skip_serializing_if = "Vec::is_empty")]
    pub custom_emails: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookNotification {
    #[serde(rename = "serviceUri", skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleNotification {
    pub operation: autoscale_notification::Operation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailNotification>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub webhooks: Vec<WebhookNotification>,
}
pub mod autoscale_notification {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operation {
        Scale,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSetting {
    pub profiles: Vec<AutoscaleProfile>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notifications: Vec<AutoscaleNotification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "targetResourceUri", skip_serializing_if = "Option::is_none")]
    pub target_resource_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSettingResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: AutoscaleSetting,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSettingResourcePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutoscaleSetting>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSettingResourceCollection {
    pub value: Vec<AutoscaleSettingResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleRuleMetricDimension {
    #[serde(rename = "DimensionName")]
    pub dimension_name: String,
    #[serde(rename = "Operator")]
    pub operator: scale_rule_metric_dimension::Operator,
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}
pub mod scale_rule_metric_dimension {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        Equals,
        NotEquals,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
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
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Incident {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "ruleName", skip_serializing)]
    pub rule_name: Option<String>,
    #[serde(rename = "isActive", skip_serializing)]
    pub is_active: Option<bool>,
    #[serde(rename = "activatedTime", skip_serializing)]
    pub activated_time: Option<String>,
    #[serde(rename = "resolvedTime", skip_serializing)]
    pub resolved_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Incident>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleCondition {
    #[serde(rename = "odata.type")]
    pub odata_type: String,
    #[serde(rename = "dataSource", skip_serializing_if = "Option::is_none")]
    pub data_source: Option<RuleDataSource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleDataSource {
    #[serde(rename = "odata.type")]
    pub odata_type: String,
    #[serde(rename = "resourceUri", skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[serde(rename = "legacyResourceId", skip_serializing_if = "Option::is_none")]
    pub legacy_resource_id: Option<String>,
    #[serde(rename = "resourceLocation", skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[serde(rename = "metricNamespace", skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleMetricDataSource {
    #[serde(flatten)]
    pub rule_data_source: RuleDataSource,
    #[serde(rename = "metricName", skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleManagementEventClaimsDataSource {
    #[serde(rename = "emailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleManagementEventDataSource {
    #[serde(flatten)]
    pub rule_data_source: RuleDataSource,
    #[serde(rename = "eventName", skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(rename = "eventSource", skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(rename = "operationName", skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(rename = "resourceGroupName", skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(rename = "resourceProviderName", skip_serializing_if = "Option::is_none")]
    pub resource_provider_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "subStatus", skip_serializing_if = "Option::is_none")]
    pub sub_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims: Option<RuleManagementEventClaimsDataSource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConditionOperator {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TimeAggregationOperator {
    Average,
    Minimum,
    Maximum,
    Total,
    Last,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThresholdRuleCondition {
    #[serde(flatten)]
    pub rule_condition: RuleCondition,
    pub operator: ConditionOperator,
    pub threshold: f64,
    #[serde(rename = "windowSize", skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
    #[serde(rename = "timeAggregation", skip_serializing_if = "Option::is_none")]
    pub time_aggregation: Option<TimeAggregationOperator>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationThresholdRuleCondition {
    #[serde(flatten)]
    pub rule_condition: RuleCondition,
    #[serde(rename = "windowSize", skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
    #[serde(rename = "failedLocationCount")]
    pub failed_location_count: i32,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementEventAggregationCondition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<ConditionOperator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "windowSize", skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementEventRuleCondition {
    #[serde(flatten)]
    pub rule_condition: RuleCondition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<ManagementEventAggregationCondition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleAction {
    #[serde(rename = "odata.type")]
    pub odata_type: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleEmailAction {
    #[serde(flatten)]
    pub rule_action: RuleAction,
    #[serde(rename = "sendToServiceOwners", skip_serializing_if = "Option::is_none")]
    pub send_to_service_owners: Option<bool>,
    #[serde(rename = "customEmails", skip_serializing_if = "Vec::is_empty")]
    pub custom_emails: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleWebhookAction {
    #[serde(flatten)]
    pub rule_action: RuleAction,
    #[serde(rename = "serviceUri", skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRule {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    pub condition: RuleCondition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleAction>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<RuleAction>,
    #[serde(rename = "lastUpdatedTime", skip_serializing)]
    pub last_updated_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: AlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleResourcePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertRule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleResourceCollection {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AlertRuleResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub enabled: bool,
    pub days: i32,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogProfileProperties {
    #[serde(rename = "storageAccountId", skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[serde(rename = "serviceBusRuleId", skip_serializing_if = "Option::is_none")]
    pub service_bus_rule_id: Option<String>,
    pub locations: Vec<String>,
    pub categories: Vec<String>,
    #[serde(rename = "retentionPolicy")]
    pub retention_policy: RetentionPolicy,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogProfileResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: LogProfileProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogProfileResourcePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<LogProfileProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogProfileCollection {
    pub value: Vec<LogProfileResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyOnlyResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSettings {
    #[serde(rename = "timeGrain", skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub enabled: bool,
    #[serde(rename = "retentionPolicy", skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub enabled: bool,
    #[serde(rename = "retentionPolicy", skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticSettings {
    #[serde(rename = "storageAccountId", skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[serde(rename = "serviceBusRuleId", skip_serializing_if = "Option::is_none")]
    pub service_bus_rule_id: Option<String>,
    #[serde(rename = "eventHubAuthorizationRuleId", skip_serializing_if = "Option::is_none")]
    pub event_hub_authorization_rule_id: Option<String>,
    #[serde(rename = "eventHubName", skip_serializing_if = "Option::is_none")]
    pub event_hub_name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<MetricSettings>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub logs: Vec<LogSettings>,
    #[serde(rename = "workspaceId", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(rename = "logAnalyticsDestinationType", skip_serializing_if = "Option::is_none")]
    pub log_analytics_destination_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticSettingsResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticSettingsResourceCollection {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DiagnosticSettingsResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticSettingsCategory {
    #[serde(rename = "categoryType", skip_serializing_if = "Option::is_none")]
    pub category_type: Option<diagnostic_settings_category::CategoryType>,
}
pub mod diagnostic_settings_category {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CategoryType {
        Metrics,
        Logs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticSettingsCategoryResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticSettingsCategory>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticSettingsCategoryResourceCollection {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DiagnosticSettingsCategoryResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionGroup>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ActionGroupResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroup {
    #[serde(rename = "groupShortName")]
    pub group_short_name: String,
    pub enabled: bool,
    #[serde(rename = "emailReceivers", skip_serializing_if = "Vec::is_empty")]
    pub email_receivers: Vec<EmailReceiver>,
    #[serde(rename = "smsReceivers", skip_serializing_if = "Vec::is_empty")]
    pub sms_receivers: Vec<SmsReceiver>,
    #[serde(rename = "webhookReceivers", skip_serializing_if = "Vec::is_empty")]
    pub webhook_receivers: Vec<WebhookReceiver>,
    #[serde(rename = "itsmReceivers", skip_serializing_if = "Vec::is_empty")]
    pub itsm_receivers: Vec<ItsmReceiver>,
    #[serde(rename = "azureAppPushReceivers", skip_serializing_if = "Vec::is_empty")]
    pub azure_app_push_receivers: Vec<AzureAppPushReceiver>,
    #[serde(rename = "automationRunbookReceivers", skip_serializing_if = "Vec::is_empty")]
    pub automation_runbook_receivers: Vec<AutomationRunbookReceiver>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailReceiver {
    pub name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiverStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsReceiver {
    pub name: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiverStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookReceiver {
    pub name: String,
    #[serde(rename = "serviceUri")]
    pub service_uri: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItsmReceiver {
    pub name: String,
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    #[serde(rename = "ticketConfiguration")]
    pub ticket_configuration: String,
    pub region: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureAppPushReceiver {
    pub name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationRunbookReceiver {
    #[serde(rename = "automationAccountId")]
    pub automation_account_id: String,
    #[serde(rename = "runbookName")]
    pub runbook_name: String,
    #[serde(rename = "webhookResourceId")]
    pub webhook_resource_id: String,
    #[serde(rename = "isGlobalRunbook")]
    pub is_global_runbook: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "serviceUri", skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReceiverStatus {
    NotSpecified,
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableRequest {
    #[serde(rename = "receiverName")]
    pub receiver_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupPatchBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionGroupPatch>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlertResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActivityLogAlert>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlertList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ActivityLogAlertResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlert {
    pub scopes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    pub condition: ActivityLogAlertAllOfCondition,
    pub actions: ActivityLogAlertActionList,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlertAllOfCondition {
    #[serde(rename = "allOf")]
    pub all_of: Vec<ActivityLogAlertLeafCondition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlertLeafCondition {
    pub field: String,
    pub equals: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlertActionList {
    #[serde(rename = "actionGroups", skip_serializing_if = "Vec::is_empty")]
    pub action_groups: Vec<ActivityLogAlertActionGroup>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlertActionGroup {
    #[serde(rename = "actionGroupId")]
    pub action_group_id: String,
    #[serde(rename = "webhookProperties", skip_serializing_if = "Option::is_none")]
    pub webhook_properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlertPatchBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActivityLogAlertPatch>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlertPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalizableString {
    pub value: String,
    #[serde(rename = "localizedValue", skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SenderAuthorization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpRequestInfo {
    #[serde(rename = "clientRequestId", skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[serde(rename = "clientIpAddress", skip_serializing_if = "Option::is_none")]
    pub client_ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<SenderAuthorization>,
    #[serde(skip_serializing)]
    pub claims: Option<serde_json::Value>,
    #[serde(skip_serializing)]
    pub caller: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "eventDataId", skip_serializing)]
    pub event_data_id: Option<String>,
    #[serde(rename = "correlationId", skip_serializing)]
    pub correlation_id: Option<String>,
    #[serde(rename = "eventName", skip_serializing_if = "Option::is_none")]
    pub event_name: Option<LocalizableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<LocalizableString>,
    #[serde(rename = "httpRequest", skip_serializing_if = "Option::is_none")]
    pub http_request: Option<HttpRequestInfo>,
    #[serde(skip_serializing)]
    pub level: Option<event_data::Level>,
    #[serde(rename = "resourceGroupName", skip_serializing)]
    pub resource_group_name: Option<String>,
    #[serde(rename = "resourceProviderName", skip_serializing_if = "Option::is_none")]
    pub resource_provider_name: Option<LocalizableString>,
    #[serde(rename = "resourceId", skip_serializing)]
    pub resource_id: Option<String>,
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<LocalizableString>,
    #[serde(rename = "operationId", skip_serializing)]
    pub operation_id: Option<String>,
    #[serde(rename = "operationName", skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<LocalizableString>,
    #[serde(skip_serializing)]
    pub properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LocalizableString>,
    #[serde(rename = "subStatus", skip_serializing_if = "Option::is_none")]
    pub sub_status: Option<LocalizableString>,
    #[serde(rename = "eventTimestamp", skip_serializing)]
    pub event_timestamp: Option<String>,
    #[serde(rename = "submissionTimestamp", skip_serializing)]
    pub submission_timestamp: Option<String>,
    #[serde(rename = "subscriptionId", skip_serializing)]
    pub subscription_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
}
pub mod event_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Level {
        Critical,
        Error,
        Warning,
        Informational,
        Verbose,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventDataCollection {
    pub value: Vec<EventData>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventCategoryCollection {
    pub value: Vec<LocalizableString>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAvailability {
    #[serde(rename = "timeGrain", skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Unit {
    Count,
    Bytes,
    Seconds,
    CountPerSecond,
    BytesPerSecond,
    Percent,
    MilliSeconds,
    ByteSeconds,
    Unspecified,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDefinition {
    #[serde(rename = "isDimensionRequired", skip_serializing_if = "Option::is_none")]
    pub is_dimension_required: Option<bool>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    #[serde(rename = "primaryAggregationType", skip_serializing_if = "Option::is_none")]
    pub primary_aggregation_type: Option<metric_definition::PrimaryAggregationType>,
    #[serde(rename = "metricAvailabilities", skip_serializing_if = "Vec::is_empty")]
    pub metric_availabilities: Vec<MetricAvailability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<LocalizableString>,
}
pub mod metric_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrimaryAggregationType {
        None,
        Average,
        Count,
        Minimum,
        Maximum,
        Total,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDefinitionCollection {
    pub value: Vec<MetricDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricValue {
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    pub timespan: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    pub value: Vec<Metric>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub name: LocalizableString,
    pub unit: Unit,
    pub timeseries: Vec<TimeSeriesElement>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesElement {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metadatavalues: Vec<MetadataValue>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<MetricValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaselineMetadataValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaselineResponse {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BaselineProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaselineProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timespan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub timestamps: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub baseline: Vec<Baseline>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<BaselineMetadataValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Baseline {
    pub sensitivity: baseline::Sensitivity,
    #[serde(rename = "lowThresholds")]
    pub low_thresholds: Vec<f64>,
    #[serde(rename = "highThresholds")]
    pub high_thresholds: Vec<f64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub timestamps: Vec<String>,
}
pub mod baseline {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Sensitivity {
        Low,
        Medium,
        High,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesInformation {
    pub sensitivities: Vec<String>,
    pub values: Vec<f64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub timestamps: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalculateBaselineResponse {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub timestamps: Vec<String>,
    pub baseline: Vec<Baseline>,
}
