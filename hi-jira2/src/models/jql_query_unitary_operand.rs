/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JqlQueryUnitaryOperand : An operand that can be part of a list operand.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JqlQueryUnitaryOperand {
    /// The operand value.
    #[serde(rename = "value")]
    pub value: String,
    /// The name of the function.
    #[serde(rename = "function")]
    pub function: String,
    /// The list of function arguments.
    #[serde(rename = "arguments")]
    pub arguments: Vec<String>,
    /// The keyword that is the operand value.
    #[serde(rename = "keyword")]
    pub keyword: Keyword,
}

impl JqlQueryUnitaryOperand {
    /// An operand that can be part of a list operand.
    pub fn new(
        value: String,
        function: String,
        arguments: Vec<String>,
        keyword: Keyword,
    ) -> JqlQueryUnitaryOperand {
        JqlQueryUnitaryOperand {
            value,
            function,
            arguments,
            keyword,
        }
    }
}

/// The keyword that is the operand value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Keyword {
    #[serde(rename = "empty")]
    Empty,
}

impl Default for Keyword {
    fn default() -> Keyword {
        Self::Empty
    }
}
