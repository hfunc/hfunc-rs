/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// FieldReferenceData : Details of a field that can be used in advanced searches.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldReferenceData {
    /// The field identifier.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// The display name contains the following:   *  for system fields, the field name. For example, `Summary`.  *  for collapsed custom fields, the field name followed by a hyphen and then the field name and field type. For example, `Component - Component[Dropdown]`.  *  for other custom fields, the field name followed by a hyphen and then the custom field ID. For example, `Component - cf[10061]`.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Whether the field can be used in a query's `ORDER BY` clause.
    #[serde(rename = "orderable", skip_serializing_if = "Option::is_none")]
    pub orderable: Option<Orderable>,
    /// Whether the content of this field can be searched.
    #[serde(rename = "searchable", skip_serializing_if = "Option::is_none")]
    pub searchable: Option<Searchable>,
    /// Whether the field provide auto-complete suggestions.
    #[serde(rename = "auto", skip_serializing_if = "Option::is_none")]
    pub auto: Option<Auto>,
    /// If the item is a custom field, the ID of the custom field.
    #[serde(rename = "cfid", skip_serializing_if = "Option::is_none")]
    pub cfid: Option<String>,
    /// The valid search operators for the field.
    #[serde(rename = "operators", skip_serializing_if = "Option::is_none")]
    pub operators: Option<Vec<String>>,
    /// The data types of items in the field.
    #[serde(rename = "types", skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

impl FieldReferenceData {
    /// Details of a field that can be used in advanced searches.
    pub fn new() -> FieldReferenceData {
        FieldReferenceData {
            value: None,
            display_name: None,
            orderable: None,
            searchable: None,
            auto: None,
            cfid: None,
            operators: None,
            types: None,
        }
    }
}

/// Whether the field can be used in a query's `ORDER BY` clause.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Orderable {
    #[serde(rename = "true")]
    _True,
    #[serde(rename = "false")]
    _False,
}

impl Default for Orderable {
    fn default() -> Orderable {
        Self::_True
    }
}
/// Whether the content of this field can be searched.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Searchable {
    #[serde(rename = "true")]
    _True,
    #[serde(rename = "false")]
    _False,
}

impl Default for Searchable {
    fn default() -> Searchable {
        Self::_True
    }
}
/// Whether the field provide auto-complete suggestions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Auto {
    #[serde(rename = "true")]
    _True,
    #[serde(rename = "false")]
    _False,
}

impl Default for Auto {
    fn default() -> Auto {
        Self::_True
    }
}

