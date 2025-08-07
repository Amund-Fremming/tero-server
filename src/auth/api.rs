use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use tracing::{debug, info};

use crate::{
    auth::{Auth0User, PutUserRequest, Subject, db},
    error::ServerError,
    state::AppState,
};

pub async fn get_user_from_subject(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
) -> Result<impl IntoResponse, ServerError> {
    let option = match subject {
        Subject::Guest(id) => db::get_user_by_guest_id(state.get_pool(), id).await?,
        Subject::Registered(id) | Subject::Admin(id) => {
            db::get_user_by_auth0_id(state.get_pool(), id).await?
        }
        Subject::Auth0 => {
            return Err(ServerError::Api(
                StatusCode::FORBIDDEN,
                "Not allowed".into(),
            ));
        }
    };

    let user = option.ok_or(ServerError::NotFound("User".into()))?;
    Ok((StatusCode::OK, Json(user)))
}

pub async fn create_guest_user(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ServerError> {
    let guest_id = db::create_guest_user(state.get_pool()).await?;
    Ok((StatusCode::CREATED, Json(guest_id)))
}

pub async fn put_user(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Json(put_request): Json<PutUserRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let auth0_id = match subject {
        Subject::Registered(id) | Subject::Admin(id) => id,
        _ => {
            return Err(ServerError::Permission(
                "Guest users cannot update personal information".into(),
            ));
        }
    };

    db::put_user_by_auth0_id(state.get_pool(), auth0_id, put_request).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
) -> Result<impl IntoResponse, ServerError> {
    let auth0_id = match subject {
        Subject::Registered(id) | Subject::Admin(id) => id,
        _ => {
            return Err(ServerError::Api(
                StatusCode::FORBIDDEN,
                "Not allowed".into(),
            ));
        }
    };

    db::delete_user_by_auth0_id(state.get_pool(), auth0_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn auth0_trigger_endpoint(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Json(auth0_user): Json<Auth0User>,
) -> Result<impl IntoResponse, ServerError> {
    debug!("HIT");
    match subject {
        Subject::Auth0 => {
            // Inject to db
            info!("Auth0 post registration trigger was triggered");
            // TODO - remove
            debug!("{}", serde_json::to_string_pretty(&auth0_user).unwrap());

            Ok(())
        }
        _ => {
            return Err(ServerError::Api(
                StatusCode::FORBIDDEN,
                "Not allowed".into(),
            ));
        }
    }
}
