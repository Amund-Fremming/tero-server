use axum::{
    body::Body,
    extract::Request,
    http::{StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};
use tracing::info;
use uuid::Uuid;

use crate::{auth::Subject, error::ServerError};

static GUEST_AUTH: &str = "GUEST_AUTH";

pub async fn subject_mw(mut req: Request<Body>, next: Next) -> Result<Response, ServerError> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .map(|s| s.to_owned());

    let guest_header = req
        .headers()
        .get(GUEST_AUTH)
        .and_then(|header| header.to_str().ok())
        .map(|s| s.to_owned());

    if auth_header.is_none() && guest_header.is_none() {
        return Err(ServerError::Api(
            StatusCode::UNAUTHORIZED,
            "Missing authorization header".into(),
        ));
    }

    if auth_header.is_some() && auth_header.unwrap().starts_with("Bearer ") {
        // validate token
        // Get user id
        let id = "some-auth0-id".to_string();
        let subject = Subject::Registered(id);
        info!("Request by subject: {:?}", &subject);
        req.extensions_mut().insert(subject);
    }

    if guest_header.is_some() {
        let id: Uuid = guest_header.unwrap().parse().map_err(|_| {
            ServerError::Api(StatusCode::BAD_REQUEST, "Failed to parse header".into())
        })?;

        let subject = Subject::Guest(id);
        info!("Request by subject: {:?}", &subject);
        req.extensions_mut().insert(subject);
    }

    Ok(next.run(req).await)
}
