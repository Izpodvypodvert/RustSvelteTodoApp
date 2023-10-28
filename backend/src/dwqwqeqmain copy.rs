use axum::{
    extract::{Path, State},
    routing::{get, post},
    Form, Json, Router,
};
use axum::response::Redirect;
use axum_error::Result;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()>{
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    let app = Router::new()
        .route("/", get(list_todo))
        .route("/create", post(create))
        .route("/read/:id", get(read))
        .route("/update", get(update))
        .route("/delete/:id", post(delete))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
        
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}

#[derive(Deserialize)]
struct NewTodo {
    description: String
}

async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>> {
    let todos= sqlx::query_as!(
        Todo,
        "SELECT id, description, done FROM todos ORDER BY id"
    ).fetch_all(&pool)
    .await?;
    Ok(Json(todos))
}

async fn create(State(pool): State<SqlitePool>, Form(todo): Form<NewTodo>) -> Result<Redirect> {
    sqlx::query!(
        "INSERT INTO todos (description) VALUES (?)",
        todo.description,
    )
    .execute(&pool)
    .await?;
    Ok(Redirect::to("http://localhost:5173"))
}

async fn read(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Todo>> {
    let todo = sqlx::query_as!(
        Todo,
        "SELECT id, description, done FROM todos WHERE id = ?",
        id
    )
    .fetch_one(&pool)
    .await?;
    Ok(Json(todo))
}

async fn update(State(pool): State<SqlitePool>, Form(todo): Form<Todo>) -> Result<Redirect> {
    sqlx::query!(
        "UPDATE todos SET description = ?, DONE = ? WHERE id = ?",
        todo.description,
        todo.done,
        todo.id
    ).execute(&pool).await?;
    Ok(Redirect::to("http://localhost:5173"))
}

async fn delete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Redirect> {
    sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&pool)
        .await?;
    Ok(Redirect::to("http://localhost:5173"))
}

