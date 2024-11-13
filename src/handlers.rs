
use crate::templates::*;
use askama_axum::Template;
use axum::debug_handler;
use axum::extract::{ConnectInfo, Path, Query};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use log::{debug, error, info};
use std::net::SocketAddr;
use std::ops::Not;
use std::sync::{Arc, LazyLock, Mutex};

pub const PROJECTS: &[&str] = &[
    "SamuConceptCharacter",
    "Saint-John",
    "HomardRojas",
    "CarbonixWorkerSuit",
    "ClimbingExoSuit",
    "Intru",
    "UmbrellaKnight",
    "ClimbingExoSuit3d",
    "TeamBlue",
    "TribalYellowDemon",
    "UrbanWhiteCrowMan",
];
pub static VISITORS: LazyLock<Arc<Mutex<Vec<String>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));

pub async fn handler_404() -> impl IntoResponse {
    let template = Error404 {};
    let reply = template.render().unwrap();
    (StatusCode::NOT_FOUND, Html(reply))
}

#[debug_handler]
pub async fn handle_main(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {

    let visitors_clone = Arc::clone(&VISITORS);
    let mut visitors = visitors_clone.lock().unwrap();
    if visitors.contains(&addr.to_string()) {
        debug!("{addr} refreshed the page")
    } else {
        visitors.push(addr.to_string());
        info!("{addr} is visiting");
    }

    let masonry_projects = PROJECTS;

    let template = Index {
        titles: Vec::from(["Audrick".into(), "Yeu".into(), "Portofolio".into(), "Concept".into(), "Art".into()]),
        fullscreen: false,
        masonry: masonry_projects.iter().map(|&s| s.to_string()).collect(),
        project: "".into(),
    }
    .render()
    .unwrap();

    let reply: String = template;

    (StatusCode::OK, Html(reply))
}

#[debug_handler]
pub async fn fullscreen_toggle_handler(
    Query(template): Query<ToggleFullscreen>,
) -> impl IntoResponse {
    let new_template = ToggleFullscreen {
        fullscreen: template.fullscreen.not(),
        project: template.project,
    };

    let reply = new_template.render().unwrap();

    // Return the HTML response
    (StatusCode::OK, Html(reply))
}

pub async fn project_request_handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(project): Path<String>,
) -> impl IntoResponse {
    info!("{} has requested {}", addr, &project);
    let reply = render_project_template(&project, false).await;
    (StatusCode::OK, Html(reply))
}

pub async fn resolution_request_handler(
    Path((project, high_res)): Path<(String, bool)>,
) -> impl IntoResponse {
    let reply = render_project_template(&project, high_res.not()).await;
    (StatusCode::OK, Html(reply))
}

pub async fn render_project_template(project: &str, high_res: bool) -> String {
    let rendered_template = match project {
        "SamuConceptCharacter" => {
            info!("loaded: SamuConceptCharacter");
            Project::SamuConceptCharacter {
                project_name: project.into(),
                high_res,
            }
        }
        "Saint-John" => {
            info!("loaded: Saint-John");
            Project::SaintJohn {
                project_name: project.into(),
                high_res,
            }
        }
        "HomardRojas" => {
            info!("loaded: HomardRojas");
            Project::HomardRojas {
                project_name: project.into(),
                high_res,
            }
        }
        "CarbonixWorkerSuit" => {
            info!("loaded: CarbonixWorkerSuit");
            Project::CarbonixWorkerSuit {
                project_name: project.into(),
                high_res,
            }
        }
        "ClimbingExoSuit" => {
            info!("loaded: ClimbingExoSuit");
            Project::ClimbingExoSuit {
                project_name: project.into(),
                high_res,
            }
        }
        "ClimbingExoSuit3d" => {
            info!("loaded: ClimbingExoSuit3d");
            Project::ClimbingExoSuit3d {
                project_name: project.into(),
                high_res,
            }
        }
        "Intru" => {
            info!("loaded: Intru");
            Project::Intru {
                project_name: project.into(),
                high_res,
            }
        }
        "UmbrellaKnight" => {
            info!("loaded: UmbrellaKnight");
            Project::UmbrellaKnight {
                project_name: project.into(),
                high_res,
            }
        }
        "TeamBlue" => {
            info!("loaded: TeamBlue");
            Project::TeamBlue {
                project_name: project.into(),
                high_res,
            }
        }
        "TribalYellowDemon" => {
            info!("loaded: TribalYellowDemon");
            Project::TribalYellowDemon {
                project_name: project.into(),
                high_res,
            }
        }
        "UrbanWhiteCrowMan" => {
            info!("loaded: UrbanWhiteCrowMan");
            Project::UrbanWhiteCrowMan {
                project_name: project.into(),
                high_res,
            }
        }
        _ => {
            error!("loaded: MissingProject");
            Project::MissingProject
        }
    };

    rendered_template.render().unwrap()
}
