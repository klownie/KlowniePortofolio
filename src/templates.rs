#[allow(unused)]
use askama_axum::Template;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::iter::Cycle;
use std::sync::Mutex;
use text_to_ascii_art::to_art;

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "index.html")]
pub struct Index {
    pub indexed: usize,
    pub name: String,
    pub fullscreen: bool,
    pub masonry: Vec<String>,
    pub path :String
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
    pub path :String
}

mod filters {
    use text_to_ascii_art::to_art;

    // This filter does not have extra arguments
    pub fn trimclean<T: std::fmt::Display>(s: T) -> askama::Result<String> {
        let s = s.to_string();
        let trimed = s
            .find(':')
            .map_or(None, |pos| Some(s[(pos + 2)..].to_string()));
        Ok(trimed.expect("didnt find"))
    }

    pub fn asciart<T: std::fmt::Display>(s: T) -> askama::Result<String> {
        let s: String = s.to_string();
        Ok(to_art(s, "", 0, 0, 0).unwrap())
    }

    pub fn toggle<T: std::fmt::Display>(s: T) -> askama::Result<String> {
        let s: String = s.to_string();
        let pos = s.find(':').expect("didnt find the ':'");
        let front: String = s[..pos.clone()].into();
        let back: String = s[(pos + 2).clone()..].into();
        Ok(format!("{}::{}", back, front))
    }
}