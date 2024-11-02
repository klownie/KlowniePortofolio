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
use log::{debug, error, info};
use std::net::SocketAddr;
use std::ops::Not;
use templates::*;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

const IP: &'static str = "0.0.0.0";
const PORT: &'static str = "3000";

const TITLE_NAMES: &[&str] = &["Audrick Yeu", "Portofolio"];

const PROJECTS: &[&str] = &[
    "SamuConceptCharacter",
    "Saint-John",
    "HomardRojas",
    "CarbonixWorkerSuit",
    "ClimbingExoSuit",
    "Intru",
    "ClimbingExoSuit3d",
    "TeamBlue",
    "TribalYellowDemon",
    "UrbanWhiteCrowMan",
];

#[tokio::main]
async fn main() {
    turf::style_sheet_values!("scss/index.scss");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let api_router = Router::new()
        .route("/name", get(next_name_handler))
        .route("/fullscreen", get(fullscreen_toggle_handler))
        .route("/projects/:project", get(project_request_handler))
        .route("/toggleres/:project_name/:high_res", get(resolution_request_handler));

    let app = Router::new()
        .nest("/api", api_router)
        .route("/", get(handle_main))
        .nest_service("/assets", ServeDir::new("assets"));

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(handler_404);

    let listen_addr: SocketAddr = format!("{}:{}", IP, PORT).parse().unwrap();

    info!("Listening on http://{}", listen_addr);

    let listener = TcpListener::bind(listen_addr).await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
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
async fn next_name_handler(Query(template): Query<InteractiveName>) -> impl IntoResponse {
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

async fn project_request_handler(Path(project): Path<String>) -> impl IntoResponse {
    let reply = render_project_template(&project, false);
    (StatusCode::OK, Html(reply))
}

async fn resolution_request_handler(Path((project, high_res)): Path<(String, bool)>) -> impl IntoResponse {
    let reply = render_project_template(&project, high_res.not());
    (StatusCode::OK, Html(reply))
}

fn render_project_template(project: &str, high_res: bool) -> String {
    match project {
        "SamuConceptCharacter" => {
            debug!("loaded: SamuConceptCharacter");
            SamuConceptCharacter {
                project_name: project.into(),
                high_res,
            }.render().unwrap()
        }
        "Saint-John" => {
            debug!("loaded: Saint-John");
            SaintJohn {
                project_name: project.into(),
                high_res,
            }.render().unwrap()
        }
        "HomardRojas" => {
            debug!("loaded: HomardRojas");
            HomardRojas {
                project_name: project.into(),
                high_res,
            }
                .render()
                .unwrap()
        }
        "CarbonixWorkerSuit" => {
            debug!("loaded: CarbonixWorkerSuit");
            CarbonixWorkerSuit {
                project_name: project.into(),
                high_res,
            }
                .render()
                .unwrap()
        }
        "ClimbingExoSuit" => {
            debug!("loaded: ClimbingExoSuit");
            ClimbingExoSuit {
                project_name: project.into(),
                high_res,
            }
                .render()
                .unwrap()
        }
        "ClimbingExoSuit3d" => {
            debug!("loaded: ClimbingExoSuit3d");
            ClimbingExoSuit3d {
                project_name: project.into(),
                high_res,
            }.render().unwrap()
        }
        "Intru" => {
            debug!("loaded: Intru");
            Intru {
                project_name: project.into(),
                high_res,
            }
                .render()
                .unwrap()
        }
        "TeamBlue" => {
            debug!("loaded: TeamBlue");
            TeamBlue {
                project_name: project.into(),
                high_res,
            }.render().unwrap()
        }
        "TribalYellowDemon" => {
            debug!("loaded: TribalYellowDemon");
            TribalYellowDemon {
                project_name: project.into(),
                high_res,
            }.render().unwrap()
        }
        "UrbanWhiteCrowMan" => {
            debug!("loaded: UrbanWhiteCrowMan");
            UrbanWhiteCrowMan {
                project_name: project.into(),
                high_res,
            }.render().unwrap()
        }
        _ => {
            error!("loaded: MissingProject");
            MissingProject {}.render().unwrap()
        }
    }
}