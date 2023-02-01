/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueTypeToContextMapping : Mapping of an issue type to a context.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueTypeToContextMapping {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The ID of the issue type.
    #[serde(rename = "issueTypeId", skip_serializing_if = "Option::is_none")]
    pub issue_type_id: Option<String>,
    /// Whether the context is mapped to any issue type.
    #[serde(rename = "isAnyIssueType", skip_serializing_if = "Option::is_none")]
    pub is_any_issue_type: Option<bool>,
}

impl IssueTypeToContextMapping {
    /// Mapping of an issue type to a context.
    pub fn new(context_id: String) -> IssueTypeToContextMapping {
        IssueTypeToContextMapping {
            context_id,
            issue_type_id: None,
            is_any_issue_type: None,
        }
    }
}
