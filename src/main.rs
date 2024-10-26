mod templates;

use askama_axum::Template;
use axum::extract::{Path, Query, State};
use axum::{debug_handler, Form};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use log::{debug, info};
use serde_qs;
use std::iter::Cycle;
use std::net::SocketAddr;
use std::ops::Not;
use std::slice::Iter;
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::PathBuf;
use templates::*;
use text_to_ascii_art::to_art;
use tower_http::services::ServeDir;
use tracing::field::debug;

const IP: &'static str = "0.0.0.0";
const PORT: &'static str = "3000";

turf::style_sheet!("templates/index.scss");

struct AppState {
    names: Vec<String>,
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app_state = Arc::new(AppState {
        names: vec![
            String::from("CD"),
            String::from("FUCK U RETARD"),
            String::from("SUCK MY DICK"),
            String::from("Fagot"),
            String::from("I miss u papa"),
            String::from("He went to get the milk"),
        ],
    });

    let api_router = Router::new()
        .route("/name", get(next_name_handler))
        .route("/fullscreen", get(fullscreen_toggle_handler))
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

    // Specify the path to your assets directory
    let assets_dir = PathBuf::from("assets/gallery");

    // Create a vector to hold the image paths
    let mut masonry_images = Vec::new();

    // Read the directory and generate paths
    if let Ok(entries) = fs::read_dir(assets_dir) {
        for entry in entries.filter_map(Result::ok) {
            if let Some(path) = entry.path().to_str() {
                // Filter for image files (you can expand this as needed)
                if path.ends_with(".jpg") || path.ends_with(".png") {
                    masonry_images.push(path.into());
                }
            }
        }
    }

    debug!("{:?}",masonry_images);

    let template = Index {
        indexed: 0,
        name: "DVD".into(),
        fullscreen: false,
        masonry: masonry_images,
        path: "".into(),
    };
    let reply = template.render().unwrap();
    (StatusCode::OK, Html(reply).into_response())
}

#[debug_handler]
async fn next_name_handler(
    State(state): State<Arc<AppState>>,
    Query(mut template): Query<InteractiveName>,
) -> impl IntoResponse {
    debug!("{:?}", template);

    let index = (template.indexed + 1) % state.names.len();

    let template = InteractiveName {
        indexed: index,
        name: state.names[index].clone(),
    };

    let reply = template.render().unwrap();

    // Return the HTML response
    (StatusCode::OK, Html(reply))
}

#[debug_handler]
async fn fullscreen_toggle_handler(
    State(state): State<Arc<AppState>>,
    Query(mut template): Query<ToggleFullscreen>,
) -> impl IntoResponse {
    debug!("{:?}", template);

    let template = ToggleFullscreen {
        fullscreen: template.fullscreen.not(),
        path: template.path,
    };

    let reply = template.render().unwrap();

    // Return the HTML response
    (StatusCode::OK, Html(reply))
}
