use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::repositories::{CreateTodo, TodoRepository, UpdateTodo};

// **memo 1**
// https://qiita.com/Sicut_study/items/5e5d6cce5ba48c225367
pub async fn create_todo<T: TodoRepository>(
    // この並びが逆だとコンパイルエラーになる
    Extension(repository): Extension<Arc<T>>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let todo = repository.create(payload);

    (StatusCode::CREATED, Json(todo))
}

pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    todo!();
    Ok(StatusCode::OK)
}

pub async fn all_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    todo!()
}

pub async fn update_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    todo!();
    Ok(StatusCode::OK)
}

pub async fn delete_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    todo!()
}
