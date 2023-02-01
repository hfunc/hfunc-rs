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

/// struct for passing parameters to the method [`add_user_to_group`]
#[derive(Clone, Debug, Default)]
pub struct AddUserToGroupParams {
    /// The user to add to the group.
    pub request_body: ::std::collections::HashMap<String, serde_json::Value>,
    /// As a group's name can change, use of `groupId` is recommended to identify a group.   The name of the group. This parameter cannot be used with the `groupId` parameter.
    pub groupname: Option<String>,
    /// The ID of the group. This parameter cannot be used with the `groupName` parameter.
    pub group_id: Option<String>,
}

/// struct for passing parameters to the method [`bulk_get_groups`]
#[derive(Clone, Debug, Default)]
pub struct BulkGetGroupsParams {
    /// The index of the first item to return in a page of results (page offset).
    pub start_at: Option<i64>,
    /// The maximum number of items to return per page.
    pub max_results: Option<i32>,
    /// The ID of a group. To specify multiple IDs, pass multiple `groupId` parameters. For example, `groupId=5b10a2844c20165700ede21g&groupId=5b10ac8d82e05b22cc7d4ef5`.
    pub group_id: Option<Vec<String>>,
    /// The name of a group. To specify multiple names, pass multiple `groupName` parameters. For example, `groupName=administrators&groupName=jira-software-users`.
    pub group_name: Option<Vec<String>>,
}

/// struct for passing parameters to the method [`create_group`]
#[derive(Clone, Debug, Default)]
pub struct CreateGroupParams {
    /// The name of the group.
    pub request_body: ::std::collections::HashMap<String, serde_json::Value>,
}

/// struct for passing parameters to the method [`find_groups`]
#[derive(Clone, Debug, Default)]
pub struct FindGroupsParams {
    /// This parameter is deprecated, setting it does not affect the results. To find groups containing a particular user, use [Get user groups](#api-rest-api-2-user-groups-get).
    pub account_id: Option<String>,
    /// The string to find in group names.
    pub query: Option<String>,
    /// As a group's name can change, use of `excludeGroupIds` is recommended to identify a group.   A group to exclude from the result. To exclude multiple groups, provide an ampersand-separated list. For example, `exclude=group1&exclude=group2`. This parameter cannot be used with the `excludeGroupIds` parameter.
    pub exclude: Option<Vec<String>>,
    /// A group ID to exclude from the result. To exclude multiple groups, provide an ampersand-separated list. For example, `excludeId=group1-id&excludeId=group2-id`. This parameter cannot be used with the `excludeGroups` parameter.
    pub exclude_id: Option<Vec<String>>,
    /// The maximum number of groups to return. The maximum number of groups that can be returned is limited by the system property `jira.ajax.autocomplete.limit`.
    pub max_results: Option<i32>,
    /// This parameter is no longer available. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details.
    pub user_name: Option<String>,
}

/// struct for passing parameters to the method [`get_group`]
#[derive(Clone, Debug, Default)]
pub struct GetGroupParams {
    /// As a group's name can change, use of `groupId` is recommended to identify a group.   The name of the group. This parameter cannot be used with the `groupId` parameter.
    pub groupname: Option<String>,
    /// The ID of the group. This parameter cannot be used with the `groupName` parameter.
    pub group_id: Option<String>,
    /// List of fields to expand.
    pub expand: Option<String>,
}

/// struct for passing parameters to the method [`get_users_from_group`]
#[derive(Clone, Debug, Default)]
pub struct GetUsersFromGroupParams {
    /// As a group's name can change, use of `groupId` is recommended to identify a group.   The name of the group. This parameter cannot be used with the `groupId` parameter.
    pub groupname: Option<String>,
    /// The ID of the group. This parameter cannot be used with the `groupName` parameter.
    pub group_id: Option<String>,
    /// Include inactive users.
    pub include_inactive_users: Option<bool>,
    /// The index of the first item to return in a page of results (page offset).
    pub start_at: Option<i64>,
    /// The maximum number of items to return per page.
    pub max_results: Option<i32>,
}

