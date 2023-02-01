/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ContextForProjectAndIssueType : The project and issue type mapping with a matching custom field context.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContextForProjectAndIssueType {
    /// The ID of the project.
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// The ID of the issue type.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
    /// The ID of the custom field context.
    #[serde(rename = "contextId")]
    pub context_id: String,
}

impl ContextForProjectAndIssueType {
    /// The project and issue type mapping with a matching custom field context.
    pub fn new(
        project_id: String,
        issue_type_id: String,
        context_id: String,
    ) -> ContextForProjectAndIssueType {
        ContextForProjectAndIssueType {
            project_id,
            issue_type_id,
            context_id,
        }
    }
}
