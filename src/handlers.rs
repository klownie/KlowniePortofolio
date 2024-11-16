use crate::templates::*;
use askama_axum::Template;
use axum::debug_handler;
use axum::extract::{ConnectInfo, Path};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use axum_extra::extract::Query;
use log::{debug, error, info};
use serde::Deserialize;
use std::fs;
use std::net::SocketAddr;
use std::ops::Not;
use std::sync::{Arc, LazyLock, Mutex};

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Ports {
    pub http: u16,
    pub https: u16,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub interactive_name: InteractiveName,
    pub masonry: Masonry,
    pub ports: Ports,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let content = fs::read_to_string("config.toml").expect("Failed to read TOML file");
    let config: Config = toml::from_str(&content).expect("Failed to parse TOML file");
    config
});

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

    let index = Index {};

    let topper = Topper {};

    let interactive_name = &CONFIG.interactive_name;

    let socials = Socials {};

    let bio = Bio {};

    let masonry = &CONFIG.masonry;

    let reply = format!(
        "\
    {index}\
    {topper}
    {interactive_name}\
    {socials}\
    {bio}\
    {masonry}\
    "
    );

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
    use Project::*;

    let template = match project {
        "SamuConceptIllustration" => SamuConceptIllustration {
            project_name: project.into(),
            high_res,
        },
        "SamuConceptCharacter" => SamuConceptCharacter {
            project_name: project.into(),
            high_res,
        },
        "Saint-John" => SaintJohn {
            project_name: project.into(),
            high_res,
        },
        "HomardRojas" => HomardRojas {
            project_name: project.into(),
            high_res,
        },
        "CarbonixWorkerSuit" => CarbonixWorkerSuit {
            project_name: project.into(),
            high_res,
        },
        "ClimbingExoSuit" => ClimbingExoSuit {
            project_name: project.into(),
            high_res,
        },
        "ClimbingExoSuit3d" => ClimbingExoSuit3d {
            project_name: project.into(),
            high_res,
        },
        "Intru" => Intru {
            project_name: project.into(),
            high_res,
        },
        "UmbrellaKnight" => UmbrellaKnight {
            project_name: project.into(),
            high_res,
        },
        "TeamBlue" => TeamBlue {
            project_name: project.into(),
            high_res,
        },
        "TribalYellowDemon" => TribalYellowDemon {
            project_name: project.into(),
            high_res,
        },
        "UrbanWhiteCrowMan" => UrbanWhiteCrowMan {
            project_name: project.into(),
            high_res,
        },
        _ => {
            error!("Project not found, rendering MissingProject");
            MissingFile
        }
    };

    info!("Rendering project: {}", project);
    template.render().unwrap()
}
