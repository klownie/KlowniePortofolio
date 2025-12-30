use crate::errors::handler_404;
use crate::handlers::{handle_main, project_request_handler, resolution_request_handler};
use axum::routing::get;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::{compression::CompressionLayer, decompression::RequestDecompressionLayer};

pub fn build_routes() -> Router {
    let api_router = Router::new()
        .route("/projects/{project}", get(project_request_handler))
        .route(
            "/toggleres/{project_name}/{high_res}",
            get(resolution_request_handler),
        );

    let middleware = ServiceBuilder::new()
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
