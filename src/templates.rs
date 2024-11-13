use askama_axum::Template;
use serde::{Deserialize, Serialize};
use askama_enum::EnumTemplate;


#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/index.html")]
pub struct Index {
    pub titles: Vec<String>,
    pub fullscreen: bool,
    pub masonry: Vec<String>,
    pub project: String,
}

#[derive(Debug, Template)]
#[template(path = "errors/error_404.html")]
pub struct Error404 {}

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

mod filters {
    use text_to_ascii_art::to_art;
    pub fn asciart<T: std::fmt::Display>(s: T) -> askama::Result<String> {
        let s: String = s.to_string();
        Ok(to_art(s, "", 0, 0, 0).unwrap())
    }
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