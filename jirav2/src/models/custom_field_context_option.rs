/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldContextOption : Details of the custom field options for a context.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldContextOption {
    /// The ID of the custom field option.
    #[serde(rename = "id")]
    pub id: String,
    /// The value of the custom field option.
    #[serde(rename = "value")]
    pub value: String,
    /// For cascading options, the ID of the custom field option containing the cascading option.
    #[serde(rename = "optionId", skip_serializing_if = "Option::is_none")]
    pub option_id: Option<String>,
    /// Whether the option is disabled.
    #[serde(rename = "disabled")]
    pub disabled: bool,
}

impl CustomFieldContextOption {
    /// Details of the custom field options for a context.
    pub fn new(id: String, value: String, disabled: bool) -> CustomFieldContextOption {
        CustomFieldContextOption {
            id,
            value,
            option_id: None,
            disabled,
        }
    }
}


