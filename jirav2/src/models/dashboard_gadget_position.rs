/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// DashboardGadgetPosition : Details of a gadget position.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DashboardGadgetPosition {
    #[serde(rename = "The row position of the gadget.")]
    pub the_row_position_of_the_gadget_: i32,
    #[serde(rename = "The column position of the gadget.")]
    pub the_column_position_of_the_gadget_: i32,
}

impl DashboardGadgetPosition {
    /// Details of a gadget position.
    pub fn new(the_row_position_of_the_gadget_: i32, the_column_position_of_the_gadget_: i32) -> DashboardGadgetPosition {
        DashboardGadgetPosition {
            the_row_position_of_the_gadget_,
            the_column_position_of_the_gadget_,
        }
    }
}


