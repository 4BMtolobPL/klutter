use crate::error::AppError;
use axum::{Router, routing::get};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/handler", get(handler))
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn handler() -> Result<(), AppError> {
    try_thing()?;
    Ok(())
}

fn try_thing() -> anyhow::Result<()> {
    anyhow::bail!("it failed!")
}
