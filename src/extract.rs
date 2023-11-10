use axum::extract::FromRequest;
use axum::response::IntoResponse;
use serde::Serialize;
use crate::error::Error;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(Error))]
pub struct Json<T>(pub T);


// We implement `IntoResponse` for our extractor so it can be used as a response
impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}