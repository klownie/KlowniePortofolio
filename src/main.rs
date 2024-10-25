mod templates;

use std::iter::Cycle;
use askama_axum::Template;
use axum::debug_handler;
use axum::extract::{Path, Query, State};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,Router,
};
use log::{debug, info};
use std::net::SocketAddr;
use std::slice::Iter;
use std::sync::{Arc, Mutex};
use text_to_ascii_art::to_art;
use templates::*;
use tower_http::services::ServeDir;

const IP: &'static str = "0.0.0.0";
const PORT: &'static str = "8080";

turf::style_sheet!("templates/index.scss");

struct AppState {
    names: Mutex<Cycle<std::vec::IntoIter<String>>>,
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app_state = Arc::new(AppState {
        names: Mutex::new(vec![
            String::from("Portofolio"),
            String::from("Universe"),
            String::from("Audrick Yeu")
        ].into_iter().cycle()),
    });

    let api_router = Router::new()
        .route("/name", get(toggle_name_handler))
        .with_state(app_state);

    let app = Router::new()
        .nest("/api", api_router)
        .route("/", get(handle_main))
        .nest_service("/assets", ServeDir::new("assets"));

    let listen_addr: SocketAddr = format!("{}:{}", IP, PORT).parse().unwrap();

    info!("listening on : http://localhost:{}", PORT);

    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn handle_main() -> impl IntoResponse {

    let template = Index {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}


#[debug_handler]
async fn toggle_name_handler(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Lock the mutex to get access to the iterator
    let mut lock = state.names.lock().unwrap();

    // Call `next` to get the next name from the iterator
    let next_name = lock.next();

    // Format the result depending on whether there's a next value or not
    let response = match next_name {
        Some(name) => format!("<pre>{}</pre>", to_art(name,"",0,0,0).unwrap()),
        None => unreachable!(),
    };

    // Return the HTML response
    (StatusCode::OK, Html(response))
}
