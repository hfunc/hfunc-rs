/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// AvailableDashboardGadgetsResponse : The list of available gadgets.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AvailableDashboardGadgetsResponse {
    /// The list of available gadgets.
    #[serde(rename = "gadgets")]
    pub gadgets: Vec<crate::models::AvailableDashboardGadget>,
}

impl AvailableDashboardGadgetsResponse {
    /// The list of available gadgets.
    pub fn new(
        gadgets: Vec<crate::models::AvailableDashboardGadget>,
    ) -> AvailableDashboardGadgetsResponse {
        AvailableDashboardGadgetsResponse { gadgets }
    }
}
