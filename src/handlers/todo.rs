use crate::repositories::todo::{CreateTodo, TodoRepository, UpdateTodo};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use super::ValidatedJson;
use std::sync::Arc;

// **memo 1**
// https://qiita.com/Sicut_study/items/5e5d6cce5ba48c225367
pub async fn create_todo<T: TodoRepository>(
    // この並びが逆だとコンパイルエラーになる
    Extension(repository): Extension<Arc<T>>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .create(payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).await.or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.all().await.unwrap();
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn update_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .update(id, payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn delete_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    repository
        .delete(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::NOT_FOUND)
}
