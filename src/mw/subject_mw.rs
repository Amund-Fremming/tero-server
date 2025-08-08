use std::{collections::HashSet, sync::Arc};

use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation, decode, decode_header};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::{
    AUTH0_AUDIENCE, AUTH0_DOMAIN,
    auth::Subject,
    error::ServerError,
    state::{AppState, Jwks},
};

static AUTH0_WEBHOOK_KEY: &str = "Auth0-Webhook-Key";

pub async fn subject_mw(
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, ServerError> {
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
        let (subject, permissions) =
            get_subject_and_permissions(auth_header.unwrap(), state.get_jwks()).await?;

        debug!("Request by subject: {:?}", &subject);
        req.extensions_mut().insert(permissions);
        req.extensions_mut().insert(subject);
    }

    if auth0_webhook_key.is_some() {
        let subject = Subject::Auth0;
        info!("Request by subject: {:?}", subject);
        req.extensions_mut().insert(subject);
    }

    Ok(next.run(req).await)
}

// MOVE
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: Vec<String>,
    azp: String,
    exp: i32,
    iat: i32,
    iss: String,
    pub scope: String,
    pub sub: String,
    pub permissions: HashSet<Permission>,
}

// Move
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Permissions {
    permissions: HashSet<Permission>,
}

// Move
impl Permissions {
    pub fn none() -> Self {
        Self {
            permissions: HashSet::new(),
        }
    }

    pub fn new(permissions: HashSet<Permission>) -> Self {
        Self { permissions }
    }

    pub fn has(&self, required_perm: Permission) -> Option<Permission> {
        if !self.permissions.contains(&required_perm) {
            return Some(required_perm);
        }
        None
    }
}

// MOVE
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub enum Permission {
    #[serde(rename(deserialize = "read:admin"))]
    ReadAdmin,
    #[serde(rename(deserialize = "write:admin"))]
    WriteAdmin,
    #[serde(rename(deserialize = "save:games"))]
    SaveGames,
}

async fn get_subject_and_permissions(
    header_value: String,
    jwks: &Jwks,
) -> Result<(Subject, Permissions), ServerError> {
    if let Some(token) = header_value.strip_prefix("Bearer ") {
        let token_data = verify_jwt(token, jwks).await?;
        let claims: Claims = serde_json::from_value(token_data.claims)
            .map_err(|e| ServerError::Json(format!("Deserialization error: {:?}", e)))?;

        let subject = Subject::Registered(claims.sub);
        let permissions = Permissions::new(claims.permissions);

        return Ok((subject, permissions));
    }

    if let Some(value) = header_value.strip_prefix("Guest ") {
        let id: Uuid = value.parse().map_err(|_| {
            ServerError::Api(StatusCode::BAD_REQUEST, "Failed to parse header".into())
        })?;
        return Ok((Subject::Guest(id), Permissions::none()));
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

// Warning: 65% AI generated code
async fn verify_jwt(token: &str, jwks: &Jwks) -> Result<TokenData<serde_json::Value>, ServerError> {
    let header = decode_header(token)
        .map_err(|e| ServerError::JwtVerification(format!("Failed to decode header: {}", e)))?;

    let kid = header
        .kid
        .ok_or_else(|| ServerError::JwtVerification("Missing JWT kid".into()))?;

    let jwk = jwks
        .keys
        .iter()
        .find(|jwk| jwk.kid == kid)
        .ok_or_else(|| ServerError::JwtVerification("JWK is not well known".into()))?;

    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
        .map_err(|e| ServerError::JwtVerification(format!("Failed to get decoding key: {}", e)))?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[AUTH0_AUDIENCE.to_string()]);
    validation.set_issuer(&[AUTH0_DOMAIN.to_string()]);

    decode::<serde_json::Value>(token, &decoding_key, &validation)
        .map_err(|e| ServerError::JwtVerification(format!("Failed to validate token: {}", e)))
}
