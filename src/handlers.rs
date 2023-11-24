use axum::{async_trait, extract::FromRequest, http::StatusCode, Json, RequestExt};
use hyper::Request;
use serde::de::DeserializeOwned;
use validator::Validate;

pub mod label;
pub mod todo;

#[derive(Debug)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for ValidatedJson<T>
where
    B: Send + 'static,
    S: Send + Sync,
    T: Validate + 'static,
    Json<T>: FromRequest<(), B>,
{
    type Rejection = (StatusCode, &'static str);

    // https://dev.to/ayush1325/validating-json-request-in-axum-2n34
    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = req
            .extract::<Json<T>, _>()
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid JSON body"))?;
        data.validate()
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid JSON body"))?;
        Ok(Self(data))
    }
}
