use crate::repositories::{CreateTodo, TodoRepository};
use axum::{extract::Extension, response::IntoResponse, Json};
use hyper::StatusCode;
use std::sync::Arc;

pub async fn create_todo<T: TodoRepository>(
    Json(payload): Json<CreateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let create = repository.create(payload);
    let todo = create;

    (StatusCode::CREATED, Json(todo))
}
