/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`add_screen_tab`]
#[derive(Clone, Debug, Default)]
pub struct AddScreenTabParams {
    /// The ID of the screen.
    pub screen_id: i64,
    pub screenable_tab: crate::models::ScreenableTab,
}

/// struct for passing parameters to the method [`delete_screen_tab`]
#[derive(Clone, Debug, Default)]
pub struct DeleteScreenTabParams {
    /// The ID of the screen.
    pub screen_id: i64,
    /// The ID of the screen tab.
    pub tab_id: i64,
}

/// struct for passing parameters to the method [`get_all_screen_tabs`]
#[derive(Clone, Debug, Default)]
pub struct GetAllScreenTabsParams {
    /// The ID of the screen.
    pub screen_id: i64,
    /// The key of the project.
    pub project_key: Option<String>,
}

/// struct for passing parameters to the method [`move_screen_tab`]
#[derive(Clone, Debug, Default)]
pub struct MoveScreenTabParams {
    /// The ID of the screen.
    pub screen_id: i64,
    /// The ID of the screen tab.
    pub tab_id: i64,
    /// The position of tab. The base index is 0.
    pub pos: i32,
}

/// struct for passing parameters to the method [`rename_screen_tab`]
#[derive(Clone, Debug, Default)]
pub struct RenameScreenTabParams {
    /// The ID of the screen.
    pub screen_id: i64,
    /// The ID of the screen tab.
    pub tab_id: i64,
    pub screenable_tab: crate::models::ScreenableTab,
}

/// struct for typed errors of method [`add_screen_tab`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddScreenTabError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_screen_tab`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteScreenTabError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_all_screen_tabs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllScreenTabsError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`move_screen_tab`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MoveScreenTabError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`rename_screen_tab`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RenameScreenTabError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// Creates a tab for a screen.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn add_screen_tab(
    configuration: &configuration::Configuration,
    params: AddScreenTabParams,
) -> Result<crate::models::ScreenableTab, Error<AddScreenTabError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let screen_id = params.screen_id;
    let screenable_tab = params.screenable_tab;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/2/screens/{screenId}/tabs",
        local_var_configuration.base_path,
        screenId = screen_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    local_var_req_builder = local_var_req_builder.json(&screenable_tab);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddScreenTabError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a screen tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn delete_screen_tab(
    configuration: &configuration::Configuration,
    params: DeleteScreenTabParams,
) -> Result<(), Error<DeleteScreenTabError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let screen_id = params.screen_id;
    let tab_id = params.tab_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/2/screens/{screenId}/tabs/{tabId}",
        local_var_configuration.base_path,
        screenId = screen_id,
        tabId = tab_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteScreenTabError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the list of tabs for a screen.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).  *  *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) when the project key is specified, providing that the screen is associated with the project through a Screen Scheme and Issue Type Screen Scheme.
pub async fn get_all_screen_tabs(
    configuration: &configuration::Configuration,
    params: GetAllScreenTabsParams,
) -> Result<Vec<crate::models::ScreenableTab>, Error<GetAllScreenTabsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let screen_id = params.screen_id;
    let project_key = params.project_key;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/2/screens/{screenId}/tabs",
        local_var_configuration.base_path,
        screenId = screen_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = project_key {
        local_var_req_builder =
            local_var_req_builder.query(&[("projectKey", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAllScreenTabsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Moves a screen tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn move_screen_tab(
    configuration: &configuration::Configuration,
    params: MoveScreenTabParams,
) -> Result<serde_json::Value, Error<MoveScreenTabError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let screen_id = params.screen_id;
    let tab_id = params.tab_id;
    let pos = params.pos;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/2/screens/{screenId}/tabs/{tabId}/move/{pos}",
        local_var_configuration.base_path,
        screenId = screen_id,
        tabId = tab_id,
        pos = pos
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MoveScreenTabError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the name of a screen tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn rename_screen_tab(
    configuration: &configuration::Configuration,
    params: RenameScreenTabParams,
) -> Result<crate::models::ScreenableTab, Error<RenameScreenTabError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let screen_id = params.screen_id;
    let tab_id = params.tab_id;
    let screenable_tab = params.screenable_tab;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/2/screens/{screenId}/tabs/{tabId}",
        local_var_configuration.base_path,
        screenId = screen_id,
        tabId = tab_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    local_var_req_builder = local_var_req_builder.json(&screenable_tab);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RenameScreenTabError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
