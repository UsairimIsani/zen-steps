use axum::response::IntoResponse;
use crate::shared::AppState;

pub async fn status(
    axum::extract::State(_state): axum::extract::State<AppState>
) -> impl IntoResponse {
    (
        axum::http::StatusCode::OK,
        format!("Ok {}", std::env::var("PROJECT_NAME").unwrap_or_default()),
    )
}