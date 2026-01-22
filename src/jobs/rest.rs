use apalis::prelude::*;
use apalis_sql::context::SqlContext;
use reqwest::Method;
use serde_json::{Map, Value};
use crate::{models::fetch::{Api, ApiMethod, CreateApiData}, repository::fetch::{FetchDataRepository, FetchHeaderRepository, FetchRepository}, state::AppState, utils::reqwest::json_to_headermap};

pub async fn execute_job(
    job: Api,
    mut ctx: SqlContext,
    state: Data<AppState>,
) -> Result<(), anyhow::Error> {
    let fetch_repo = FetchRepository::new(state.database.clone());
    let header_repo = FetchHeaderRepository::new(state.database.clone());
    let data_repo = FetchDataRepository::new(state.database.clone());

    let fetch_api = fetch_repo.get_by_id(&job.id).await?;
    let header_id = fetch_api.header_id.unwrap_or_default();
    let headers_data = header_repo.find_by_id(header_id).await?;
    
    let headers_map = json_to_headermap(Some(headers_data.headers)).await; 

    let req_method = match fetch_api.method {
        Some(ApiMethod::Get) => Method::GET,
        Some(ApiMethod::Post) => Method::POST,
        Some(ApiMethod::Put) => Method::PUT,
        Some(ApiMethod::Delete) => Method::DELETE,
        Some(ApiMethod::Patch) => Method::PATCH,
        None => Method::GET, 
        // _ => Method::GET,
    };

    let mut request_builder = state.http_client
        .request(req_method, &fetch_api.endpoint)
        .headers(headers_map);

    if let Some(payload) = fetch_api.payload {
        if !payload.is_empty() {
             request_builder = request_builder.body(payload);
        }
    }

    let response = request_builder.send().await?;

    let status_obj = response.status();
    let status_code = status_obj.as_u16() as i16;

    let mut response_headers_map = Map::new();
    for (k, v) in response.headers() {
        let key_str = k.to_string();
        let val_str = v.to_str().unwrap_or("").to_string();
        response_headers_map.insert(key_str, Value::String(val_str));
    }
    let response_headers_json = Value::Object(response_headers_map);

    let res_body = response.text().await?;

    // Save data
    let fetch_id = fetch_api.id.clone();
    let fetch_job_id = fetch_api.job_id.clone().unwrap_or_else(|| "unknown".to_string());
    let name_data = format!("{} [{}-{}]", fetch_api.name,fetch_id, fetch_job_id);

    let response_data = CreateApiData {
        fetch_id: fetch_api.id,
        name: name_data,
        status_code: Some(status_code),
        response: Some(res_body),
        response_headers: Some(response_headers_json),
    };

    data_repo.create(response_data).await?;

    tracing::info!("Done request to {}. [{}]", fetch_api.endpoint, status_obj,);

    let max_attempts = 10;
    ctx.set_max_attempts(max_attempts);

    Ok(())
}