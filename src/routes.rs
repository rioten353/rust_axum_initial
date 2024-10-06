use crate::handler::hello_world;
use axum::routing::{get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(hello_world))
}
