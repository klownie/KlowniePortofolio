use crate::errors::AppError;
use crate::templates::*;
use axum::debug_handler;
use axum::extract::Path;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use std::fs;
use std::ops::Not;
use std::sync::LazyLock;

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Ports {
    pub http: u16,
    pub https: u16,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub topper: Topper,
    pub interactive_name: InteractiveName,
    pub masonry: Masonry,
    pub ports: Ports,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let content = fs::read_to_string("server.toml").expect("Failed to read TOML file");
    let config: Config = toml::from_str(&content).expect("Failed to parse TOML file");
    config
});

#[debug_handler]
pub async fn handle_main() -> Result<impl IntoResponse, AppError> {
    let index = Index {};

    let topper = &CONFIG.topper;

    let interactive_name = &CONFIG.interactive_name;

    let socials = Socials {};

    let bio = Bio {};

    let masonry = &CONFIG.masonry;

    // let cv = CurriculumVitae {};

    let reply = format!(
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

    // let cfg = Cfg {
    //     minify_js: true,
    //     ..Cfg::default()
    // };
    // match truncate(&mut reply, &cfg) {
    //     Ok(()) => debug!("js minified"),
    //     Err(Error { position, .. }) => {
    //         error!("minification failed at : {}", position)
    //     }
    // };

    Ok((StatusCode::OK, Html(reply)))
}

pub async fn project_request_handler(Path(project): Path<String>) -> impl IntoResponse {
    let reply = render_project_template(&project, false).await;
    reply
}

pub async fn resolution_request_handler(
    Path((project, high_res)): Path<(String, bool)>,
) -> impl IntoResponse {
    let reply = render_project_template(&project, high_res.not()).await;
    reply
}
