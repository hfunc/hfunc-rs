/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldContextDefaultValueUrl : The default value for a URL custom field.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueUrl {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The default URL.
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "type")]
    pub _type: String,
}

impl CustomFieldContextDefaultValueUrl {
    /// The default value for a URL custom field.
    pub fn new(
        context_id: String,
        url: String,
        _type: String,
    ) -> CustomFieldContextDefaultValueUrl {
        CustomFieldContextDefaultValueUrl {
            context_id,
            url,
            _type,
        }
    }
}
