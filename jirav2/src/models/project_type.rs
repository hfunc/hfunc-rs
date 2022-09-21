/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectType : Details about a project type.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectType {
    /// The key of the project type.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The formatted key of the project type.
    #[serde(rename = "formattedKey", skip_serializing_if = "Option::is_none")]
    pub formatted_key: Option<String>,
    /// The key of the project type's description.
    #[serde(rename = "descriptionI18nKey", skip_serializing_if = "Option::is_none")]
    pub description_i18n_key: Option<String>,
    /// The icon of the project type.
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// The color of the project type.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl ProjectType {
    /// Details about a project type.
    pub fn new() -> ProjectType {
        ProjectType {
            key: None,
            formatted_key: None,
            description_i18n_key: None,
            icon: None,
            color: None,
        }
    }
}


