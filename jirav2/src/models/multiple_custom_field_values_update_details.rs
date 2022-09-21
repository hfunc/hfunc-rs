/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// MultipleCustomFieldValuesUpdateDetails : List of updates for a custom fields.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MultipleCustomFieldValuesUpdateDetails {
    #[serde(rename = "updates", skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<crate::models::MultipleCustomFieldValuesUpdate>>,
}

impl MultipleCustomFieldValuesUpdateDetails {
    /// List of updates for a custom fields.
    pub fn new() -> MultipleCustomFieldValuesUpdateDetails {
        MultipleCustomFieldValuesUpdateDetails {
            updates: None,
        }
    }
}


