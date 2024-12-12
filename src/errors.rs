use crate::handlers::CONFIG;
use crate::templates::{Error404, Error500};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

pub async fn handler_404() -> Result<impl IntoResponse, AppError> {
    let topper = &CONFIG.topper;
    let error = Error404 {};
    let reply = format!(
        "\
        {topper}
        {error}
        "
    );
    Ok((StatusCode::NOT_FOUND, Html(reply)))
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!(
                "{}",
                Error500 {
                    err: self.0.to_string()
                }
            ),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
