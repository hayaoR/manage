use axum::{routing::{get, IntoMakeService}, Router, AddExtensionLayer};
use std::net::TcpListener;
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;
use crate::routes::health_check;

pub fn run(listener: TcpListener, db_pool: PgPool) -> hyper::Result<axum::Server<AddrIncoming, IntoMakeService<axum::Router>>> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/health_check", get(health_check))
        .layer(AddExtensionLayer::new(db_pool));

    let server = axum::Server::from_tcp(listener)?
        .serve(app.into_make_service());

    Ok(server)
}