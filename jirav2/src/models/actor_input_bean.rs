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
pub struct ActorInputBean {
    /// The account IDs of the users to add as default actors. This parameter accepts a comma-separated list. For example, `\"user\":[\"5b10a2844c20165700ede21g\", \"5b109f2e9729b51b54dc274d\"]`.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Vec<String>>,
    /// The name of the group to add as a default actor. This parameter accepts a comma-separated list. For example, `\"group\":[\"project-admin\", \"jira-developers\"]`.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Vec<String>>,
}

impl ActorInputBean {
    pub fn new() -> ActorInputBean {
        ActorInputBean {
            user: None,
            group: None,
        }
    }
}


