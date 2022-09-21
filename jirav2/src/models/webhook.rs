/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Webhook : A webhook.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Webhook {
    /// The ID of the webhook.
    #[serde(rename = "id")]
    pub id: i64,
    /// The JQL filter that specifies which issues the webhook is sent for.
    #[serde(rename = "jqlFilter")]
    pub jql_filter: String,
    /// A list of field IDs. When the issue changelog contains any of the fields, the webhook `jira:issue_updated` is sent. If this parameter is not present, the app is notified about all field updates.
    #[serde(rename = "fieldIdsFilter", skip_serializing_if = "Option::is_none")]
    pub field_ids_filter: Option<Vec<String>>,
    /// A list of issue property keys. A change of those issue properties triggers the `issue_property_set` or `issue_property_deleted` webhooks. If this parameter is not present, the app is notified about all issue property updates.
    #[serde(rename = "issuePropertyKeysFilter", skip_serializing_if = "Option::is_none")]
    pub issue_property_keys_filter: Option<Vec<String>>,
    /// The Jira events that trigger the webhook.
    #[serde(rename = "events")]
    pub events: Vec<Events>,
    /// The date after which the webhook is no longer sent. Use [Extend webhook life](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-webhooks/#api-rest-api-3-webhook-refresh-put) to extend the date.
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<i64>,
}

impl Webhook {
    /// A webhook.
    pub fn new(id: i64, jql_filter: String, events: Vec<Events>) -> Webhook {
        Webhook {
            id,
            jql_filter,
            field_ids_filter: None,
            issue_property_keys_filter: None,
            events,
            expiration_date: None,
        }
    }
}

/// The Jira events that trigger the webhook.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Events {
    #[serde(rename = "jira:issue_created")]
    JiraissueCreated,
    #[serde(rename = "jira:issue_updated")]
    JiraissueUpdated,
    #[serde(rename = "jira:issue_deleted")]
    JiraissueDeleted,
    #[serde(rename = "comment_created")]
    CommentCreated,
    #[serde(rename = "comment_updated")]
    CommentUpdated,
    #[serde(rename = "comment_deleted")]
    CommentDeleted,
    #[serde(rename = "issue_property_set")]
    IssuePropertySet,
    #[serde(rename = "issue_property_deleted")]
    IssuePropertyDeleted,
}

impl Default for Events {
    fn default() -> Events {
        Self::JiraissueCreated
    }
}

