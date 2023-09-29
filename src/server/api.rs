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
