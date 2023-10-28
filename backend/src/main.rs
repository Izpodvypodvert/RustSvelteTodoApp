use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use axum::{Server, Router, Extension};
use axum_error::Result;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;

mod controllers;
mod models;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;
    let arc_pool = Arc::new(pool);  // Оборачиваем в Arc

    let app = Router::new()
        .merge(routes::router())  // Предполагаем, что routes::router возвращает Router
        .layer(Extension(arc_pool))  // Добавляем Arc<SqlitePool> как Extension
        .layer(CorsLayer::very_permissive());

    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    Ok(Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}




