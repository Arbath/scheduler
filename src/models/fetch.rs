use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use chrono::{DateTime,Utc};

// Struct for table fetch_api
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "fetch_api_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ApiType {
    Rest,
    Websocket,
    Mqtt,
    Graphql,
}
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "fetch_api_method", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ApiMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Api {
    pub id: i32,
    pub name: String,
    pub r#type: ApiType,
    pub method: ApiMethod,
    pub topic: Option<Value>,
    pub job_id: String,
    pub description: String,
    pub execute_id: i32,
    pub header_id: Option<i32>,
    pub is_active: bool,
    pub secure: bool,
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CreateApi {
    pub name: String,
    pub r#type: Option<ApiType>,
    pub method: Option<ApiMethod>,
    pub topic: Option<Value>,
    pub job_id: String,
    pub description: String,
    pub execute_id: i32,
    pub header_id: Option<i32>,
    pub is_active: Option<bool>,
    pub secure: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UpdateApi {
    pub name: Option<String>,
    pub r#type: Option<ApiType>,
    pub method: Option<ApiMethod>,
    pub topic: Option<Value>,
    pub job_id: Option<String>,
    pub description: Option<String>,
    pub execute_id: Option<i32>,
    pub header_id: Option<i32>,
    pub is_active: Option<bool>,
    pub secure: Option<bool>,
}

// Struct for table fetch_api_members
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "fetch_member_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Owner,
    Editor,
    Viewer,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ApiMembers {
    pub id: i32,
    pub fetch_id: i32,
    pub user_id: i32,
    pub role: Option<Role>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateApiMembers {
    pub user_id: i32,
    pub role: Role,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct UpdateApiMembers {
    pub role: Role, 
}

// Struct for table fetch_api_execute
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "execute_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ExecuteType {
    Seconds,
    Minutes,
    Hours,
    Days,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ApiExecute {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub is_repeat: bool,
    pub r#type: Option<ExecuteType>,
    pub value: i64,
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CreateApiExecute {
    pub user_id: i32,
    pub name: String,
    pub is_repeat: bool,
    pub r#type: Option<ExecuteType>,
    pub value: i64,
}

#[derive(Deserialize)]
pub struct ReqCreateApiExecute {
    pub name: String,
    pub is_repeat: bool,
    pub r#type: Option<ExecuteType>,
    pub value: i64,
}
impl ReqCreateApiExecute {
    pub fn into_model(self, user_id:i32) -> CreateApiExecute {
        CreateApiExecute {
            user_id,
            name: self.name,
            is_repeat: self.is_repeat,
            r#type: self.r#type,
            value: self.value,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UpdateApiExecute {
    pub name: Option<String>,
    pub is_repeat: Option<bool>,
    pub r#type: Option<ExecuteType>,
    pub value: Option<i64>,
}

// Struct for table fetch_api_header
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ApiHeader {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub headers: Value,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CreateApiHeader {
    pub user_id: i32,
    pub name: String,
    pub headers: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UpdateApiHeader {
    pub user_id: i32,
    pub name: Option<String>,
    pub headers: Option<Value>,
}

// Struct for table fetch_api_data
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ApiData {
    pub id: i32,
    pub fetch_id: i32,
    pub user_id: i32,
    pub name: String,
    pub payload: String,
    pub status_code: i16,
    pub response: String,
    pub response_headers: Value,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CreateApiData {
    pub fetch_id: i32,
    pub user_id: i32,
    pub name: String,
    pub payload: Option<String>,
    pub status_code: Option<i16>,
    pub response: Option<String>,
    pub response_headers: Option<Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UpdateApiData {
    pub fetch_id: Option<i32>,
    pub user_id: i32,
    pub name: Option<String>,
    pub payload: Option<String>,
    pub status_code: Option<i16>,
    pub response: Option<String>,
    pub response_headers: Value,
}