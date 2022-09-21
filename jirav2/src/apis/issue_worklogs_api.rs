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

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`add_worklog`]
#[derive(Clone, Debug, Default)]
pub struct AddWorklogParams {
    /// The ID or key the issue.
    pub issue_id_or_key: String,
    pub request_body: ::std::collections::HashMap<String, serde_json::Value>,
    /// Whether users watching the issue are notified by email.
    pub notify_users: Option<bool>,
    /// Defines how to update the issue's time estimate, the options are:   *  `new` Sets the estimate to a specific value, defined in `newEstimate`.  *  `leave` Leaves the estimate unchanged.  *  `manual` Reduces the estimate by amount specified in `reduceBy`.  *  `auto` Reduces the estimate by the value of `timeSpent` in the worklog.
    pub adjust_estimate: Option<String>,
    /// The value to set as the issue's remaining time estimate, as days (\\#d), hours (\\#h), or minutes (\\#m or \\#). For example, *2d*. Required when `adjustEstimate` is `new`.
    pub new_estimate: Option<String>,
    /// The amount to reduce the issue's remaining estimate by, as days (\\#d), hours (\\#h), or minutes (\\#m). For example, *2d*. Required when `adjustEstimate` is `manual`.
    pub reduce_by: Option<String>,
    /// Use [expand](#expansion) to include additional information about work logs in the response. This parameter accepts `properties`, which returns worklog properties.
    pub expand: Option<String>,
    /// Whether the worklog entry should be added to the issue even if the issue is not editable, because jira.issue.editable set to false or missing. For example, the issue is closed. Connect app users with admin permission and Forge app users with the `manage:jira-configuration` scope can use this flag.
    pub override_editable_flag: Option<bool>
}

/// struct for passing parameters to the method [`delete_worklog`]
#[derive(Clone, Debug, Default)]
pub struct DeleteWorklogParams {
    /// The ID or key of the issue.
    pub issue_id_or_key: String,
    /// The ID of the worklog.
    pub id: String,
    /// Whether users watching the issue are notified by email.
    pub notify_users: Option<bool>,
    /// Defines how to update the issue's time estimate, the options are:   *  `new` Sets the estimate to a specific value, defined in `newEstimate`.  *  `leave` Leaves the estimate unchanged.  *  `manual` Increases the estimate by amount specified in `increaseBy`.  *  `auto` Reduces the estimate by the value of `timeSpent` in the worklog.
    pub adjust_estimate: Option<String>,
    /// The value to set as the issue's remaining time estimate, as days (\\#d), hours (\\#h), or minutes (\\#m or \\#). For example, *2d*. Required when `adjustEstimate` is `new`.
    pub new_estimate: Option<String>,
    /// The amount to increase the issue's remaining estimate by, as days (\\#d), hours (\\#h), or minutes (\\#m or \\#). For example, *2d*. Required when `adjustEstimate` is `manual`.
    pub increase_by: Option<String>,
    /// Whether the work log entry should be added to the issue even if the issue is not editable, because jira.issue.editable set to false or missing. For example, the issue is closed. Connect app users with admin permission and Forge app users with the `manage:jira-configuration` scope can use this flag.
    pub override_editable_flag: Option<bool>
}

/// struct for passing parameters to the method [`get_ids_of_worklogs_deleted_since`]
#[derive(Clone, Debug, Default)]
pub struct GetIdsOfWorklogsDeletedSinceParams {
    /// The date and time, as a UNIX timestamp in milliseconds, after which deleted worklogs are returned.
    pub since: Option<i64>
}

/// struct for passing parameters to the method [`get_ids_of_worklogs_modified_since`]
#[derive(Clone, Debug, Default)]
pub struct GetIdsOfWorklogsModifiedSinceParams {
    /// The date and time, as a UNIX timestamp in milliseconds, after which updated worklogs are returned.
    pub since: Option<i64>,
    /// Use [expand](#expansion) to include additional information about worklogs in the response. This parameter accepts `properties` that returns the properties of each worklog.
    pub expand: Option<String>
}

