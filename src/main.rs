mod errors;
mod handlers;
mod macros;
mod routes;
mod templates;

use crate::handlers::Ports;
use crate::handlers::CONFIG;
use axum::{
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    response::Redirect,
    BoxError,
};
use axum_extra::extract::Host;
use routes::build_routes;
use std::net::SocketAddr;

turf::style_sheet!("scss/index.scss");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let app = build_routes();

    #[cfg(feature = "tls")]
    {
        use axum_server::tls_rustls::RustlsConfig;
        use std::path::PathBuf;

        tokio::spawn(redirect_http_to_https(CONFIG.ports));

        let config = RustlsConfig::from_pem_file(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("private_certs")
                .join("cert.pem"),
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("private_certs")
                .join("key.pem"),
        )
        .await
        .expect("failed to load TLS certs");

        let addr = SocketAddr::from(([127, 0, 0, 1], CONFIG.ports.https));
        tracing::info!("listening on https://{}", addr);

        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    }

    #[cfg(not(feature = "tls"))]
    {
        let addr = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", CONFIG.ports.http))
            .await
            .unwrap();
        tracing::info!("listening on http://{}", addr.local_addr().unwrap());

        axum::serve(addr, app).await.unwrap();
    }
}

#[allow(dead_code)]
async fn redirect_http_to_https(ports: Ports) {
    fn make_https(host: String, uri: Uri, ports: Ports) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&ports.http.to_string(), &ports.https.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, ports) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", CONFIG.ports.http))
        .await
        .unwrap();
    tracing::info!("listening on http://{}", addr.local_addr().unwrap());
    axum::serve(
        addr,
        redirect.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
