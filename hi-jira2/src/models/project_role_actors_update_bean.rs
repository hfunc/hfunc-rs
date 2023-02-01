/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectRoleActorsUpdateBean {
    /// The ID of the project role. Use [Get all project roles](#api-rest-api-2-role-get) to get a list of project role IDs.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The actors to add to the project role. Add groups using `atlassian-group-role-actor` and a list of group names. For example, `\"atlassian-group-role-actor\":[\"another\",\"administrators\"]}`. Add users using `atlassian-user-role-actor` and a list of account IDs. For example, `\"atlassian-user-role-actor\":[\"12345678-9abc-def1-2345-6789abcdef12\", \"abcdef12-3456-789a-bcde-f123456789ab\"]`.
    #[serde(rename = "categorisedActors", skip_serializing_if = "Option::is_none")]
    pub categorised_actors: Option<::std::collections::HashMap<String, Vec<String>>>,
}

impl ProjectRoleActorsUpdateBean {
    pub fn new() -> ProjectRoleActorsUpdateBean {
        ProjectRoleActorsUpdateBean {
            id: None,
            categorised_actors: None,
        }
    }
}
