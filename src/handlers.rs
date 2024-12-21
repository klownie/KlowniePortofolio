use crate::errors::AppError;
use crate::templates::*;
use crate::uiua;
use askama_axum::Template;
use axum::debug_handler;
use axum::extract::{ConnectInfo, Path};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use minify_html_onepass::{truncate, Cfg, Error};
use serde::Deserialize;
use std::fs;
use std::net::SocketAddr;
use std::ops::Not;
use std::sync::LazyLock;
use time::Duration;
use tracing::{debug, error, info};
use uiua::*;
use uuid::Uuid;

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
pub async fn handle_main(jar: CookieJar) -> Result<impl IntoResponse, AppError> {
    let new_uuid = Uuid::new_v4();
    let cookie = Cookie::build(("SessionID", new_uuid.to_string()))
        .domain("portofolio.klownie.me")
        .max_age(Duration::days(100))
        .http_only(true)
        .secure(true)
        .build();

    let new_jar = match jar.get("SessionID").map(|cookie| cookie.value().to_owned()) {
        Some(uuid) => {
            if uiua!(
                "# Experimental!
            Path     ← \"sessions.data\"
            Database ← &frab Path
            °binaryDatabase
            has  □\"{uuid}\""
            )
            .pop_bool()
            .unwrap()
            {
                info!("Welcome back : {}", uuid);
                jar
            } else {
                info!("Invalid session : {}", uuid);
                create_session(&new_uuid);
                jar.add(cookie)
            }
        }
        None => {
            create_session(&new_uuid);
            jar.add(cookie)
        }
    };

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

    Ok((StatusCode::OK, new_jar, Html(reply)))
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

pub async fn render_project_template(
    project: &str,
    high_res: bool,
) -> Result<impl IntoResponse, AppError> {
    use Project::*;

    let template = match project {
        "VulturesBrigadeCaptain" => VulturesBrigadeCaptain {
            project: project.into(),
            high_res,
        },
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
            return Ok((StatusCode::NOT_FOUND, Html(Error404 {}.render()?)));
        }
    };

    debug!("Rendering project: {}", project);
    Ok((StatusCode::OK, Html(template.render()?)))
}
fn create_session(new_uuid: &Uuid) {
    uiua!(
        "# Experimental!
        Path     ← \"sessions.data\"
        Database ← &frab Path

        □\"{new_uuid}\"
        □NaN
        □°binaryDatabase
        ⊙⊙°□°⊟₃⇌⊟₃
        insert
        &s .
        binary
        &fwa Path"
    );
    info!("Created : {}", new_uuid);
}

pub async fn db_handle(Path(action): Path<String>) {
    todo!()
}

