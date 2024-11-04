#[allow(unused)]
use askama_axum::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "index.html")]
pub struct Index {
    pub indexed: usize,
    pub name: String,
    pub fullscreen: bool,
    pub masonry: Vec<String>,
    pub project: String,
}

#[derive(Debug, Template)]
#[template(path = "error_404.html")]
pub struct Error404 {}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "interactive_name.html")]
pub struct InteractiveName {
    pub indexed: usize,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "frame_toggle.html")]
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

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/missing_project.html")]
pub struct MissingProject {}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/samu_concept_character.html")]
pub struct SamuConceptCharacter {
    pub project_name: String,
    pub high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/saint_john.html")]
pub struct SaintJohn {
    pub project_name: String,
    pub high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/homard_rojas.html")]
pub struct HomardRojas {
    pub project_name: String,
    pub high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/carbonix_worker_suit.html")]
pub struct CarbonixWorkerSuit {
    pub project_name: String,
    pub high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/climbing_exo_suit.html")]
pub struct ClimbingExoSuit {
    pub project_name: String,
    pub high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/intru.html")]
pub struct Intru {
    pub project_name: String,
    pub high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/climbing_exo_suit_3d.html")]
pub struct ClimbingExoSuit3d {
    pub project_name: String,
    pub high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/team_blue.html")]
pub struct TeamBlue {
    pub project_name: String,
    pub high_res: bool,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/tribal_yellow_demon.html")]
pub struct TribalYellowDemon {
    pub project_name: String,
    pub high_res: bool,
}
#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "projects/urban_white_crow.html")]
pub struct UrbanWhiteCrowMan {
    pub project_name: String,
    pub high_res: bool,
}
