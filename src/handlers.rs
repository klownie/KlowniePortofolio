use crate::templates::*;
use askama_axum::Template;
use axum::debug_handler;
use axum::extract::{ConnectInfo, Path};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use log::{debug, error, info};
use minify_html_onepass::{truncate, Cfg, Error};
use serde::Deserialize;
use std::fs;
use std::net::SocketAddr;
use std::ops::Not;
use std::sync::LazyLock;
use uiua::*;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Ports {
    pub http: u16,
    pub https: u16,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    topper: Topper,
    pub interactive_name: InteractiveName,
    pub masonry: Masonry,
    pub ports: Ports,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let content = fs::read_to_string("server.toml").expect("Failed to read TOML file");
    let config: Config = toml::from_str(&content).expect("Failed to parse TOML file");
    config
});

pub async fn handler_404() -> impl IntoResponse {
    let topper = &CONFIG.topper;
    let error = Error404 {};
    let reply = format!(
        "\
        {topper}
        {error}
        "
    );
    (StatusCode::NOT_FOUND, Html(reply))
}

#[debug_handler]
pub async fn handle_main() -> impl IntoResponse {
    let new_uuid = Uuid::new_v4().to_string();
    info!{"created {new_uuid}"};
    let mut uiua = Uiua::with_native_sys()
        .with_recursion_limit(100);
    uiua.push(new_uuid);
    uiua.run_file(std::path::Path::new("uiua/manager.ua")).unwrap();

    let index = Index {};

    let topper = &CONFIG.topper;

    let interactive_name = &CONFIG.interactive_name;

    let socials = Socials {};

    let bio = Bio {};

    let masonry = &CONFIG.masonry;

    let mut reply = format!(
        "\
    {index}\
    {topper}
    {interactive_name}\
    {socials}\
    {bio}\
    {masonry}\
    "
    )
    .into_bytes();

    let cfg = Cfg {
        minify_js: true,
        ..Cfg::default()
    };
    match truncate(&mut reply, &cfg) {
        Ok(()) => debug!("js minified"),
        Err(Error { position, .. }) => {
            error!("minification failed at : {}", position)
        }
    };

    (StatusCode::OK, Html(reply))
}

pub async fn project_request_handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(project): Path<String>,
) -> impl IntoResponse {
    debug!("{} has requested {}", addr, &project);
    let reply = render_project_template(&project, false).await;
    reply
}

pub async fn resolution_request_handler(
    Path((project, high_res)): Path<(String, bool)>,
) -> impl IntoResponse {
    let reply = render_project_template(&project, high_res.not()).await;
    reply
}

pub async fn render_project_template(project: &str, high_res: bool) -> (StatusCode, Html<String>) {
    use Project::*;

    let template = match project {
        "SamuConceptIllustration" => SamuConceptIllustration {
            project: project.into(),
            high_res,
        },
        "SamuConceptCharacter" => SamuConceptCharacter {
            project: project.into(),
            high_res,
        },
        "Saint-John" => SaintJohn {
            project: project.into(),
            high_res,
        },
        "HomardRojas" => HomardRojas {
            project: project.into(),
            high_res,
        },
        "CarbonixWorkerSuit" => CarbonixWorkerSuit {
            project: project.into(),
            high_res,
        },
        "ClimbingExoSuit" => ClimbingExoSuit {
            project: project.into(),
            high_res,
        },
        "ClimbingExoSuit3d" => ClimbingExoSuit3d {
            project: project.into(),
            high_res,
        },
        "Intru" => Intru {
            project: project.into(),
            high_res,
        },
        "UmbrellaKnight" => UmbrellaKnight {
            project: project.into(),
            high_res,
        },
        "TeamBlue" => TeamBlue {
            project: project.into(),
            high_res,
        },
        "TribalYellowDemon" => TribalYellowDemon {
            project: project.into(),
            high_res,
        },
        "UrbanWhiteCrowMan" => UrbanWhiteCrowMan {
            project: project.into(),
            high_res,
        },
        _ => {
            error!("no project named {project} was found");
            return (StatusCode::NOT_FOUND, Html(Error404 {}.render().unwrap()));
        }
    };

    debug!("Rendering project: {}", project);
    (StatusCode::OK, Html(template.render().unwrap()))
}
