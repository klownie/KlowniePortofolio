mod templates;

use askama_axum::Template;
use axum::debug_handler;
use axum::extract::Path;
use axum::{
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use log::{debug, error, info, trace, warn};
use std::net::SocketAddr;
use std::str::FromStr;
use templates::*;
use tower_http::services::{ServeDir, ServeFile};

const IP: &'static str = "0.0.0.0";
const PORT: &'static str = "8080";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/", get(handle_main))
        .route("/counter/:count", get(handle_counter)) // Added counter route
        .route("/name/:name", get(toggle_name_handler)) // Added name route
        .nest_service("/assets", ServeDir::new("assets"));

    let listen_addr: SocketAddr = format!("{}:{}", IP, PORT).parse().unwrap();

    info!("listening on : {}", listen_addr);

    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn handle_main() -> impl IntoResponse {
    let template = Index {
        count: 0,
        name: ToggleName::Normal(String::from("Audrick Yeu")),
    };

    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

// Handle /counter/:count route
#[debug_handler]
async fn handle_counter(Path(mut count): Path<u64>) -> impl IntoResponse {
    count = count + 1;
    let template = TestCounter { count };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

// Handle /name/:name route
#[debug_handler]
async fn toggle_name_handler(Path(name): Path<String>) -> impl IntoResponse {
    debug!("{:}", name);
    let toggle_name: ToggleName = ToggleName::from_str(&name).unwrap().toggle(); // Make sure to handle errors

    let template = InteractiveName { name: toggle_name };

    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
