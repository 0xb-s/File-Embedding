use crate::db::DbForStorageFile;
use crate::read_path::File;

use axum::response::IntoResponse;

use axum::{extract::Json, response::Json as JsonResponse};
use serde::Deserialize;
use sqlx::SqlitePool;

use std::path::Path;
use std::sync::Arc;
#[derive(Deserialize)]
pub struct EmbedReq {
    pub path: String,
}
pub struct AppState {
    pub pool: Arc<SqlitePool>,
}

pub async fn create_db_table(state: Arc<AppState>) -> impl IntoResponse {
    DbForStorageFile::create_table(&state.pool).await;
    JsonResponse(serde_json::json!({"status": "ok"}))
}

pub async fn file_post(Json(req): Json<EmbedReq>, state: Arc<AppState>) -> impl IntoResponse {
    let path = Path::new(&req.path);
    let file = File::new(path.to_path_buf());
    DbForStorageFile::embed_file(&state.pool, &file).await;
    Json(serde_json::json!({"status": "ok"}))
}

pub async fn directory_post(Json(req): Json<EmbedReq>, state: Arc<AppState>) -> impl IntoResponse {
    let path = Path::new(&req.path);
    DbForStorageFile::embed_directory(&state.pool, &path)
        .await
        .unwrap();
    Json(serde_json::json!({"status": "ok"}))
}
// pub async fn test(
//     pool: Arc<SqlitePool>,
//     Json(EmbedReq { path }): Json<EmbedReq>,
// ) -> impl axum::response::IntoResponse {
//     let path = Path::new(&path);

//     match DbForStorageFile::embed_directory(&pool, &path).await {
//         Ok(_) => JsonResponse(serde_json::json!({"status": "ok"})),
//         Err(err) => {
//             eprintln!("Error embedding directory: {}", err);
//             JsonResponse(serde_json::json!({"status": "error", "error": err.to_string()}))
//         }
//     }
// }
