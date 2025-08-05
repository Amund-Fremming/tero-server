use std::env;

use axum::{Router, middleware::from_fn};
use dotenv::dotenv;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::FmtSubscriber;

use crate::{
    auth::{protected_auth_routes, public_auth_routes},
    health::health_routes,
    mw::{request_mw, subject_mw},
    quiz::quiz_routes,
    state::AppState,
};

mod auth;
mod common;
mod error;
mod health;
mod mw;
mod quiz;
mod spinner;
mod state;
mod ws;

#[tokio::main]
async fn main() {
    // Initialize .env
    dotenv().ok();

    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global tracing");

    // Initialize state
    let connection_string =
        env::var("DATABASE_URL").expect("DATABASE_URL is missing as env variable");
    let state = AppState::from_connection_string(&connection_string)
        .await
        .unwrap_or_else(|e| panic!("{}", e));

    // Initialize routes
    let public_routes = Router::new()
        .nest("/health", health_routes())
        .nest("/guest-user", public_auth_routes(state.clone()));

    let protected_routes = Router::new()
        .nest("/user", protected_auth_routes(state.clone()))
        .layer(from_fn(subject_mw));

    let app = Router::new()
        .merge(protected_routes)
        .merge(public_routes)
        .layer(from_fn(request_mw));

    // Initialize webserver
    let port = env::var("PORT").expect("PORT is missing as env variable");
    let host = env::var("HOST").expect("HOST is missing as env variable");

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    info!("Server listening on address: {}:{}", host, port);
    axum::serve(listener, app).await.unwrap();
}
