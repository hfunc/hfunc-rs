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
pub struct ConnectModules {
    /// A list of app modules in the same format as the `modules` property in the [app descriptor](https://developer.atlassian.com/cloud/jira/platform/app-descriptor/).
    #[serde(rename = "modules")]
    pub modules: Vec<serde_json::Value>,
}

impl ConnectModules {
    pub fn new(modules: Vec<serde_json::Value>) -> ConnectModules {
        ConnectModules {
            modules,
        }
    }
}


