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
    pub path: String,
}

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
    pub path: String,
}

mod filters {
    use text_to_ascii_art::to_art;
    pub fn asciart<T: std::fmt::Display>(s: T) -> askama::Result<String> {
        let s: String = s.to_string();
        Ok(to_art(s, "", 0, 0, 0).unwrap())
    }
}
