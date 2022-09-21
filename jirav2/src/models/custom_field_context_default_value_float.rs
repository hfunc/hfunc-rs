/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldContextDefaultValueFloat : Default value for a float (number) custom field.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueFloat {
    /// The default floating-point number.
    #[serde(rename = "number")]
    pub number: f64,
    #[serde(rename = "type")]
    pub _type: String,
}

impl CustomFieldContextDefaultValueFloat {
    /// Default value for a float (number) custom field.
    pub fn new(number: f64, _type: String) -> CustomFieldContextDefaultValueFloat {
        CustomFieldContextDefaultValueFloat {
            number,
            _type,
        }
    }
}


