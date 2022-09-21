/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// RemoteObject : The linked item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoteObject {
    /// The URL of the item.
    #[serde(rename = "url")]
    pub url: String,
    /// The title of the item.
    #[serde(rename = "title")]
    pub title: String,
    /// The summary details of the item.
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Details of the icon for the item. If no icon is defined, the default link icon is used in Jira.
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<crate::models::Icon>,
    /// The status of the item.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::Status>,
}

impl RemoteObject {
    /// The linked item.
    pub fn new(url: String, title: String) -> RemoteObject {
        RemoteObject {
            url,
            title,
            summary: None,
            icon: None,
            status: None,
        }
    }
}


