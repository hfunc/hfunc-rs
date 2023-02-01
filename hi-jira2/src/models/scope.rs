/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Scope : The projects the item is associated with. Indicated for items associated with [next-gen projects](https://confluence.atlassian.com/x/loMyO).

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Scope {
    /// The type of scope.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// The project the item has scope in.
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<crate::models::ProjectDetails>,
}

impl Scope {
    /// The projects the item is associated with. Indicated for items associated with [next-gen projects](https://confluence.atlassian.com/x/loMyO).
    pub fn new() -> Scope {
        Scope {
            _type: None,
            project: None,
        }
    }
}

/// The type of scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "PROJECT")]
    PROJECT,
    #[serde(rename = "TEMPLATE")]
    TEMPLATE,
}

impl Default for Type {
    fn default() -> Type {
        Self::PROJECT
    }
}