/// struct for passing parameters to the method [`remove_group`]
#[derive(Clone, Debug, Default)]
pub struct RemoveGroupParams {
    /// As a group's name can change, use of `groupId` is recommended to identify a group.   The name of the group. This parameter cannot be used with the `groupId` parameter.
    pub groupname: Option<String>,
    /// The ID of the group. This parameter cannot be used with the `groupId` parameter. This parameter cannot be used with the `groupName` parameter.
    pub group_id: Option<String>,
    /// As a group's name can change, use of `swapGroupId` is recommended to identify a group.   The group to transfer restrictions to. Only comments and worklogs are transferred. If restrictions are not transferred, comments and worklogs are inaccessible after the deletion. This parameter cannot be used with the `swapGroupId` parameter.
    pub swap_group: Option<String>,
    /// The ID of the group to transfer restrictions to. Only comments and worklogs are transferred. If restrictions are not transferred, comments and worklogs are inaccessible after the deletion. This parameter cannot be used with the `swapGroup` parameter.
    pub swap_group_id: Option<String>,
}

/// struct for passing parameters to the method [`remove_user_from_group`]
#[derive(Clone, Debug, Default)]
pub struct RemoveUserFromGroupParams {
    /// The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*.
    pub account_id: String,
    /// As a group's name can change, use of `groupId` is recommended to identify a group.   The name of the group. This parameter cannot be used with the `groupId` parameter.
    pub groupname: Option<String>,
    /// The ID of the group. This parameter cannot be used with the `groupName` parameter.
    pub group_id: Option<String>,
    /// This parameter is no longer available. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details.
    pub username: Option<String>,
}

