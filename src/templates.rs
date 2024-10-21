use askama_axum::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub count: u64,
}

#[derive(Template)]
#[template(path = "testcounter.html")]
pub struct TestCounter {
    pub count: u64,
}