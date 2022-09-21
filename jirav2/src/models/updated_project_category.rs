/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// UpdatedProjectCategory : A project category.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdatedProjectCategory {
    /// The URL of the project category.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    /// The ID of the project category.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the project category.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The description of the project category.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdatedProjectCategory {
    /// A project category.
    pub fn new() -> UpdatedProjectCategory {
        UpdatedProjectCategory {
            _self: None,
            id: None,
            description: None,
            name: None,
        }
    }
}