/// struct for typed errors of method [`add_user_to_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddUserToGroupError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_get_groups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkGetGroupsError {
    Status400(),
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGroupError {
    Status400(),
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`find_groups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindGroupsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGroupError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_users_from_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsersFromGroupError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveGroupError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_user_from_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveUserFromGroupError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// Adds a user to a group.  **[Permissions](#permissions) required:** Site administration (that is, member of the *site-admin* [group](https://confluence.atlassian.com/x/24xjL)).
pub async fn add_user_to_group(
    configuration: &configuration::Configuration,
    params: AddUserToGroupParams,
) -> Result<crate::models::Group, Error<AddUserToGroupError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let request_body = params.request_body;
    let groupname = params.groupname;
    let group_id = params.group_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/2/group/user",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = groupname {
        local_var_req_builder =
            local_var_req_builder.query(&[("groupname", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = group_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("groupId", &local_var_str.to_string())]);
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
    local_var_req_builder = local_var_req_builder.json(&request_body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddUserToGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a [paginated](#pagination) list of groups.  **[Permissions](#permissions) required:** *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn bulk_get_groups(
    configuration: &configuration::Configuration,
    params: BulkGetGroupsParams,
) -> Result<crate::models::PageBeanGroupDetails, Error<BulkGetGroupsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let start_at = params.start_at;
    let max_results = params.max_results;
    let group_id = params.group_id;
    let group_name = params.group_name;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/2/group/bulk",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = start_at {
        local_var_req_builder =
            local_var_req_builder.query(&[("startAt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_results {
        local_var_req_builder =
            local_var_req_builder.query(&[("maxResults", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = group_id {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("groupId".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "groupId",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = group_name {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("groupName".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "groupName",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
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
        let local_var_entity: Option<BulkGetGroupsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a group.  **[Permissions](#permissions) required:** Site administration (that is, member of the *site-admin* [group](https://confluence.atlassian.com/x/24xjL)).
pub async fn create_group(
    configuration: &configuration::Configuration,
    params: CreateGroupParams,
) -> Result<crate::models::Group, Error<CreateGroupError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let request_body = params.request_body;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/group", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&request_body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of groups whose names contain a query string. A list of group names can be provided to exclude groups from the results.  The primary use case for this resource is to populate a group picker suggestions list. To this end, the returned object includes the `html` field where the matched query term is highlighted in the group name with the HTML strong tag. Also, the groups list is wrapped in a response object that contains a header for use in the picker, specifically *Showing X of Y matching groups*.  The list returns with the groups sorted. If no groups match the list criteria, an empty list is returned.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg). Anonymous calls and calls by users without the required permission return an empty list.
pub async fn find_groups(
    configuration: &configuration::Configuration,
    params: FindGroupsParams,
) -> Result<crate::models::FoundGroups, Error<FindGroupsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_id = params.account_id;
    let query = params.query;
    let exclude = params.exclude;
    let exclude_id = params.exclude_id;
    let max_results = params.max_results;
    let user_name = params.user_name;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/2/groups/picker",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = account_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("accountId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = query {
        local_var_req_builder =
            local_var_req_builder.query(&[("query", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = exclude {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("exclude".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "exclude",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = exclude_id {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("excludeId".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "excludeId",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = max_results {
        local_var_req_builder =
            local_var_req_builder.query(&[("maxResults", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_name {
        local_var_req_builder =
            local_var_req_builder.query(&[("userName", &local_var_str.to_string())]);
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
        let local_var_entity: Option<FindGroupsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This operation is deprecated, use [`group/member`](#api-rest-api-2-group-member-get).  Returns all users in a group.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn get_group(
    configuration: &configuration::Configuration,
    params: GetGroupParams,
) -> Result<crate::models::Group, Error<GetGroupError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let groupname = params.groupname;
    let group_id = params.group_id;
    let expand = params.expand;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/group", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = groupname {
        local_var_req_builder =
            local_var_req_builder.query(&[("groupname", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = group_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("groupId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = expand {
        local_var_req_builder =
            local_var_req_builder.query(&[("expand", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetGroupError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a [paginated](#pagination) list of all users in a group.  Note that users are ordered by username, however the username is not returned in the results due to privacy reasons.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn get_users_from_group(
    configuration: &configuration::Configuration,
    params: GetUsersFromGroupParams,
) -> Result<crate::models::PageBeanUserDetails, Error<GetUsersFromGroupError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let groupname = params.groupname;
    let group_id = params.group_id;
    let include_inactive_users = params.include_inactive_users;
    let start_at = params.start_at;
    let max_results = params.max_results;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/2/group/member",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = groupname {
        local_var_req_builder =
            local_var_req_builder.query(&[("groupname", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = group_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("groupId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_inactive_users {
        local_var_req_builder =
            local_var_req_builder.query(&[("includeInactiveUsers", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_at {
        local_var_req_builder =
            local_var_req_builder.query(&[("startAt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_results {
        local_var_req_builder =
            local_var_req_builder.query(&[("maxResults", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetUsersFromGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a group.  **[Permissions](#permissions) required:** Site administration (that is, member of the *site-admin* strategic [group](https://confluence.atlassian.com/x/24xjL)).
pub async fn remove_group(
    configuration: &configuration::Configuration,
    params: RemoveGroupParams,
) -> Result<(), Error<RemoveGroupError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let groupname = params.groupname;
    let group_id = params.group_id;
    let swap_group = params.swap_group;
    let swap_group_id = params.swap_group_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/group", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = groupname {
        local_var_req_builder =
            local_var_req_builder.query(&[("groupname", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = group_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("groupId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = swap_group {
        local_var_req_builder =
            local_var_req_builder.query(&[("swapGroup", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = swap_group_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("swapGroupId", &local_var_str.to_string())]);
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
        Ok(())
    } else {
        let local_var_entity: Option<RemoveGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Removes a user from a group.  **[Permissions](#permissions) required:** Site administration (that is, member of the *site-admin* [group](https://confluence.atlassian.com/x/24xjL)).
pub async fn remove_user_from_group(
    configuration: &configuration::Configuration,
    params: RemoveUserFromGroupParams,
) -> Result<(), Error<RemoveUserFromGroupError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_id = params.account_id;
    let groupname = params.groupname;
    let group_id = params.group_id;
    let username = params.username;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/2/group/user",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = groupname {
        local_var_req_builder =
            local_var_req_builder.query(&[("groupname", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = group_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("groupId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = username {
        local_var_req_builder =
            local_var_req_builder.query(&[("username", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("accountId", &account_id.to_string())]);
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
        let local_var_entity: Option<RemoveUserFromGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
