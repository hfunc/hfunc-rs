/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateWorkflowDetails : The details of a workflow.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateWorkflowDetails {
    /// The name of the workflow. The name must be unique. The maximum length is 255 characters. Characters can be separated by a whitespace but the name cannot start or end with a whitespace.
    #[serde(rename = "name")]
    pub name: String,
    /// The description of the workflow. The maximum length is 1000 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The transitions of the workflow. For the request to be valid, these transitions must:   *  include one *initial* transition.  *  not use the same name for a *global* and *directed* transition.  *  have a unique name for each *global* transition.  *  have a unique 'to' status for each *global* transition.  *  have unique names for each transition from a status.  *  not have a 'from' status on *initial* and *global* transitions.  *  have a 'from' status on *directed* transitions.  All the transition statuses must be included in `statuses`.
    #[serde(rename = "transitions")]
    pub transitions: Vec<crate::models::CreateWorkflowTransitionDetails>,
    /// The statuses of the workflow. Any status that does not include a transition is added to the workflow without a transition.
    #[serde(rename = "statuses")]
    pub statuses: Vec<crate::models::CreateWorkflowStatusDetails>,
}

impl CreateWorkflowDetails {
    /// The details of a workflow.
    pub fn new(name: String, transitions: Vec<crate::models::CreateWorkflowTransitionDetails>, statuses: Vec<crate::models::CreateWorkflowStatusDetails>) -> CreateWorkflowDetails {
        CreateWorkflowDetails {
            name,
            description: None,
            transitions,
            statuses,
        }
    }
}


