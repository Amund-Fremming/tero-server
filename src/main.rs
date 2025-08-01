use std::env;

use axum::Router;
use dotenv::dotenv;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::FmtSubscriber;

use crate::{
    auth::auth_routes, health::health_routes, quiz::quiz_routes, spinner::spinner_routes,
    state::AppState,
};

mod auth;
mod error;
mod health;
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
    let app = Router::new()
        .nest("/health", health_routes())
        .nest("/auth", auth_routes(state.clone()))
        .nest("/quiz", quiz_routes(state.clone()))
        .nest("/spinner", spinner_routes(state.clone()));

    // Initialize webserver
    let port = env::var("PORT").expect("PORT is missing as env variable");
    let host = env::var("HOST").expect("HOST is missing as env variable");

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    info!("Server listening on address: {}:{}", host, port);
    axum::serve(listener, app).await.unwrap();
}
