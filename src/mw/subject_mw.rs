use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::{auth::Subject, error::ServerError};

static AUTH0_WEBHOOK_KEY: &str = "Auth0-Webhook-Key";

pub async fn subject_mw(mut req: Request<Body>, next: Next) -> Result<Response, ServerError> {
    let auth_header = extract_header(AUTHORIZATION.as_str(), req.headers());
    let auth0_webhook_key = extract_header(&AUTH0_WEBHOOK_KEY, req.headers());

    if auth_header.is_none() && auth0_webhook_key.is_none() {
        error!("Missing authentication headers");
        return Err(ServerError::Api(
            StatusCode::UNAUTHORIZED,
            "Missing authorization header".into(),
        ));
    }

    if auth_header.is_some() {
        let subject = get_subject(auth_header.unwrap())?;
        info!("Request by subject: {:?}", &subject);
        req.extensions_mut().insert(subject);
    }

    if auth0_webhook_key.is_some() {
        let subject = Subject::Auth0;
        info!("Request by subject: {:?}", subject);
        req.extensions_mut().insert(subject);
    }

    Ok(next.run(req).await)
}

fn get_subject(header_value: String) -> Result<Subject, ServerError> {
    if let Some(token) = header_value.strip_prefix("Bearer ") {
        // validate token
        // Get user id
        debug!("Recieved token: {}", token);
        let id = "some-auth0-id".to_string();
        return Ok(Subject::Registered(id));
    }

    if let Some(value) = header_value.strip_prefix("Guest ") {
        let id: Uuid = value.parse().map_err(|_| {
            ServerError::Api(StatusCode::BAD_REQUEST, "Failed to parse header".into())
        })?;
        return Ok(Subject::Guest(id));
    }

    Err(ServerError::Api(
        StatusCode::UNAUTHORIZED,
        "Not allowed".into(),
    ))
}

fn extract_header(key: &str, header_map: &HeaderMap) -> Option<String> {
    header_map
        .get(key)
        .and_then(|header| header.to_str().ok())
        .map(|s| s.to_owned())
}
