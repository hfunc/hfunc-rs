/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// AssociateFieldConfigurationsWithIssueTypesRequest : Details of a field configuration to issue type mappings.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssociateFieldConfigurationsWithIssueTypesRequest {
    /// Field configuration to issue type mappings.
    #[serde(rename = "mappings")]
    pub mappings: Vec<crate::models::FieldConfigurationToIssueTypeMapping>,
}

impl AssociateFieldConfigurationsWithIssueTypesRequest {
    /// Details of a field configuration to issue type mappings.
    pub fn new(mappings: Vec<crate::models::FieldConfigurationToIssueTypeMapping>) -> AssociateFieldConfigurationsWithIssueTypesRequest {
        AssociateFieldConfigurationsWithIssueTypesRequest {
            mappings,
        }
    }
}


