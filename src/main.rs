mod templates;

use templates::*;
use askama_axum::Template;
use axum::debug_handler;
use axum::extract::Path;
use axum::{
    http::{StatusCode, header, HeaderMap},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use log::{debug, error, info, trace, warn};
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

const IP: &'static str = "0.0.0.0";
const PORT: &'static str = "8080";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(handle_main))
        .route("/counter/:count", get(handle_counter))  // Added counter route
        .route("/name/:name", get(toggle_name_handler))  // Added name route
        .route("/_assets/*path", get(handle_assets));

    let listen_addr: SocketAddr = format!("{}:{}", IP, PORT).parse().unwrap();

    info!("listening on : {}", listen_addr);

    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

static FAVICON: &str = include_str!("../assets/favicon.svg");

#[debug_handler]
async fn handle_main() -> impl IntoResponse {



    let template = Index {
        count : 0,
        name : ToggleName::Normal(String::from("Audrick Yeu")),
    };

    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

// Handle /counter/:count route
#[debug_handler]
async fn handle_counter(Path(count): Path<u64>) -> impl IntoResponse {
    let template = TestCounter { count };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

// Handle /name/:name route
#[debug_handler]
async fn toggle_name_handler(Path(name): Path<ToggleName>) -> impl IntoResponse {
    // Toggle the name based on the current state
    let toggled_name = name.toggle();

    let template = InteractiveName {
        name: toggled_name,
    };

    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
#[debug_handler]
async fn handle_assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if path == "favicon.svg" {
        (StatusCode::OK, headers, FAVICON)
    } else {
        (StatusCode::NOT_FOUND, headers, "")
    }
}