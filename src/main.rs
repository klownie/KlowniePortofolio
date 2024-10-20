use axum::{
    routing::{get, post,},
    http::StatusCode,
    Json, Router,
    response::Html,
};
use std::net::SocketAddr;
use askama_axum::Template;
use log::{log, Level, debug, warn, info, error, trace};

const IP : &'static str = "0.0.0.0";
const PORT : &'static str = "3000";
#[derive(Template)]
#[template(path = "index.html")]
pub struct MyTemplate {}

async fn handler() -> Html<String> {
    Html(MyTemplate {}.render().unwrap())
}

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(handler));

    let listen_addr: SocketAddr = format!("{}:{}", IP, PORT)
        .parse()
        .unwrap();

    log!(Level::Info,"listening on : {}",listen_addr);

    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
