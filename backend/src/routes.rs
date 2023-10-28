use axum::{Router, routing::{get, post, delete, put}};
use crate::controllers;

pub fn router() -> Router {
    Router::new()
        .route("/", get(controllers::list_todo))
        .route("/create", post(controllers::create_todo))
        .route("/read/:id", get(controllers::read_todo))
        .route("/update", put(controllers::update_todo))
        .route("/delete/:id", delete(controllers::delete_todo))
}