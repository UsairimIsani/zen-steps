mod open_ai;
mod status;

use crate::routes::open_ai::{open_ai, open_ai_text_to_speech};
use crate::routes::status::status;
use crate::shared::AppState;
use axum::routing::{get, post};
use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub fn public(state: AppState) -> Router {
    let cors = tower_http::cors::CorsLayer::permissive();

    Router::new()
        .route("/status", get(status))
        .route("/openai", post(open_ai))
        .route("/speech", post(open_ai_text_to_speech))
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors),
        )
}
