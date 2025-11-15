use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Template)]
#[template(path = "errors/error_404.html")]
pub struct Error404 {}

#[derive(Debug, Template)]
#[template(path = "errors/error_500.html")]
pub struct Error500 {
    pub err: String,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/index.html")]
pub struct Index {}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/topper.html")]
pub struct Topper {
    pub themes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/interactive_name.html")]
pub struct InteractiveName {
    pub titles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/socials.html")]
pub struct Socials {}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/bio.html")]
pub struct Bio {}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/masonry.html")]
pub struct Masonry {
    pub projects: Vec<String>,
    pub loaded: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/BatBossIllustration.html")]
pub struct BatBossIllustration {
    project: String,
    high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/VulturesBrigadeCaptain.html")]
pub struct VulturesBrigadeCaptain {
    project: String,
    high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/SamuConceptCharacter.html")]
pub struct SamuConceptCharacter {
    project: String,
    high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/SamuConceptIllustration.html")]
pub struct SamuConceptIllustration {
    project: String,
    high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/SaintJohn.html")]
pub struct SaintJohn {
    project: String,
    high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/HomardRojas.html")]
pub struct HomardRojas {
    project: String,
    high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/CarbonixWorkerSuit.html")]
pub struct CarbonixWorkerSuit {
    project: String,
    high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/ClimbingExoSuit.html")]
pub struct ClimbingExoSuit {
    project: String,
    high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/Intru.html")]
pub struct Intru {
    project: String,
    high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/UmbrellaKnight.html")]
pub struct UmbrellaKnight {
    project: String,
    high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/ClimbingExoSuit3d.html")]
pub struct ClimbingExoSuit3d {
    project: String,
    high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/TeamBlue.html")]
pub struct TeamBlue {
    project: String,
    high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/TribalYellowDemon.html")]
pub struct TribalYellowDemon {
    project: String,
    high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/UrbanWhiteCrowMan.html")]
pub struct UrbanWhiteCrowMan {
    project: String,
    high_res: bool,
}

pub async fn render_project_template(
    project: &str,
    high_res: bool,
) -> Result<impl IntoResponse, AppError> {
    match project {
        "BatBossIllustration" => {
            debug!("Rendering project: {}", project);
            return Ok((
                StatusCode::OK,
                Html(
                    BatBossIllustration {
                        project: project.into(),
                        high_res,
                    }
                    .render()?,
                ),
            ));
        }

        "VulturesBrigadeCaptain" => {
            debug!("Rendering project: {}", project);
            return Ok((
                StatusCode::OK,
                Html(
                    VulturesBrigadeCaptain {
                        project: project.into(),
                        high_res,
                    }
                    .render()?,
                ),
            ));
        }

        "SamuConceptIllustration" => {
            debug!("Rendering project: {}", project);
            return Ok((
                StatusCode::OK,
                Html(
                    SamuConceptIllustration {
                        project: project.into(),
                        high_res,
                    }
                    .render()?,
                ),
            ));
        }

        "SamuConceptCharacter" => {
            debug!("Rendering project: {}", project);
            return Ok((
                StatusCode::OK,
                Html(
                    SamuConceptCharacter {
                        project: project.into(),
                        high_res,
                    }
                    .render()?,
                ),
            ));
        }

        "SaintJohn" => {
            debug!("Rendering project: {}", project);
            return Ok((
                StatusCode::OK,
                Html(
                    SaintJohn {
                        project: project.into(),
                        high_res,
                    }
                    .render()?,
                ),
            ));
        }

        "HomardRojas" => {
            debug!("Rendering project: {}", project);
            return Ok((
                StatusCode::OK,
                Html(
                    HomardRojas {
                        project: project.into(),
                        high_res,
                    }
                    .render()?,
                ),
            ));
        }

        "CarbonixWorkerSuit" => {
            debug!("Rendering project: {}", project);
            return Ok((
                StatusCode::OK,
                Html(
                    CarbonixWorkerSuit {
                        project: project.into(),
                        high_res,
                    }
                    .render()?,
                ),
            ));
        }

        "ClimbingExoSuit" => {
            debug!("Rendering project: {}", project);
            return Ok((
                StatusCode::OK,
                Html(
                    ClimbingExoSuit {
                        project: project.into(),
                        high_res,
                    }
                    .render()?,
                ),
            ));
        }

        "ClimbingExoSuit3d" => {
            debug!("Rendering project: {}", project);
            return Ok((
                StatusCode::OK,
                Html(
                    ClimbingExoSuit3d {
                        project: project.into(),
                        high_res,
                    }
                    .render()?,
                ),
            ));
        }

        "Intru" => {
            debug!("Rendering project: {}", project);
            return Ok((
                StatusCode::OK,
                Html(
                    Intru {
                        project: project.into(),
                        high_res,
                    }
                    .render()?,
                ),
            ));
        }

        "UmbrellaKnight" => {
            debug!("Rendering project: {}", project);
            return Ok((
                StatusCode::OK,
                Html(
                    UmbrellaKnight {
                        project: project.into(),
                        high_res,
                    }
                    .render()?,
                ),
            ));
        }

        "TeamBlue" => {
            debug!("Rendering project: {}", project);
            return Ok((
                StatusCode::OK,
                Html(
                    TeamBlue {
                        project: project.into(),
                        high_res,
                    }
                    .render()?,
                ),
            ));
        }

        "TribalYellowDemon" => {
            debug!("Rendering project: {}", project);
            return Ok((
                StatusCode::OK,
                Html(
                    TribalYellowDemon {
                        project: project.into(),
                        high_res,
                    }
                    .render()?,
                ),
            ));
        }

        "UrbanWhiteCrowMan" => {
            debug!("Rendering project: {}", project);
            return Ok((
                StatusCode::OK,
                Html(
                    UrbanWhiteCrowMan {
                        project: project.into(),
                        high_res,
                    }
                    .render()?,
                ),
            ));
        }

        _ => {
            return Ok((StatusCode::NOT_FOUND, Html(Error404 {}.render()?)));
        }
    }
}

mod filters {
    use text_to_ascii_art::to_art;
    pub fn asciart<T: std::fmt::Display>(s: T, _: &dyn askama::Values) -> askama::Result<String> {
        let s: String = s.to_string();
        Ok(to_art(s, "", 0, 0, 0).unwrap())
    }

