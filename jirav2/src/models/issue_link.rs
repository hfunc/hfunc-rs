/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueLink : Details of a link between issues.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueLink {
    /// The ID of the issue link.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The URL of the issue link.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    /// The type of link between the issues.
    #[serde(rename = "type")]
    pub _type: Option<crate::models::IssueLinkType>,
    /// Provides details about the linked issue. If presenting this link in a user interface, use the `inward` field of the issue link type to label the link.
    #[serde(rename = "inwardIssue")]
    pub inward_issue: Option<crate::models::LinkedIssue>,
    /// Provides details about the linked issue. If presenting this link in a user interface, use the `outward` field of the issue link type to label the link.
    #[serde(rename = "outwardIssue")]
    pub outward_issue: Option<crate::models::LinkedIssue>,
}

impl IssueLink {
    /// Details of a link between issues.
    pub fn new(_type: Option<crate::models::IssueLinkType>, inward_issue: Option<crate::models::LinkedIssue>, outward_issue: Option<crate::models::LinkedIssue>) -> IssueLink {
        IssueLink {
            id: None,
            _self: None,
            _type: _type,
            inward_issue: inward_issue,
            outward_issue: outward_issue,
        }
    }
}


