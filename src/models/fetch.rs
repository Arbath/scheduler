use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use chrono::{DateTime,Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "fetch_api_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ApiType {
    Rest,
    Websocket,
    Mqtt,
    Graph,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Api {
    pub id: i32,
    pub name: String,
    pub r#type: ApiType,
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
    pub job_id: String,
    pub description: String,
    pub execute_id: i32,
    pub header_id: Option<i32>,
    pub is_active: Option<bool>,
    pub secure: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UpdateApi {
    pub name: String,
    pub r#type: ApiType,
    pub job_id: String,
    pub description: String,
    pub execute_id: i32,
    pub header_id: Option<i32>,
    pub is_active: Option<bool>,
    pub secure: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
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
    pub fetch_id: i32,
    pub user_id: i32,
    pub role: Option<Role>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRequest {
    pub fetch_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role: Option<Role>, 
}

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
pub struct ApiHeader {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub headers: Value,
    pub updated_at: DateTime<Utc>,
}
