mod open_ai;
mod status;

use crate::routes::open_ai::{open_ai, open_ai_text_to_speech};
use crate::routes::status::status;
use crate::shared::AppState;
use axum::http::header::{ACCEPT, ACCEPT_LANGUAGE, AUTHORIZATION, CONTENT_LANGUAGE, CONTENT_TYPE};
use axum::http::Method;
use axum::routing::{get, post};
use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub fn public(state: AppState) -> Router {
    let cors = tower_http::cors::CorsLayer::new()
        //     .allow_methods(vec![
        //         Method::GET,
        //         Method::POST,
        //         Method::PATCH,
        //         Method::DELETE,
        //     ])
        //     .allow_headers(vec![
        //         ACCEPT,
        //         ACCEPT_LANGUAGE,
        //         AUTHORIZATION,
        //         CONTENT_LANGUAGE,
        //         CONTENT_TYPE,
        //     ])
        .allow_origin("*".parse::<axum::http::HeaderValue>().unwrap());
    Router::new()
        .route("/status", get(status))
        .route("/openai", post(open_ai))
        .route("/speech", post(open_ai_text_to_speech))
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(cors)
                .layer(TraceLayer::new_for_http()),
        )
}
