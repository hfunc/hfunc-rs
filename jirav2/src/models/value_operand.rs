/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ValueOperand : An operand that is a user-provided value.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValueOperand {
    /// The operand value.
    #[serde(rename = "value")]
    pub value: String,
}

impl ValueOperand {
    /// An operand that is a user-provided value.
    pub fn new(value: String) -> ValueOperand {
        ValueOperand {
            value,
        }
    }
}


