/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueTypeIds : The list of issue type IDs.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueTypeIds {
    /// The list of issue type IDs.
    #[serde(rename = "issueTypeIds")]
    pub issue_type_ids: Vec<String>,
}

impl IssueTypeIds {
    /// The list of issue type IDs.
    pub fn new(issue_type_ids: Vec<String>) -> IssueTypeIds {
        IssueTypeIds { issue_type_ids }
    }
}
