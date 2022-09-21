# JiraExpressionsComplexityBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**steps** | Option<[**crate::models::JiraExpressionsComplexityValueBean**](JiraExpressionsComplexityValueBean.md)> | The number of steps it took to evaluate the expression, where a step is a high-level operation performed by the expression. A step is an operation such as arithmetic, accessing a property, accessing a context variable, or calling a function. | 
**expensive_operations** | Option<[**crate::models::JiraExpressionsComplexityValueBean**](JiraExpressionsComplexityValueBean.md)> | The number of expensive operations executed while evaluating the expression. Expensive operations are those that load additional data, such as entity properties, comments, or custom fields. | 
**beans** | Option<[**crate::models::JiraExpressionsComplexityValueBean**](JiraExpressionsComplexityValueBean.md)> | The number of Jira REST API beans returned in the response. | 
**primitive_values** | Option<[**crate::models::JiraExpressionsComplexityValueBean**](JiraExpressionsComplexityValueBean.md)> | The number of primitive values returned in the response. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


