#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentFilter {
    #[serde(rename = "principalId", skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "canDelegate", skip_serializing_if = "Option::is_none")]
    pub can_delegate: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentPropertiesWithScope {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "roleDefinitionId", skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[serde(rename = "principalId", skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "principalType", skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<role_assignment_properties_with_scope::PrincipalType>,
    #[serde(rename = "canDelegate", skip_serializing_if = "Option::is_none")]
    pub can_delegate: Option<bool>,
}
pub mod role_assignment_properties_with_scope {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        Unknown,
        DirectoryRoleTemplate,
        ForeignGroup,
        Application,
        #[serde(rename = "MSI")]
        Msi,
        DirectoryObjectOrGroup,
        Everyone,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignment {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleAssignmentPropertiesWithScope>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleAssignment>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentProperties {
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "principalType", skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<role_assignment_properties::PrincipalType>,
    #[serde(rename = "canDelegate", skip_serializing_if = "Option::is_none")]
    pub can_delegate: Option<bool>,
}
pub mod role_assignment_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        Unknown,
        DirectoryRoleTemplate,
        ForeignGroup,
        Application,
        #[serde(rename = "MSI")]
        Msi,
        DirectoryObjectOrGroup,
        Everyone,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentCreateParameters {
    pub properties: RoleAssignmentProperties,
}
