/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JqlQueryWithUnknownUsers : JQL queries that contained users that could not be found

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JqlQueryWithUnknownUsers {
    /// The original query, for reference
    #[serde(rename = "originalQuery", skip_serializing_if = "Option::is_none")]
    pub original_query: Option<String>,
    /// The converted query, with accountIDs instead of user identifiers, or 'unknown' for users that could not be found
    #[serde(rename = "convertedQuery", skip_serializing_if = "Option::is_none")]
    pub converted_query: Option<String>,
}

impl JqlQueryWithUnknownUsers {
    /// JQL queries that contained users that could not be found
    pub fn new() -> JqlQueryWithUnknownUsers {
        JqlQueryWithUnknownUsers {
            original_query: None,
            converted_query: None,
        }
    }
}
