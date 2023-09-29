
use axum::routing::post;
use axum::Router;

use axum::extract::Json;
use sqlx::SqlitePool;
use std::net::SocketAddr;

use std::sync::Arc;

use file::server::api::create_db_table;
use file::server::api::directory_post;
use file::server::api::file_post;
use file::server::api::AppState;
use file::server::api::EmbedReq;



#[tokio::main]
async fn main() {
    
    let pool = SqlitePool::connect("TODO").await.unwrap();
    let shared_state = Arc::new(AppState {
        pool: Arc::new(pool),
    });

    let app = Router::new()
        .route(
            "/create_table",
            post({
                let shared_state = Arc::clone(&shared_state);
                move || create_db_table(shared_state.clone())
            }),
        )
        .route(
            "/embed_file",
            post({
                let shared_state = Arc::clone(&shared_state);
                move |body: Json<EmbedReq>| file_post(body, shared_state.clone())
            }),
        )
        .route(
            "/embed_directory",
            post({
                let shared_state = Arc::clone(&shared_state);
                move |body: Json<EmbedReq>| directory_post(body, shared_state.clone())
            }),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));


    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
