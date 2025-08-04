use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    auth::{Subject, User, db},
    error::ServerError,
    state::AppState,
};

pub async fn create_guest_user(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ServerError> {
    let user = User::new_guest_user();
    let guest_id = db::create_guest_user(state.get_pool(), &user).await?;

    Ok((StatusCode::CREATED, Json(guest_id)))
}

pub async fn put_user(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
) -> Result<impl IntoResponse, ServerError> {
    if let Subject::GuestUser(_) = subject {
        return Err(ServerError::Permission(
            "Guest users cannot update personal information".into(),
        ));
    }

    Ok(())
}

pub async fn patch_user(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
) -> Result<impl IntoResponse, ServerError> {
    if let Subject::GuestUser(_) = subject {
        return Err(ServerError::Permission(
            "Guest users cannot update personal information".into(),
        ));
    }

    Ok(())
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
) -> Result<impl IntoResponse, ServerError> {
    //

    Ok(())
}

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
) -> Result<impl IntoResponse, ServerError> {
    //
    Ok(())
}
