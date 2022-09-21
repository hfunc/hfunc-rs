/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// AutoCompleteSuggestion : A field auto-complete suggestion.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AutoCompleteSuggestion {
    /// The value of a suggested item.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// The display name of a suggested item. If `fieldValue` or `predicateValue` are provided, the matching text is highlighted with the HTML bold tag.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl AutoCompleteSuggestion {
    /// A field auto-complete suggestion.
    pub fn new() -> AutoCompleteSuggestion {
        AutoCompleteSuggestion {
            value: None,
            display_name: None,
        }
    }
}


