/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// PermissionHolder : Details of a user, group, field, or project role that holds a permission. See [Holder object](../api-group-permission-schemes/#holder-object) in *Get all permission schemes* for more information.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PermissionHolder {
    /// The type of permission holder.
    #[serde(rename = "type")]
    pub _type: String,
    /// The identifier of permission holder.
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    /// Expand options that include additional permission holder details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
}

impl PermissionHolder {
    /// Details of a user, group, field, or project role that holds a permission. See [Holder object](../api-group-permission-schemes/#holder-object) in *Get all permission schemes* for more information.
    pub fn new(_type: String) -> PermissionHolder {
        PermissionHolder {
            _type,
            parameter: None,
            expand: None,
        }
    }
}
