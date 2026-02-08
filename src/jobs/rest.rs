use apalis::prelude::*;
use apalis_sql::context::SqlContext;
use reqwest::Method;
use serde_json::{Map, Value};
use crate::{models::fetch::{Api, ApiMethod, CreateApiData}, repository::fetch::{FetchDataRepository, FetchExecuteRepository, FetchHeaderRepository, FetchRepository}, services::fetch::FetchService, state::AppState, utils::reqwest::json_to_headermap};

pub async fn execute_job(
    job: Api,
    mut ctx: SqlContext,
    state: Data<AppState>,
) -> Result<(), anyhow::Error> {
    ctx.set_max_attempts(10);
    let execute_repo = FetchExecuteRepository::new(state.database.clone());
    let fetch_repo = FetchRepository::new(state.database.clone());
    let header_repo = FetchHeaderRepository::new(state.database.clone());
    let data_repo = FetchDataRepository::new(state.database.clone());
    let fetch_service = FetchService::new((*state).clone());

    let fetch_api = fetch_repo.get_by_id(&job.id).await?;

    let headers_json = if let Some(h_id) = fetch_api.header_id {
        match header_repo.find_by_id(h_id).await {
            Ok(data) => Some(data.headers),
            Err(e) => {
                tracing::warn!("Header ID {} not found: {:?}. Default.", h_id, e);
                None
            }
        }
    } else {
        None
    };
    
    let headers_map = json_to_headermap(headers_json).await; 

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

    if let Some(payload) = &fetch_api.payload {
        if !payload.is_empty() {
             request_builder = request_builder.body(payload.clone());
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

    let res_text = response.text().await?;
    let res_body = match serde_json::from_str::<Value>(&res_text) {
        Ok(json) => {
            Some(serde_json::to_string_pretty(&json)?)
        }
        Err(_) => Some(res_text),
    };


    // Save data
    let fetch_id = fetch_api.id.clone();
    let fetch_job_id = fetch_api.job_id.clone().unwrap_or_else(|| "unknown".to_string());
    let name_data = format!("{} [{}-{}]", fetch_api.name,fetch_id, fetch_job_id);

    let response_data = CreateApiData {
        fetch_id: fetch_api.id,
        name: name_data,
        status_code: Some(status_code),
        response: res_body,
        response_headers: Some(response_headers_json),
    };

    data_repo.create(response_data).await?;

    tracing::info!("Done request to {}. [{}]", fetch_api.endpoint, status_obj,);

    // Create repeatable jobs
    let execute = execute_repo.find_by_id(fetch_api.execute_id).await?;
    if execute.is_repeat {
        let job_id = fetch_service.create_apalis_job(&fetch_api, execute)
            .await.map_err(|e| anyhow::anyhow!("Failed to create repeatable jobs: {:?}", e))?;
        let _ = fetch_repo.update_job_id(fetch_api.id, job_id)
            .await.map_err(|e| anyhow::anyhow!("Failed to update job id: {:?}", e))?;
    }

    Ok(())
}