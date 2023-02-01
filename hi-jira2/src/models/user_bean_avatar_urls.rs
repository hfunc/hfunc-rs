/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserBeanAvatarUrls {
    /// The URL of the user's 24x24 pixel avatar.
    #[serde(rename = "24x24", skip_serializing_if = "Option::is_none")]
    pub var_24x24: Option<String>,
    /// The URL of the user's 32x32 pixel avatar.
    #[serde(rename = "32x32", skip_serializing_if = "Option::is_none")]
    pub var_32x32: Option<String>,
    /// The URL of the user's 48x48 pixel avatar.
    #[serde(rename = "48x48", skip_serializing_if = "Option::is_none")]
    pub var_48x48: Option<String>,
    /// The URL of the user's 16x16 pixel avatar.
    #[serde(rename = "16x16", skip_serializing_if = "Option::is_none")]
    pub var_16x16: Option<String>,
}

impl UserBeanAvatarUrls {
    pub fn new() -> UserBeanAvatarUrls {
        UserBeanAvatarUrls {
            var_24x24: None,
            var_32x32: None,
            var_48x48: None,
            var_16x16: None,
        }
    }
}
