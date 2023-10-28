use axum::{
    extract::Path,
    Json, Form, Extension,
    response::Redirect,
    http::StatusCode,
};
use std::sync::Arc;
use axum_error::Result;
use sqlx::sqlite::SqlitePool;
use crate::models::{Todo, NewTodo};

pub async fn list_todo(Extension(pool): Extension<Arc<SqlitePool>>) -> Result<Json<Vec<Todo>>> {
    let todos= sqlx::query_as!(
        Todo,
        "SELECT id, description, done FROM todos ORDER BY id"
    ).fetch_all(&*pool)
    .await?;
    Ok(Json(todos))
}

pub async fn create_todo(Extension(pool): Extension<Arc<SqlitePool>>, Form(todo): Form<NewTodo>) -> Result<Redirect> {
    sqlx::query!(
        "INSERT INTO todos (description) VALUES (?)",
        todo.description,
    )
    .execute(&*pool)
    .await?;
    Ok(Redirect::to("http://localhost:5173"))
}

pub async fn read_todo(Extension(pool): Extension<Arc<SqlitePool>>, Path(id): Path<i64>) -> Result<Json<Todo>> {
    let todo = sqlx::query_as!(
        Todo,
        "SELECT id, description, done FROM todos WHERE id = ?",
        id
    )
    .fetch_one(&*pool)
    .await?;
    Ok(Json(todo))
}

// pub async fn update_todo(Extension(pool): Extension<Arc<SqlitePool>>, Form(todo): Form<Todo>) -> Result<Redirect> {
//     sqlx::query!(
//         "UPDATE todos SET description = ?, DONE = ? WHERE id = ?",
//         todo.description,
//         todo.done,
//         todo.id
//     ).execute(&*pool).await?;
//     Ok(Redirect::to("http://localhost:5173"))
// }

pub async fn update_todo(
    Extension(pool): Extension<Arc<SqlitePool>>,
    Json(todo): Json<Todo>,
) -> Result<Redirect, StatusCode> {
    let result = sqlx::query!(
        "UPDATE todos SET description = ?, done = ? WHERE id = ?",
        todo.description,
        todo.done,
        todo.id
    )
    .execute(&*pool)
    .await;

    match result {
        Ok(_) => Ok(Redirect::to("http://localhost:5173")),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_todo(Extension(pool): Extension<Arc<SqlitePool>>, Path(id): Path<i64>) -> Result<Redirect> {
    sqlx::query!("DELETE FROM todos WHERE id = ?", id)
    .execute(&*pool)
    .await?;
    Ok(Redirect::to("http://localhost:5173"))
}