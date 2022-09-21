/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueTypeScreenSchemeMappingDetails : A list of issue type screen scheme mappings.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueTypeScreenSchemeMappingDetails {
    /// The list of issue type to screen scheme mappings. A *default* entry cannot be specified because a default entry is added when an issue type screen scheme is created.
    #[serde(rename = "issueTypeMappings")]
    pub issue_type_mappings: Vec<crate::models::IssueTypeScreenSchemeMapping>,
}

impl IssueTypeScreenSchemeMappingDetails {
    /// A list of issue type screen scheme mappings.
    pub fn new(issue_type_mappings: Vec<crate::models::IssueTypeScreenSchemeMapping>) -> IssueTypeScreenSchemeMappingDetails {
        IssueTypeScreenSchemeMappingDetails {
            issue_type_mappings,
        }
    }
}


