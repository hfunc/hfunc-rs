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
pub struct IssueFieldOptionCreateBean {
    /// The option's name, which is displayed in Jira.
    #[serde(rename = "value")]
    pub value: String,
    /// The properties of the option as arbitrary key-value pairs. These properties can be searched using JQL, if the extractions (see https://developer.atlassian.com/cloud/jira/platform/modules/issue-field-option-property-index/) are defined in the descriptor for the issue field module.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<crate::models::IssueFieldOptionConfiguration>,
}

impl IssueFieldOptionCreateBean {
    pub fn new(value: String) -> IssueFieldOptionCreateBean {
        IssueFieldOptionCreateBean {
            value,
            properties: None,
            config: None,
        }
    }
}


