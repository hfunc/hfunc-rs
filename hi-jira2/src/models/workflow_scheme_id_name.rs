/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowSchemeIdName : The ID and the name of the workflow scheme.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowSchemeIdName {
    /// The ID of the workflow scheme.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the workflow scheme.
    #[serde(rename = "name")]
    pub name: String,
}

impl WorkflowSchemeIdName {
    /// The ID and the name of the workflow scheme.
    pub fn new(id: String, name: String) -> WorkflowSchemeIdName {
        WorkflowSchemeIdName { id, name }
    }
}
