use crate::server::api::AppState;

use crate::view_file::acces_db::{fetch_data_by_id, fetch_path_by_id, fetch_size_by_id};
use axum::extract::Path;
use axum::{extract::Extension, Json};

use std::sync::Arc;

pub async fn fetch_path_handler(
    id: Path<i32>,
    state: Extension<Arc<AppState>>,
) -> Result<Json<String>, String> {
    let path = fetch_path_by_id(&state.pool, *id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(path))
}

pub async fn fetch_size_handler(
    id: Path<i32>,
    state: Extension<Arc<AppState>>,
) -> Result<Json<i64>, String> {
    let size = fetch_size_by_id(&state.pool, *id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(size))
}

pub async fn fetch_data_handler(
    id: Path<i32>,
    state: Extension<Arc<AppState>>,
) -> Result<Json<String>, String> {
    let id = *id;
    let data = fetch_data_by_id(&state.pool, id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(data))
}
