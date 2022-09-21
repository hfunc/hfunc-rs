/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ScreenSchemeDetails : Details of a screen scheme.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScreenSchemeDetails {
    /// The name of the screen scheme. The name must be unique. The maximum length is 255 characters.
    #[serde(rename = "name")]
    pub name: String,
    /// The description of the screen scheme. The maximum length is 255 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The IDs of the screens for the screen types of the screen scheme. Only screens used in classic projects are accepted.
    #[serde(rename = "screens")]
    pub screens: Option<crate::models::ScreenTypes>,
}

impl ScreenSchemeDetails {
    /// Details of a screen scheme.
    pub fn new(name: String, screens: Option<crate::models::ScreenTypes>) -> ScreenSchemeDetails {
        ScreenSchemeDetails {
            name,
            description: None,
            screens: screens,
        }
    }
}


