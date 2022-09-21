/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowTransitionRule : A workflow transition rule.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowTransitionRule {
    /// The type of the transition rule.
    #[serde(rename = "type")]
    pub _type: String,
    /// EXPERIMENTAL. The configuration of the transition rule.
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
}

impl WorkflowTransitionRule {
    /// A workflow transition rule.
    pub fn new(_type: String) -> WorkflowTransitionRule {
        WorkflowTransitionRule {
            _type,
            configuration: None,
        }
    }
}


