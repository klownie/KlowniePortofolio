use askama_axum::Template;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub count: u64,
    pub name: ToggleName,
}

#[derive(Template)]
#[template(path = "testcounter.html")]
pub struct TestCounter {
    pub count: u64,
}

#[derive(Template)]
#[template(path = "interactive_name.html")]
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
            ToggleName::Normal(_) => ToggleName::AsciArt(String::from("WHUTT")),
            ToggleName::AsciArt(_) => ToggleName::Normal(String::from("Audrick Yeu")),
        }
    }
}

impl fmt::Display for ToggleName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ToggleName::Normal(text) => write!(f, "Normal::{}", text),
            ToggleName::AsciArt(text) => write!(f, "AsciArt::{}", text),
        }
    }
}

impl FromStr for ToggleName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("AsciArt::") {
            Ok(ToggleName::AsciArt(s[9..].to_string()))
        } else {
            Ok(ToggleName::Normal(s[8..].to_string()))
        }
    }
}

mod filters {
    // This filter does not have extra arguments
    pub fn trimclean<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let s = s.to_string();
        let trimed = s
            .rfind(':')
            .map_or(None, |pos| Some(s[(pos + 1)..].to_string()));
        Ok(trimed.expect("didnt find"))
    }
}
