/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldValueUpdateDetails : Details of updates for a custom field.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldValueUpdateDetails {
    /// The list of custom field update details.
    #[serde(rename = "updates", skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<crate::models::CustomFieldValueUpdate>>,
}

impl CustomFieldValueUpdateDetails {
    /// Details of updates for a custom field.
    pub fn new() -> CustomFieldValueUpdateDetails {
        CustomFieldValueUpdateDetails { updates: None }
    }
}
