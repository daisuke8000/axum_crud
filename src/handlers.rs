use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

use crate::repositories::{CreateTodo, TodoRepository};

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
