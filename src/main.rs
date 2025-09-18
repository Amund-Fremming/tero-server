use std::env;

use axum::{
    Router,
    middleware::{from_fn, from_fn_with_state},
};
use dotenv::dotenv;
use once_cell::sync::Lazy;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::FmtSubscriber;

use crate::{
    auth::handlers::{protected_auth_routes, public_auth_routes},
    common::{app_state::AppState, handlers::common_routes},
    health::health_routes,
    mw::{auth_mw::auth_mw, request_mw::request_mw},
};

mod auth;
mod common;
mod health;
mod mw;
mod quiz;
mod spin;

static AUTH0_WEBHOOK_KEY: Lazy<String> = Lazy::new(|| {
    env::var("AUTH0_WEBHOOK_KEY").expect("AUTH0_WEBHOOK_KEY is missing as env variable")
});
static AUTH0_DOMAIN: Lazy<String> =
    Lazy::new(|| env::var("AUTH0_DOMAIN").expect("AUTH0_DOMAIN is missing as env variable"));
static AUTH0_AUDIENCE: Lazy<String> =
    Lazy::new(|| env::var("AUTH0_AUDIENCE").expect("AUTH0_AUDIENCE is missing as env variable"));

#[tokio::main]
async fn main() {
    // Initialize .env
    dotenv().ok();

    // Validate that env variables exists
    Lazy::force(&AUTH0_WEBHOOK_KEY);
    Lazy::force(&AUTH0_DOMAIN);

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
        .nest("/guest-user", public_auth_routes(state.clone()))
        .nest("/games", common_routes(state.clone()));

    let protected_routes = Router::new()
        .nest("/user", protected_auth_routes(state.clone()))
        .layer(from_fn_with_state(state.clone(), auth_mw));

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
