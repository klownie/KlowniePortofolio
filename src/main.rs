mod templates;

use askama_axum::Template;
use axum::debug_handler;
use axum::extract::{Query, State};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use listenfd::ListenFd;
use log::{debug, info};
use std::fs;
use std::ops::Not;
use std::path::PathBuf;
use std::sync::Arc;
use templates::*;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

const IP: &'static str = "127.0.0.1";
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
        names: vec![String::from("Audrick Yeu"), String::from("Portofolio")],
    });

    let api_router = Router::new()
        .route("/name", get(next_name_handler))
        .route("/fullscreen", get(fullscreen_toggle_handler))
        .with_state(app_state);

    let app = Router::new()
        .nest("/api", api_router)
        .route("/", get(handle_main))
        .nest_service("/assets", ServeDir::new("assets"));

    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => {
            let ip = format!("{}:{}", IP, PORT);
            TcpListener::bind(ip).await.unwrap()
        }
    };

    // run it
    info!("listening on http://{}", listener.local_addr().unwrap());

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

    debug!("{:?}", masonry_images);

    let template = Index {
        indexed: 0,
        name: "Audrick Yeu".into(),
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
    Query(template): Query<InteractiveName>,
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
async fn fullscreen_toggle_handler(Query(template): Query<ToggleFullscreen>) -> impl IntoResponse {
    debug!("{:?}", template);

    let template = ToggleFullscreen {
        fullscreen: template.fullscreen.not(),
        path: template.path,
    };

    let reply = template.render().unwrap();

    // Return the HTML response
    (StatusCode::OK, Html(reply))
}