/// struct for passing parameters to the method [`get_issue_worklog`]
#[derive(Clone, Debug, Default)]
pub struct GetIssueWorklogParams {
    /// The ID or key of the issue.
    pub issue_id_or_key: String,
    /// The index of the first item to return in a page of results (page offset).
    pub start_at: Option<i64>,
    /// The maximum number of items to return per page.
    pub max_results: Option<i32>,
    /// The worklog start date and time, as a UNIX timestamp in milliseconds, after which worklogs are returned.
    pub started_after: Option<i64>,
    /// The worklog start date and time, as a UNIX timestamp in milliseconds, before which worklogs are returned.
    pub started_before: Option<i64>,
    /// Use [expand](#expansion) to include additional information about worklogs in the response. This parameter accepts`properties`, which returns worklog properties.
    pub expand: Option<String>
}

/// struct for passing parameters to the method [`get_worklog`]
#[derive(Clone, Debug, Default)]
pub struct GetWorklogParams {
    /// The ID or key of the issue.
    pub issue_id_or_key: String,
    /// The ID of the worklog.
    pub id: String,
    /// Use [expand](#expansion) to include additional information about work logs in the response. This parameter accepts  `properties`, which returns worklog properties.
    pub expand: Option<String>
}

/// struct for passing parameters to the method [`get_worklogs_for_ids`]
#[derive(Clone, Debug, Default)]
pub struct GetWorklogsForIdsParams {
    /// A JSON object containing a list of worklog IDs.
    pub worklog_ids_request_bean: crate::models::WorklogIdsRequestBean,
    /// Use [expand](#expansion) to include additional information about worklogs in the response. This parameter accepts `properties` that returns the properties of each worklog.
    pub expand: Option<String>
}

/// struct for passing parameters to the method [`update_worklog`]
#[derive(Clone, Debug, Default)]
pub struct UpdateWorklogParams {
    /// The ID or key the issue.
    pub issue_id_or_key: String,
    /// The ID of the worklog.
    pub id: String,
    pub request_body: ::std::collections::HashMap<String, serde_json::Value>,
    /// Whether users watching the issue are notified by email.
    pub notify_users: Option<bool>,
    /// Defines how to update the issue's time estimate, the options are:   *  `new` Sets the estimate to a specific value, defined in `newEstimate`.  *  `leave` Leaves the estimate unchanged.  *  `auto` Updates the estimate by the difference between the original and updated value of `timeSpent` or `timeSpentSeconds`.
    pub adjust_estimate: Option<String>,
    /// The value to set as the issue's remaining time estimate, as days (\\#d), hours (\\#h), or minutes (\\#m or \\#). For example, *2d*. Required when `adjustEstimate` is `new`.
    pub new_estimate: Option<String>,
    /// Use [expand](#expansion) to include additional information about worklogs in the response. This parameter accepts `properties`, which returns worklog properties.
    pub expand: Option<String>,
    /// Whether the worklog should be added to the issue even if the issue is not editable. For example, because the issue is closed. Connect app users with admin permission and Forge app users with the `manage:jira-configuration` scope can use this flag.
    pub override_editable_flag: Option<bool>
}


