/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// BulkIssuePropertyUpdateRequest : Bulk issue property update request details.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BulkIssuePropertyUpdateRequest {
    /// The value of the property. The value must be a [valid](https://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    /// EXPERIMENTAL. The Jira expression to calculate the value of the property. The value of the expression must be an object that can be converted to JSON, such as a number, boolean, string, list, or map. The context variables available to the expression are `issue` and `user`. Issues for which the expression returns a value whose JSON representation is longer than 32768 characters are ignored.
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// The bulk operation filter.
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<crate::models::IssueFilterForBulkPropertySet>,
}

impl BulkIssuePropertyUpdateRequest {
    /// Bulk issue property update request details.
    pub fn new() -> BulkIssuePropertyUpdateRequest {
        BulkIssuePropertyUpdateRequest {
            value: None,
            expression: None,
            filter: None,
        }
    }
}
