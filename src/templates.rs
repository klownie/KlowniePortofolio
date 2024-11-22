use askama_axum::Template;
use askama_enum::EnumTemplate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Template)]
#[template(path = "errors/error_404.html")]
pub struct Error404 {}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/index.html")]
pub struct Index {}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/topper.html")]
pub struct Topper {}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/interactive_name.html")]
pub struct InteractiveName {
    pub titles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/frame_toggle.html")]
pub struct ToggleFullscreen {
    pub fullscreen: bool,
    pub project: String,
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
}

#[derive(Debug, Serialize, Deserialize, EnumTemplate)]
pub enum Project {
    #[template(path = "projects/MissingProject.html")]
    MissingFile,

    #[template(path = "projects/SamuConceptCharacter.html")]
    SamuConceptCharacter {
        project: String,
        high_res: bool,
    },

    #[template(path = "projects/SamuConceptIllustration.html")]
    SamuConceptIllustration {
        project: String,
        high_res: bool,
    },

    #[template(path = "projects/SaintJohn.html")]
    SaintJohn {
        project: String,
        high_res: bool,
    },

    #[template(path = "projects/HomardRojas.html")]
    HomardRojas {
        project: String,
        high_res: bool,
    },

    #[template(path = "projects/CarbonixWorkerSuit.html")]
    CarbonixWorkerSuit {
        project: String,
        high_res: bool,
    },

    #[template(path = "projects/ClimbingExoSuit.html")]
    ClimbingExoSuit {
        project: String,
        high_res: bool,
    },

    #[template(path = "projects/Intru.html")]
    Intru {
        project: String,
        high_res: bool,
    },

    #[template(path = "projects/UmbrellaKnight.html")]
    UmbrellaKnight {
        project: String,
        high_res: bool,
    },

    #[template(path = "projects/ClimbingExoSuit3d.html")]
    ClimbingExoSuit3d {
        project: String,
        high_res: bool,
    },

    #[template(path = "projects/TeamBlue.html")]
    TeamBlue {
        project: String,
        high_res: bool,
    },

    #[template(path = "projects/TribalYellowDemon.html")]
    TribalYellowDemon {
        project: String,
        high_res: bool,
    },

    #[template(path = "projects/UrbanWhiteCrowMan.html")]
    UrbanWhiteCrowMan {
        project: String,
        high_res: bool,
    },
}

mod filters {
    use text_to_ascii_art::to_art;
    pub fn asciart<T: std::fmt::Display>(s: T) -> askama::Result<String> {
        let s: String = s.to_string();
        Ok(to_art(s, "", 0, 0, 0).unwrap())
    }

    pub fn asciart_return_to_line<T: std::fmt::Display>(s: T) -> askama::Result<String> {
        let s: String = s.to_string();

        let r = s.split_whitespace()
            .map(|part| to_art(part.into(), "", 0, 0, 0).unwrap())
            .collect::<Vec<_>>()
            .join("\n\n");

        Ok(r)
    }

    pub fn clean_name<T: std::fmt::Display>(s: T) -> askama::Result<String> {
        let mut result = String::new();

        for (prev, c) in std::iter::once(None)
            .chain(s.to_string().chars().map(Some))
            .zip(s.to_string().chars())
        {
            if c.is_uppercase() && prev.map_or(false, |p| p != '.' && p != '-') {
                result.push(' ');
            }
            result.push(c);
        }

        Ok(result)
    }
}
