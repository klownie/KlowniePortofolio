mod templates;

use askama_axum::Template;
use axum::debug_handler;
use axum::extract::{ConnectInfo, Path, Query};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use listenfd::ListenFd;
use log::{debug, error, info};
use std::ops::Not;
use templates::*;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use std::net::SocketAddr;

const IP: &'static str = "0.0.0.0";
const PORT: &'static str = "3000";

const TITLE_NAMES: &[&str] = &["Audrick Yeu","Portofolio"];

const PROJECTS: &[&str] = &["SamuConceptCharacter","Saint-John","HomardRojas","CarbonixWorkerSuit","ClimbingExoSuit","Intru","ClimbingExoSuit3d","TeamBlue","TribalYellowDemon","UrbanWhiteCrowMan"];

#[tokio::main]
async fn main() {

    turf::style_sheet_values!("scss/index.scss");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let api_router = Router::new()
        .route("/name", get(next_name_handler))
        .route("/fullscreen", get(fullscreen_toggle_handler))
        .route("/projects/:project", get(project_request_handler));

    let app = Router::new()
        .nest("/api", api_router)
        .route("/", get(handle_main))
        .nest_service("/assets", ServeDir::new("assets"));

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(handler_404);

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

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}


async fn handler_404() -> impl IntoResponse {

    let template = Error404 {};
    let reply = template.render().unwrap();
    (StatusCode::NOT_FOUND, Html(reply))
}

#[debug_handler]
async fn handle_main(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {

    info!("{addr} is visiting");
    let masonry_projects = PROJECTS;

    let template = Index {
        indexed: 0,
        name: TITLE_NAMES[0].into(),
        fullscreen: false,
        masonry: masonry_projects.iter().map(|&s| s.to_string()).collect(),
        project: "".into(),
    };
    let reply = template.render().unwrap();
    (StatusCode::OK, Html(reply).into_response())
}

#[debug_handler]
async fn next_name_handler(
    Query(template): Query<InteractiveName>,
) -> impl IntoResponse {

    let index = (template.indexed + 1) % TITLE_NAMES.len();

    let template = InteractiveName {
        indexed: index,
        name: TITLE_NAMES[index].to_string(),
    };

    let reply = template.render().unwrap();

    // Return the HTML response
    (StatusCode::OK, Html(reply))
}

#[debug_handler]
async fn fullscreen_toggle_handler(Query(template): Query<ToggleFullscreen>) -> impl IntoResponse {

    let new_template = ToggleFullscreen {
        fullscreen: template.fullscreen.not(),
        project: template.project,
    };

    let reply = new_template.render().unwrap();

    // Return the HTML response
    (StatusCode::OK, Html(reply))
}

async fn project_request_handler(Path(project):Path<String>) -> impl IntoResponse {
    debug!("loading project : {}",project);
    let reply = match project.as_str()  {
        "SamuConceptCharacter" => {
            let reply_template = SamuConceptCharacter{};
            debug!("loaded : {}","SamuConceptCharacter");
            reply_template.render().unwrap()
        }
        "Saint-John" => {
            let reply_template = SaintJohn{};
            debug!("loaded : {}","Saint-John");
            reply_template.render().unwrap()
        }
        "HomardRojas" => {
            let reply_template = HomardRojas{};
            debug!("loaded : {}","HomardRojas");
            reply_template.render().unwrap()
        }
        "CarbonixWorkerSuit" => {
            let reply_template = CarbonixWorkerSuit{};
            debug!("loaded : {}","CarbonixWorkerSuit");
            reply_template.render().unwrap()
        }
        "ClimbingExoSuit" => {
            let reply_template = ClimbingExoSuit{};
            debug!("loaded : {}","ClimbingExoSuit");
            reply_template.render().unwrap()
        }
        "ClimbingExoSuit3d" => {
            let reply_template = ClimbingExoSuit3d{};
            debug!("loaded : {}","ClimbingExoSuit3d");
            reply_template.render().unwrap()
        }
        "Intru" => {
            let reply_template = Intru{};
            debug!("loaded : {}","Intru");
            reply_template.render().unwrap()
        }
        "TeamBlue" => {
            let reply_template = TeamBlue{};
            debug!("loaded : {}","TeamBlue");
            reply_template.render().unwrap()
        }
        "TribalYellowDemon" => {
            let reply_template = TribalYellowDemon{};
            debug!("loaded : {}","TribalYellowDemon");
            reply_template.render().unwrap()
        }
        "UrbanWhiteCrowMan" => {
            let reply_template = UrbanWhiteCrowMan{};
            debug!("loaded : {}","UrbanWhiteCrowMan");
            reply_template.render().unwrap()
        }
        _ => {
            let reply_template = MissingProject{};
            error!("loaded : {}","MissingProject");
            reply_template.render().unwrap()
        }
    };

    (StatusCode::OK, Html(reply))
}