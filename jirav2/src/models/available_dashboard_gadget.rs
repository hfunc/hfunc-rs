/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// AvailableDashboardGadget : The details of the available dashboard gadget.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AvailableDashboardGadget {
    /// The module key of the gadget type.
    #[serde(rename = "moduleKey", skip_serializing_if = "Option::is_none")]
    pub module_key: Option<String>,
    /// The URI of the gadget type.
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// The title of the gadget.
    #[serde(rename = "title")]
    pub title: String,
}

impl AvailableDashboardGadget {
    /// The details of the available dashboard gadget.
    pub fn new(title: String) -> AvailableDashboardGadget {
        AvailableDashboardGadget {
            module_key: None,
            uri: None,
            title,
        }
    }
}


