/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// NewUserDetails : The user details.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NewUserDetails {
    /// The URL of the user.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    /// This property is no longer available. See the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// This property is no longer available. See the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This property is no longer available. If the user has an Atlassian account, their password is not changed. If the user does not have an Atlassian account, they are sent an email asking them set up an account.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// The email address for the user.
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    /// A suggested display name for the user. If the user has an Atlassian account, their display name is not changed. If the user does not have an Atlassian account, this display name is used as a suggestion for creating an account. The user is sent an email asking them to set their display name and privacy preferences.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// Deprecated, do not use.
    #[serde(rename = "applicationKeys", skip_serializing_if = "Option::is_none")]
    pub application_keys: Option<Vec<String>>,
}

impl NewUserDetails {
    /// The user details.
    pub fn new(email_address: String, display_name: String) -> NewUserDetails {
        NewUserDetails {
            _self: None,
            key: None,
            name: None,
            password: None,
            email_address,
            display_name,
            application_keys: None,
        }
    }
}
