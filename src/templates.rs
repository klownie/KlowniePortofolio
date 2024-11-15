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
    pub fullscreen: bool,
    pub masonry: Vec<String>,
    pub project: String,
}

#[derive(Debug, Serialize, Deserialize, EnumTemplate)]
pub enum Project {
    #[template(path = "projects/MissingProject.html")]
    MissingProject,

    #[template(path = "projects/SamuConceptCharacter.html")]
    SamuConceptCharacter {
        project_name: String,
        high_res: bool,
    },

    #[template(path = "projects/SamuConceptIllustration.html")]
    SamuConceptIllustration {
        project_name: String,
        high_res: bool,
    },

    #[template(path = "projects/SaintJohn.html")]
    SaintJohn {
        project_name: String,
        high_res: bool,
    },

    #[template(path = "projects/HomardRojas.html")]
    HomardRojas {
        project_name: String,
        high_res: bool,
    },

    #[template(path = "projects/CarbonixWorkerSuit.html")]
    CarbonixWorkerSuit {
        project_name: String,
        high_res: bool,
    },

    #[template(path = "projects/ClimbingExoSuit.html")]
    ClimbingExoSuit {
        project_name: String,
        high_res: bool,
    },

    #[template(path = "projects/Intru.html")]
    Intru {
        project_name: String,
        high_res: bool,
    },

    #[template(path = "projects/UmbrellaKnight.html")]
    UmbrellaKnight {
        project_name: String,
        high_res: bool,
    },

    #[template(path = "projects/ClimbingExoSuit3d.html")]
    ClimbingExoSuit3d {
        project_name: String,
        high_res: bool,
    },

    #[template(path = "projects/TeamBlue.html")]
    TeamBlue {
        project_name: String,
        high_res: bool,
    },

    #[template(path = "projects/TribalYellowDemon.html")]
    TribalYellowDemon {
        project_name: String,
        high_res: bool,
    },

    #[template(path = "projects/UrbanWhiteCrowMan.html")]
    UrbanWhiteCrowMan {
        project_name: String,
        high_res: bool,
    },
}

mod filters {
    use text_to_ascii_art::to_art;
    pub fn asciart<T: std::fmt::Display>(s: T) -> askama::Result<String> {
        let s: String = s.to_string();
        Ok(to_art(s, "", 0, 0, 0).unwrap())
    }

    pub fn clean_name<T: std::fmt::Display>(s: T) -> askama::Result<String> {
        let s = s.to_string();
        let mut result = String::new();
        let mut chars = s.chars().peekable();
        let mut last_char: Option<char> = None;

        while let Some(c) = chars.next() {
            if c.is_uppercase() {
                if let Some(prev_char) = last_char {
                    if prev_char != '.' && prev_char != '-' {
                        result.push(' ');
                    }
                }
            }
            result.push(c);
            last_char = Some(c);
        }

        Ok(result)
    }
}
