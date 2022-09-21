/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// SharePermission : Details of a share permission for the filter.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SharePermission {
    /// The unique identifier of the share permission.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The type of share permission:   *  `user` Shared with a user.  *  `group` Shared with a group. If set in a request, then specify `sharePermission.group` as well.  *  `project` Shared with a project. If set in a request, then specify `sharePermission.project` as well.  *  `projectRole` Share with a project role in a project. This value is not returned in responses. It is used in requests, where it needs to be specify with `projectId` and `projectRoleId`.  *  `global` Shared globally. If set in a request, no other `sharePermission` properties need to be specified.  *  `loggedin` Shared with all logged-in users. Note: This value is set in a request by specifying `authenticated` as the `type`.  *  `project-unknown` Shared with a project that the user does not have access to. Cannot be set in a request.
    #[serde(rename = "type")]
    pub _type: Type,
    /// The project that the filter is shared with. This is similar to the project object returned by [Get project](#api-rest-api-2-project-projectIdOrKey-get) but it contains a subset of the properties, which are: `self`, `id`, `key`, `assigneeType`, `name`, `roles`, `avatarUrls`, `projectType`, `simplified`.   For a request, specify the `id` for the project.
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<crate::models::Project>,
    /// The project role that the filter is shared with.   For a request, specify the `id` for the role. You must also specify the `project` object and `id` for the project that the role is in.
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<crate::models::ProjectRole>,
    /// The group that the filter is shared with. For a request, specify the `name` property for the group.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<crate::models::GroupName>,
    /// The user account ID that the filter is shared with. For a request, specify the `accountId` property for the user.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::UserBean>,
}

impl SharePermission {
    /// Details of a share permission for the filter.
    pub fn new(_type: Type) -> SharePermission {
        SharePermission {
            id: None,
            _type,
            project: None,
            role: None,
            group: None,
            user: None,
        }
    }
}

/// The type of share permission:   *  `user` Shared with a user.  *  `group` Shared with a group. If set in a request, then specify `sharePermission.group` as well.  *  `project` Shared with a project. If set in a request, then specify `sharePermission.project` as well.  *  `projectRole` Share with a project role in a project. This value is not returned in responses. It is used in requests, where it needs to be specify with `projectId` and `projectRoleId`.  *  `global` Shared globally. If set in a request, no other `sharePermission` properties need to be specified.  *  `loggedin` Shared with all logged-in users. Note: This value is set in a request by specifying `authenticated` as the `type`.  *  `project-unknown` Shared with a project that the user does not have access to. Cannot be set in a request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "projectRole")]
    ProjectRole,
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "loggedin")]
    Loggedin,
    #[serde(rename = "authenticated")]
    Authenticated,
    #[serde(rename = "project-unknown")]
    ProjectUnknown,
}

impl Default for Type {
    fn default() -> Type {
        Self::User
    }
}

