/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldContextDefaultValueMultipleGroupPicker : The default value for a multiple group picker custom field.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueMultipleGroupPicker {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The IDs of the default groups.
    #[serde(rename = "groupIds")]
    pub group_ids: Vec<String>,
    #[serde(rename = "type")]
    pub _type: String,
}

impl CustomFieldContextDefaultValueMultipleGroupPicker {
    /// The default value for a multiple group picker custom field.
    pub fn new(context_id: String, group_ids: Vec<String>, _type: String) -> CustomFieldContextDefaultValueMultipleGroupPicker {
        CustomFieldContextDefaultValueMultipleGroupPicker {
            context_id,
            group_ids,
            _type,
        }
    }
}


