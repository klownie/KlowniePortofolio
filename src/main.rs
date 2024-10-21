mod templates;

use templates::*;

use axum::{
    routing::{get, post,},
    http::StatusCode,
    Json, Router,
    response::Html,
};
use std::net::SocketAddr;
use axum::extract::Path;
use log::{log, Level, debug, warn, info, error, trace};

use tower_http::services::{ServeDir,ServeFile};


const IP : &'static str = "0.0.0.0";
const PORT : &'static str = "3000";

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    let assets_service = ServeDir::new("assets");
    let favicon = ServeFile::new("favicon.ico");

    let app = Router::new()
        .route("/", get(|| async { Index {count:0} }))
        .route(
            "/counter/:count",
            get(|Path(count): Path<u64>| async move { TestCounter { count } }),
        )
        .nest_service("/assets",assets_service);

    let listen_addr: SocketAddr = format!("{}:{}", IP, PORT)
        .parse()
        .unwrap();

    log!(Level::Info,"listening on : {}",listen_addr);

    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
