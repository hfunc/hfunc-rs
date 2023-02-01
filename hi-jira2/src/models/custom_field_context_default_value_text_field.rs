/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldContextDefaultValueTextField : The default text for a text custom field.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueTextField {
    /// The default text. The maximum length is 254 characters.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
}

impl CustomFieldContextDefaultValueTextField {
    /// The default text for a text custom field.
    pub fn new(_type: String) -> CustomFieldContextDefaultValueTextField {
        CustomFieldContextDefaultValueTextField { text: None, _type }
    }
}
