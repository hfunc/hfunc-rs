/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// SecuritySchemes : List of security schemes.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecuritySchemes {
    /// List of security schemes.
    #[serde(
        rename = "issueSecuritySchemes",
        skip_serializing_if = "Option::is_none"
    )]
    pub issue_security_schemes: Option<Vec<crate::models::SecurityScheme>>,
}

impl SecuritySchemes {
    /// List of security schemes.
    pub fn new() -> SecuritySchemes {
        SecuritySchemes {
            issue_security_schemes: None,
        }
    }
}
