use crate::handlers::{
    handle_main, handler_404, project_request_handler,
    resolution_request_handler,
};
use axum::routing::get;
use axum::Router;

pub fn build_routes() -> Router {
    let api_router = Router::new()
        .route("/projects/{project}", get(project_request_handler))
        .route(
            "/toggleres/{project_name}/{high_res}",
            get(resolution_request_handler),
        );

    let mut app = Router::new()
        .nest("/api", api_router)
        .route("/", get(handle_main));

    app = app.fallback(handler_404);

    app
}
