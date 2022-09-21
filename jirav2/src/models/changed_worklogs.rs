/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ChangedWorklogs : List of changed worklogs.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangedWorklogs {
    /// Changed worklog list.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<crate::models::ChangedWorklog>>,
    /// The datetime of the first worklog item in the list.
    #[serde(rename = "since", skip_serializing_if = "Option::is_none")]
    pub since: Option<i64>,
    /// The datetime of the last worklog item in the list.
    #[serde(rename = "until", skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    /// The URL of this changed worklogs list.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    /// The URL of the next list of changed worklogs.
    #[serde(rename = "nextPage", skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    #[serde(rename = "lastPage", skip_serializing_if = "Option::is_none")]
    pub last_page: Option<bool>,
}

impl ChangedWorklogs {
    /// List of changed worklogs.
    pub fn new() -> ChangedWorklogs {
        ChangedWorklogs {
            values: None,
            since: None,
            until: None,
            _self: None,
            next_page: None,
            last_page: None,
        }
    }
}


