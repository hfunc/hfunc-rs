/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WebhookRegistrationDetails : Details of webhooks to register.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WebhookRegistrationDetails {
    /// A list of webhooks.
    #[serde(rename = "webhooks")]
    pub webhooks: Vec<crate::models::WebhookDetails>,
    /// The URL that specifies where to send the webhooks. This URL must use the same base URL as the Connect app.
    #[serde(rename = "url")]
    pub url: String,
}

impl WebhookRegistrationDetails {
    /// Details of webhooks to register.
    pub fn new(
        webhooks: Vec<crate::models::WebhookDetails>,
        url: String,
    ) -> WebhookRegistrationDetails {
        WebhookRegistrationDetails { webhooks, url }
    }
}
