/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldContextDefaultValueLabels : Default value for a labels custom field.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueLabels {
    /// The default labels value.
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    #[serde(rename = "type")]
    pub _type: String,
}

impl CustomFieldContextDefaultValueLabels {
    /// Default value for a labels custom field.
    pub fn new(labels: Vec<String>, _type: String) -> CustomFieldContextDefaultValueLabels {
        CustomFieldContextDefaultValueLabels {
            labels,
            _type,
        }
    }
}


