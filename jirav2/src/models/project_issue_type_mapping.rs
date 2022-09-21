/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectIssueTypeMapping : The project and issue type mapping.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectIssueTypeMapping {
    /// The ID of the project.
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// The ID of the issue type.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
}

impl ProjectIssueTypeMapping {
    /// The project and issue type mapping.
    pub fn new(project_id: String, issue_type_id: String) -> ProjectIssueTypeMapping {
        ProjectIssueTypeMapping {
            project_id,
            issue_type_id,
        }
    }
}


