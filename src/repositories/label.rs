use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

// #[async_trait]
// pub trait LabelRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
//     async fn create(&self, name: String) -> anyhow::Result<Label>;
// }