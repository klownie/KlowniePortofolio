use askama_axum::Template;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub count: u64,
    pub name:ToggleName,
}

#[derive(Template)]
#[template(path = "testcounter.html")]
pub struct TestCounter {
    pub count: u64,
}

#[derive(Template)]
#[template(path = "interactive_name.html", print = "ast")]
pub struct InteractiveName {
    pub name: ToggleName,
}



#[derive(Serialize, Deserialize, Clone)]
pub enum ToggleName {
    Normal(String),
    AsciArt(String),
}

impl ToggleName {
    #[inline]
    pub fn toggle(&self) -> Self {
        match self {
            ToggleName::Normal(_) => ToggleName::AsciArt(String::from("WHAT")),
            ToggleName::AsciArt(_) => ToggleName::Normal(String::from("Audrick Yeu")),
        }
    }
}

impl fmt::Display for ToggleName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ToggleName::Normal(text) => write!(f, "{}", text),
            ToggleName::AsciArt(text) => write!(f, "{}", text),
        }
    }
}