    pub fn asciart_return_to_line<T: std::fmt::Display>(
        s: T,
        _: &dyn askama::Values,
    ) -> askama::Result<String> {
        let s: String = s.to_string();

        let r = s
            .split_whitespace()
            .map(|part| to_art(part.into(), "", 0, 0, 0).unwrap())
            .collect::<Vec<_>>()
            .join("\n\n");

        Ok(r)
    }

    pub fn clean_name<T: std::fmt::Display>(
        s: T,
        _: &dyn askama::Values,
    ) -> askama::Result<String> {
        let mut result = String::new();

        for (prev, c) in std::iter::once(None)
            .chain(s.to_string().chars().map(Some))
            .zip(s.to_string().chars())
        {
            if c.is_uppercase() && prev.is_some_and(|p| p != '.' && p != '-') {
                result.push(' ');
            }
            result.push(c);
        }

        Ok(result)
    }

    pub fn to_javascript<T: std::fmt::Display>(
        s: &[T],
        _: &dyn askama::Values,
    ) -> askama::Result<String> {
        let mut result = String::from("[");

        for (i, item) in s.iter().enumerate() {
            result.push('\''); // Use single quote
            result.push_str(&item.to_string());
            result.push('\''); // Use single quote
            if i < s.len() - 1 {
                result.push(',');
            }
        }

        result.push(']');
        Ok(result)
    }
}