/// struct for typed errors of method [`add_worklog`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddWorklogError {
    Status400(),
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_worklog`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWorklogError {
    Status400(),
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_ids_of_worklogs_deleted_since`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIdsOfWorklogsDeletedSinceError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_ids_of_worklogs_modified_since`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIdsOfWorklogsModifiedSinceError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_issue_worklog`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIssueWorklogError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_worklog`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorklogError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_worklogs_for_ids`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorklogsForIdsError {
    Status400(),
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_worklog`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWorklogError {
    Status400(),
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}


/// Adds a worklog to an issue.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* and *Work on issues* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub async fn add_worklog(configuration: &configuration::Configuration, params: AddWorklogParams) -> Result<crate::models::Worklog, Error<AddWorklogError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let issue_id_or_key = params.issue_id_or_key;
    let request_body = params.request_body;
    let notify_users = params.notify_users;
    let adjust_estimate = params.adjust_estimate;
    let new_estimate = params.new_estimate;
    let reduce_by = params.reduce_by;
    let expand = params.expand;
    let override_editable_flag = params.override_editable_flag;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/issue/{issueIdOrKey}/worklog", local_var_configuration.base_path, issueIdOrKey=crate::apis::urlencode(issue_id_or_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = notify_users {
        local_var_req_builder = local_var_req_builder.query(&[("notifyUsers", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = adjust_estimate {
        local_var_req_builder = local_var_req_builder.query(&[("adjustEstimate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = new_estimate {
        local_var_req_builder = local_var_req_builder.query(&[("newEstimate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = reduce_by {
        local_var_req_builder = local_var_req_builder.query(&[("reduceBy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = expand {
        local_var_req_builder = local_var_req_builder.query(&[("expand", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = override_editable_flag {
        local_var_req_builder = local_var_req_builder.query(&[("overrideEditableFlag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&request_body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddWorklogError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a worklog from an issue.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  *Delete all worklogs*[ project permission](https://confluence.atlassian.com/x/yodKLg) to delete any worklog or *Delete own worklogs* to delete worklogs created by the user,  *  If the worklog has visibility restrictions, belongs to the group or has the role visibility is restricted to.
pub async fn delete_worklog(configuration: &configuration::Configuration, params: DeleteWorklogParams) -> Result<(), Error<DeleteWorklogError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let issue_id_or_key = params.issue_id_or_key;
    let id = params.id;
    let notify_users = params.notify_users;
    let adjust_estimate = params.adjust_estimate;
    let new_estimate = params.new_estimate;
    let increase_by = params.increase_by;
    let override_editable_flag = params.override_editable_flag;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/issue/{issueIdOrKey}/worklog/{id}", local_var_configuration.base_path, issueIdOrKey=crate::apis::urlencode(issue_id_or_key), id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = notify_users {
        local_var_req_builder = local_var_req_builder.query(&[("notifyUsers", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = adjust_estimate {
        local_var_req_builder = local_var_req_builder.query(&[("adjustEstimate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = new_estimate {
        local_var_req_builder = local_var_req_builder.query(&[("newEstimate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = increase_by {
        local_var_req_builder = local_var_req_builder.query(&[("increaseBy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = override_editable_flag {
        local_var_req_builder = local_var_req_builder.query(&[("overrideEditableFlag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteWorklogError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of IDs and delete timestamps for worklogs deleted after a date and time.  This resource is paginated, with a limit of 1000 worklogs per page. Each page lists worklogs from oldest to youngest. If the number of items in the date range exceeds 1000, `until` indicates the timestamp of the youngest item on the page. Also, `nextPage` provides the URL for the next page of worklogs. The `lastPage` parameter is set to true on the last page of worklogs.  This resource does not return worklogs deleted during the minute preceding the request.  **[Permissions](#permissions) required:** Permission to access Jira.
pub async fn get_ids_of_worklogs_deleted_since(configuration: &configuration::Configuration, params: GetIdsOfWorklogsDeletedSinceParams) -> Result<crate::models::ChangedWorklogs, Error<GetIdsOfWorklogsDeletedSinceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let since = params.since;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/worklog/deleted", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = since {
        local_var_req_builder = local_var_req_builder.query(&[("since", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetIdsOfWorklogsDeletedSinceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of IDs and update timestamps for worklogs updated after a date and time.  This resource is paginated, with a limit of 1000 worklogs per page. Each page lists worklogs from oldest to youngest. If the number of items in the date range exceeds 1000, `until` indicates the timestamp of the youngest item on the page. Also, `nextPage` provides the URL for the next page of worklogs. The `lastPage` parameter is set to true on the last page of worklogs.  This resource does not return worklogs updated during the minute preceding the request.  **[Permissions](#permissions) required:** Permission to access Jira, however, worklogs are only returned where either of the following is true:   *  the worklog is set as *Viewable by All Users*.  *  the user is a member of a project role or group with permission to view the worklog.
pub async fn get_ids_of_worklogs_modified_since(configuration: &configuration::Configuration, params: GetIdsOfWorklogsModifiedSinceParams) -> Result<crate::models::ChangedWorklogs, Error<GetIdsOfWorklogsModifiedSinceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let since = params.since;
    let expand = params.expand;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/worklog/updated", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = since {
        local_var_req_builder = local_var_req_builder.query(&[("since", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = expand {
        local_var_req_builder = local_var_req_builder.query(&[("expand", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetIdsOfWorklogsModifiedSinceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns worklogs for an issue, starting from the oldest worklog or from the worklog started on or after a date and time.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** Workloads are only returned where the user has:   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  If the worklog has visibility restrictions, belongs to the group or has the role visibility is restricted to.
pub async fn get_issue_worklog(configuration: &configuration::Configuration, params: GetIssueWorklogParams) -> Result<crate::models::PageOfWorklogs, Error<GetIssueWorklogError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let issue_id_or_key = params.issue_id_or_key;
    let start_at = params.start_at;
    let max_results = params.max_results;
    let started_after = params.started_after;
    let started_before = params.started_before;
    let expand = params.expand;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/issue/{issueIdOrKey}/worklog", local_var_configuration.base_path, issueIdOrKey=crate::apis::urlencode(issue_id_or_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = start_at {
        local_var_req_builder = local_var_req_builder.query(&[("startAt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_results {
        local_var_req_builder = local_var_req_builder.query(&[("maxResults", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = started_after {
        local_var_req_builder = local_var_req_builder.query(&[("startedAfter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = started_before {
        local_var_req_builder = local_var_req_builder.query(&[("startedBefore", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = expand {
        local_var_req_builder = local_var_req_builder.query(&[("expand", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetIssueWorklogError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a worklog.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  If the worklog has visibility restrictions, belongs to the group or has the role visibility is restricted to.
pub async fn get_worklog(configuration: &configuration::Configuration, params: GetWorklogParams) -> Result<crate::models::Worklog, Error<GetWorklogError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let issue_id_or_key = params.issue_id_or_key;
    let id = params.id;
    let expand = params.expand;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/issue/{issueIdOrKey}/worklog/{id}", local_var_configuration.base_path, issueIdOrKey=crate::apis::urlencode(issue_id_or_key), id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = expand {
        local_var_req_builder = local_var_req_builder.query(&[("expand", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetWorklogError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns worklog details for a list of worklog IDs.  The returned list of worklogs is limited to 1000 items.  **[Permissions](#permissions) required:** Permission to access Jira, however, worklogs are only returned where either of the following is true:   *  the worklog is set as *Viewable by All Users*.  *  the user is a member of a project role or group with permission to view the worklog.
pub async fn get_worklogs_for_ids(configuration: &configuration::Configuration, params: GetWorklogsForIdsParams) -> Result<Vec<crate::models::Worklog>, Error<GetWorklogsForIdsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let worklog_ids_request_bean = params.worklog_ids_request_bean;
    let expand = params.expand;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/worklog/list", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = expand {
        local_var_req_builder = local_var_req_builder.query(&[("expand", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&worklog_ids_request_bean);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetWorklogsForIdsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a worklog.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  *Edit all worklogs*[ project permission](https://confluence.atlassian.com/x/yodKLg) to update any worklog or *Edit own worklogs* to update worklogs created by the user.  *  If the worklog has visibility restrictions, belongs to the group or has the role visibility is restricted to.
pub async fn update_worklog(configuration: &configuration::Configuration, params: UpdateWorklogParams) -> Result<crate::models::Worklog, Error<UpdateWorklogError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let issue_id_or_key = params.issue_id_or_key;
    let id = params.id;
    let request_body = params.request_body;
    let notify_users = params.notify_users;
    let adjust_estimate = params.adjust_estimate;
    let new_estimate = params.new_estimate;
    let expand = params.expand;
    let override_editable_flag = params.override_editable_flag;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/issue/{issueIdOrKey}/worklog/{id}", local_var_configuration.base_path, issueIdOrKey=crate::apis::urlencode(issue_id_or_key), id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = notify_users {
        local_var_req_builder = local_var_req_builder.query(&[("notifyUsers", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = adjust_estimate {
        local_var_req_builder = local_var_req_builder.query(&[("adjustEstimate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = new_estimate {
        local_var_req_builder = local_var_req_builder.query(&[("newEstimate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = expand {
        local_var_req_builder = local_var_req_builder.query(&[("expand", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = override_editable_flag {
        local_var_req_builder = local_var_req_builder.query(&[("overrideEditableFlag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&request_body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateWorklogError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

