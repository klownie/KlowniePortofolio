use crate::handlers::{
    handle_main, handler_404, project_request_handler,
    resolution_request_handler,
};
use axum::routing::get;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use axum::http::HeaderValue;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use axum::http::header;
use tower_http::{compression::CompressionLayer, decompression::RequestDecompressionLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use axum_prometheus::PrometheusMetricLayer;

pub fn build_routes() -> Router {
    let api_router = Router::new()
        .route("/projects/{project}", get(project_request_handler))
        .route(
            "/toggleres/{project_name}/{high_res}",
            get(resolution_request_handler),
        );

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    let middleware =
        ServiceBuilder::new()
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
            )
            .layer(prometheus_layer);

    let mut app = Router::new()
        .nest("/api", api_router)
        .route("/", get(handle_main))
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(middleware);

    app = app.fallback(handler_404);

    app
}

pub fn generate_expires_header(days_in_future: u64) -> HeaderValue {
    let expiration = SystemTime::now() + Duration::from_secs(days_in_future * 86400); // days to seconds
    let unix_timestamp = expiration.duration_since(UNIX_EPOCH).unwrap().as_secs();
    HeaderValue::from_str(&format!("{}", unix_timestamp)).unwrap()
}
