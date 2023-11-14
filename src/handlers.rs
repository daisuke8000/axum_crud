use axum::{
    async_trait,
    extract::{Extension, FromRequest, Path},
    http::StatusCode,
    response::IntoResponse,
    Json, RequestExt,
};
use hyper::Request;
use std::sync::Arc;
use validator::{Validate};

use crate::repositories::{CreateTodo, TodoRepository, UpdateTodo};

#[derive(Debug)]
pub struct ValidatedJson<J>(pub J);

// #[async_trait]
// impl<S, B, J> FromRequest<S, B> for ValidatedJson<J>
// where
//     B: Send + 'static,
//     S: Send + Sync,
//     J: Validate + 'static,
//     Json<J>: FromRequest<(), B>,
// {
//     type Rejection = (StatusCode, &'static str);
//
//     // https://dev.to/ayush1325/validating-json-request-in-axum-2n34
//     async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
//         let Json(value) = req.extract::<Json<J>, _>().await.map_err(|_| {
//             let message = format!("Json parse error: [{}]", "hoge".to_string());
//             (StatusCode::BAD_REQUEST, message)
//         })?;
//
//         value.validate().map_err(|rejection| {
//             let message = format!("Validation error: [{}]", rejection).replace('\n', ", ");
//             (StatusCode::BAD_REQUEST, message)
//         })?;
//         Ok(ValidatedJson(value))
//     }
// }

// **memo 1**
// https://qiita.com/Sicut_study/items/5e5d6cce5ba48c225367
pub async fn create_todo<T: TodoRepository>(
    // この並びが逆だとコンパイルエラーになる
    Extension(repository): Extension<Arc<T>>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> impl IntoResponse {
    let todo = repository.create(payload);

    (StatusCode::CREATED, Json(todo))
}

pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todo = repository.all();
    (StatusCode::OK, Json(todo))
}

pub async fn update_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .update(id, payload)
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn delete_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    repository
        .delete(id)
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::NOT_FOUND)
}
