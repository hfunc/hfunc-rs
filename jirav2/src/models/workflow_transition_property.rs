/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowTransitionProperty : Details about the server Jira is running on.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowTransitionProperty {
    /// The key of the transition property. Also known as the name of the transition property.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The value of the transition property.
    #[serde(rename = "value")]
    pub value: String,
    /// The ID of the transition property.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl WorkflowTransitionProperty {
    /// Details about the server Jira is running on.
    pub fn new(value: String) -> WorkflowTransitionProperty {
        WorkflowTransitionProperty {
            key: None,
            value,
            id: None,
        }
    }
}


