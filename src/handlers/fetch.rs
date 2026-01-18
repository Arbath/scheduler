use axum::{Json, extract::State, http::Uri, response::IntoResponse};
use serde::{Deserialize, Serialize};
use crate::middleware::auth::{AuthAdmin, AuthUser}; 
use crate::services::fetch::FetchService;
use crate::state::AppState;
use crate::utils::{requests::{ValidatedJson, ValidatedPath}, response::{ApiError, WebResponse}};
use crate::models::fetch::{CreateApi, CreateApiMembers, ReqCreateApiExecute, UpdateApi, UpdateApiExecute, UpdateApiMembers};

#[derive(Deserialize, Serialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: i32,
    title: String,
    completed: bool,
}

pub async fn fetch_external_api(
    State(state): State<AppState>, 
) -> Json<serde_json::Value> {
    let url = "https://jsonplaceholder.typicode.com/todos/1";

    // Gunakan client yang sudah ada di state
    // .send() dan .json() bersifat async (non-blocking)
    let response = state.http_client
        .get(url)
        .send()
        .await;

    println!("INI RESPONSENYA {:?}", response);

    match response {
        Ok(resp) => {
            // Parse JSON langsung ke struct atau Value
            match resp.json::<Todo>().await {
                Ok(data) => Json(serde_json::json!({ "status": "success", "data": data })),
                Err(_) => Json(serde_json::json!({ "status": "error", "message": "Gagal parse JSON" })),
            }
        }
        Err(_) => Json(serde_json::json!({ "status": "error", "message": "Request gagal" })),
    }
}

pub async fn get_all(
    uri: Uri,
    AuthAdmin(_): AuthAdmin,
    service: FetchService,
) -> Result<impl IntoResponse, ApiError> {
    let response = service.get_all_fetch().await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok(&uri, "Success", response))
}

pub async fn get_fetch_job(
    ValidatedPath(job_id): ValidatedPath<String>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: FetchService,
) -> Result<impl IntoResponse, ApiError> {
    let response = service.get_fetch_by_job(&job_id).await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok(&uri, "Success", response))
}

pub async fn get_fetch_api(
    ValidatedPath(id): ValidatedPath<i32>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: FetchService,
) -> Result<impl IntoResponse, ApiError> {
    let response = service.get_fetch_by_id(&id).await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok(&uri, "Success", response))
}

pub async fn create_fetch_api(
    uri: Uri,
    AuthUser(user): AuthUser,
    service: FetchService,
    ValidatedJson(data): ValidatedJson<CreateApi>
) -> Result<impl IntoResponse, ApiError> {
    let response = service.create_fetch(data, user).await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok(&uri, "Fetch Api Created", response))
}

pub async fn update_fetch_api(
    ValidatedPath(id): ValidatedPath<i32>,
    uri: Uri,
    AuthUser(user): AuthUser,
    service: FetchService,
    ValidatedJson(data): ValidatedJson<UpdateApi>
) -> Result<impl IntoResponse, ApiError> {
    let response = service.update_fetch(&id, data, user).await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok(&uri, "Fetch Api Created", response))
}

pub async fn delete_fetch_api(
    ValidatedPath(id): ValidatedPath<i32>,
    uri: Uri,
    AuthUser(user): AuthUser,
    service: FetchService,
) -> Result<impl IntoResponse, ApiError> {
    service.delete_fetch(&id, user).await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok_empty(&uri, "Fetch Api Deleted"))
}

pub async fn get_fetch_member(
    ValidatedPath((fetch_id, id)): ValidatedPath<(i32, i32)>,
    uri: Uri,
    AuthUser(user): AuthUser,
    service: FetchService,
) -> Result<impl IntoResponse, ApiError>{
    let response = service.find_member(user,fetch_id, id)
        .await
        .map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok(&uri, "List fetch members", response))
}

pub async fn get_all_member(
    ValidatedPath(fetch_id): ValidatedPath<i32>,
    uri: Uri,
    AuthUser(user): AuthUser,
    service: FetchService,
) -> Result<impl IntoResponse, ApiError>{
    let response = service.find_members(user, fetch_id)
        .await
        .map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok(&uri, "List fetch members", response))
}

pub async fn create_fetch_member(
    ValidatedPath(fetch_id): ValidatedPath<i32>,
    uri: Uri,
    AuthUser(user): AuthUser,
    service: FetchService,
    ValidatedJson(data): ValidatedJson<CreateApiMembers>
) -> Result<impl IntoResponse, ApiError>{
    let response = service.add_member(user, fetch_id, data)
        .await
        .map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::created(&uri, "Success add a member", response))
}

pub async fn update_fetch_member(
    ValidatedPath((fetch_id, id)): ValidatedPath<(i32, i32)>,
    uri: Uri,
    AuthUser(user): AuthUser,
    service: FetchService,
    ValidatedJson(data): ValidatedJson<UpdateApiMembers>
) -> Result<impl IntoResponse, ApiError>{
    let response = service.update_member(user, id, fetch_id, data)
        .await
        .map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::created(&uri, "Success add a member", response))
}

pub async fn delete_fetch_member(
    ValidatedPath((fetch_id, id)): ValidatedPath<(i32, i32)>,
    uri: Uri,
    AuthUser(user): AuthUser,
    service: FetchService,
) -> Result<impl IntoResponse, ApiError> {
    service.delete_member(user, id, fetch_id).await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok_empty(&uri, "Member Deleted"))
}

pub async fn get_fetch_execute(
    ValidatedPath(id): ValidatedPath<i32>,
    uri: Uri,
    AuthUser(user): AuthUser,
    service: FetchService,
) -> Result<impl IntoResponse, ApiError> {
    let response = service.get_execute(user, id).await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok(&uri, "Success get api execute", response))
}
pub async fn get_all_execute(
    uri: Uri,
    AuthUser(user): AuthUser,
    service: FetchService,
) -> Result<impl IntoResponse, ApiError> {
    let response = service.get_all_execute(user).await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok(&uri, "List get api execute", response))
}

pub async fn create_fetch_execute(
    uri: Uri,
    AuthUser(user): AuthUser,
    service: FetchService,
    ValidatedJson(data): ValidatedJson<ReqCreateApiExecute>,
) -> Result<impl IntoResponse, ApiError> {
    let response = service.create_execute(user, data).await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::created(&uri, "Api execute created!", response))
}

pub async fn update_fetch_execute(
    ValidatedPath(id): ValidatedPath<i32>,
    uri: Uri,
    AuthUser(user): AuthUser,
    service: FetchService,
    ValidatedJson(data): ValidatedJson<UpdateApiExecute>
) -> Result<impl IntoResponse, ApiError> {
    let response = service.update_execute(user, id, data).await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok(&uri, "Api execute updated!", response))
}

pub async fn delete_fetch_execute(
    ValidatedPath(id): ValidatedPath<i32>,
    uri: Uri,
    AuthUser(user): AuthUser,
    service: FetchService,
) -> Result<impl IntoResponse, ApiError> {
    let response = service.delete_execute(user, id).await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok(&uri, "Api execute deleted!", response))
}