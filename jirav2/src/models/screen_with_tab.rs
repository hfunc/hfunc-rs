/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ScreenWithTab : A screen with tab details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScreenWithTab {
    /// The ID of the screen.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the screen.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the screen.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The scope of the screen.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<crate::models::Scope>,
    /// The tab for the screen.
    #[serde(rename = "tab", skip_serializing_if = "Option::is_none")]
    pub tab: Option<crate::models::ScreenableTab>,
}

impl ScreenWithTab {
    /// A screen with tab details.
    pub fn new() -> ScreenWithTab {
        ScreenWithTab {
            id: None,
            name: None,
            description: None,
            scope: None,
            tab: None,
        }
    }
}


