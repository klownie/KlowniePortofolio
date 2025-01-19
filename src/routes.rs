use crate::errors::handler_404;
use crate::handlers::{handle_main, project_request_handler, resolution_request_handler, db_handle, log_message};
use axum::http::header;
use axum::http::HeaderValue;
use axum::routing::{get, post};
use axum::Router;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::{compression::CompressionLayer, decompression::RequestDecompressionLayer};

pub fn build_routes() -> Router {
    let api_router = Router::new()
        .route("/projects/{project}", get(project_request_handler))
        .route(
            "/toggleres/{project_name}/{high_res}",
            get(resolution_request_handler),
        )
        .route("/db/{action}", post(db_handle))
        .route("/log/{message}", post(log_message));

    let middleware = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::if_not_present(
            header::EXPIRES,
            generate_expires_header(7),
        ))
        .layer(
            RequestDecompressionLayer::new()
                .br(true)
                .deflate(true)
                .gzip(true)
                .zstd(true),
        )
        .layer(
            CompressionLayer::new()
                .br(true)
                .deflate(true)
                .gzip(true)
                .zstd(true),
        );

    let mut app = Router::new()
        .nest("/api", api_router)
        .route("/", get(handle_main))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(middleware);

    app = app.fallback(handler_404);

    app
}

pub fn generate_expires_header(days_in_future: u64) -> HeaderValue {
    let expiration = SystemTime::now() + Duration::from_secs(days_in_future * 86400); // days to seconds
    let unix_timestamp = expiration.duration_since(UNIX_EPOCH).unwrap().as_secs();
    HeaderValue::from_str(&format!("{}", unix_timestamp)).unwrap()
}
