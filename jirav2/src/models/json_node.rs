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
pub struct JsonNode {
    #[serde(rename = "elements", skip_serializing_if = "Option::is_none")]
    pub elements: Option<serde_json::Value>,
    #[serde(rename = "floatingPointNumber", skip_serializing_if = "Option::is_none")]
    pub floating_point_number: Option<bool>,
    #[serde(rename = "pojo", skip_serializing_if = "Option::is_none")]
    pub pojo: Option<bool>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<bool>,
    #[serde(rename = "integralNumber", skip_serializing_if = "Option::is_none")]
    pub integral_number: Option<bool>,
    #[serde(rename = "int", skip_serializing_if = "Option::is_none")]
    pub int: Option<bool>,
    #[serde(rename = "long", skip_serializing_if = "Option::is_none")]
    pub long: Option<bool>,
    #[serde(rename = "double", skip_serializing_if = "Option::is_none")]
    pub double: Option<bool>,
    #[serde(rename = "bigDecimal", skip_serializing_if = "Option::is_none")]
    pub big_decimal: Option<bool>,
    #[serde(rename = "bigInteger", skip_serializing_if = "Option::is_none")]
    pub big_integer: Option<bool>,
    #[serde(rename = "textual", skip_serializing_if = "Option::is_none")]
    pub textual: Option<bool>,
    #[serde(rename = "boolean", skip_serializing_if = "Option::is_none")]
    pub boolean: Option<bool>,
    #[serde(rename = "binary", skip_serializing_if = "Option::is_none")]
    pub binary: Option<bool>,
    #[serde(rename = "containerNode", skip_serializing_if = "Option::is_none")]
    pub container_node: Option<bool>,
    #[serde(rename = "missingNode", skip_serializing_if = "Option::is_none")]
    pub missing_node: Option<bool>,
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<bool>,
    #[serde(rename = "valueNode", skip_serializing_if = "Option::is_none")]
    pub value_node: Option<bool>,
    #[serde(rename = "numberValue", skip_serializing_if = "Option::is_none")]
    pub number_value: Option<f32>,
    #[serde(rename = "numberType", skip_serializing_if = "Option::is_none")]
    pub number_type: Option<NumberType>,
    #[serde(rename = "intValue", skip_serializing_if = "Option::is_none")]
    pub int_value: Option<i32>,
    #[serde(rename = "longValue", skip_serializing_if = "Option::is_none")]
    pub long_value: Option<i64>,
    #[serde(rename = "bigIntegerValue", skip_serializing_if = "Option::is_none")]
    pub big_integer_value: Option<i32>,
    #[serde(rename = "doubleValue", skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    #[serde(rename = "decimalValue", skip_serializing_if = "Option::is_none")]
    pub decimal_value: Option<f32>,
    #[serde(rename = "booleanValue", skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<bool>,
    #[serde(rename = "binaryValue", skip_serializing_if = "Option::is_none")]
    pub binary_value: Option<Vec<String>>,
    #[serde(rename = "valueAsInt", skip_serializing_if = "Option::is_none")]
    pub value_as_int: Option<i32>,
    #[serde(rename = "valueAsLong", skip_serializing_if = "Option::is_none")]
    pub value_as_long: Option<i64>,
    #[serde(rename = "valueAsDouble", skip_serializing_if = "Option::is_none")]
    pub value_as_double: Option<f64>,
    #[serde(rename = "valueAsBoolean", skip_serializing_if = "Option::is_none")]
    pub value_as_boolean: Option<bool>,
    #[serde(rename = "fieldNames", skip_serializing_if = "Option::is_none")]
    pub field_names: Option<serde_json::Value>,
    #[serde(rename = "textValue", skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    #[serde(rename = "valueAsText", skip_serializing_if = "Option::is_none")]
    pub value_as_text: Option<String>,
    #[serde(rename = "array", skip_serializing_if = "Option::is_none")]
    pub array: Option<bool>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
    #[serde(rename = "null", skip_serializing_if = "Option::is_none")]
    pub null: Option<bool>,
}

impl JsonNode {
    pub fn new() -> JsonNode {
        JsonNode {
            elements: None,
            floating_point_number: None,
            pojo: None,
            number: None,
            integral_number: None,
            int: None,
            long: None,
            double: None,
            big_decimal: None,
            big_integer: None,
            textual: None,
            boolean: None,
            binary: None,
            container_node: None,
            missing_node: None,
            object: None,
            value_node: None,
            number_value: None,
            number_type: None,
            int_value: None,
            long_value: None,
            big_integer_value: None,
            double_value: None,
            decimal_value: None,
            boolean_value: None,
            binary_value: None,
            value_as_int: None,
            value_as_long: None,
            value_as_double: None,
            value_as_boolean: None,
            field_names: None,
            text_value: None,
            value_as_text: None,
            array: None,
            fields: None,
            null: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NumberType {
    #[serde(rename = "INT")]
    INT,
    #[serde(rename = "LONG")]
    LONG,
    #[serde(rename = "BIG_INTEGER")]
    BIGINTEGER,
    #[serde(rename = "FLOAT")]
    FLOAT,
    #[serde(rename = "DOUBLE")]
    DOUBLE,
    #[serde(rename = "BIG_DECIMAL")]
    BIGDECIMAL,
}

impl Default for NumberType {
    fn default() -> NumberType {
        Self::INT
    }
}

