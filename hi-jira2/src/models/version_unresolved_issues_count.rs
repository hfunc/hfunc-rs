/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// VersionUnresolvedIssuesCount : Count of a version's unresolved issues.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VersionUnresolvedIssuesCount {
    /// The URL of these count details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    /// Count of unresolved issues.
    #[serde(
        rename = "issuesUnresolvedCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub issues_unresolved_count: Option<i64>,
    /// Count of issues.
    #[serde(rename = "issuesCount", skip_serializing_if = "Option::is_none")]
    pub issues_count: Option<i64>,
}

impl VersionUnresolvedIssuesCount {
    /// Count of a version's unresolved issues.
    pub fn new() -> VersionUnresolvedIssuesCount {
        VersionUnresolvedIssuesCount {
            _self: None,
            issues_unresolved_count: None,
            issues_count: None,
        }
    }
}
