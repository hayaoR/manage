use axum::{routing::{get, IntoMakeService}, Router};
use std::net::TcpListener;
use hyper::server::conn::AddrIncoming;


async fn health_check() {}

pub fn run(listener: TcpListener) -> hyper::Result<axum::Server<AddrIncoming, IntoMakeService<axum::Router>>> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/health_check", get(health_check));

    let server = axum::Server::from_tcp(listener)?
        .serve(app.into_make_service());

    Ok(server)
}