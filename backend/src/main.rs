use axum::{Router, http::{HeaderValue, Method, header::{ACCEPT, AUTHORIZATION}}};
use tower_http::cors::CorsLayer;
use crate::{config::Config};
use crate::routes::create_routes;

mod config;
mod database;
mod models;
mod error;
mod http;
mod routes;
mod services;
mod repositories;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>()?)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT])
        .allow_credentials(true);

    let app = Router::new()
        .merge(create_routes())
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))
        .await?;

    axum::serve(listener, app).await?;
    Ok(())
}
