use axum::{
    body::Body,
    extract::Request,
    http::{StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};

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
        req.extensions_mut().insert(Subject::PersistentUser(1));
    }

    if guest_header.is_some() {
        let id: i32 = guest_header.unwrap().parse().map_err(|_| {
            ServerError::Api(StatusCode::BAD_REQUEST, "Failed to parse header".into())
        })?;

        req.extensions_mut().insert(Subject::GuestUser(id));
    }

    Ok(next.run(req).await)
}
