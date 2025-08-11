use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::{Pool, Postgres};
use tracing::info;

use crate::{
    auth::{Auth0User, Permission, PermissionCtx, PutUserRequest, Subject, db},
    error::ServerError,
    state::AppState,
};

pub async fn get_user_from_subject(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Extension(_permissions): Extension<PermissionCtx>,
) -> Result<impl IntoResponse, ServerError> {
    let option = match subject {
        Subject::Guest(id) => db::get_user_by_guest_id(state.get_pool(), id).await?,
        Subject::Registered(id) | Subject::Admin(id) => {
            db::get_user_by_auth0_id(state.get_pool(), id).await?
        }
        Subject::Auth0 => {
            return Err(ServerError::AccessDenied);
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

pub async fn patch_user(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Extension(permission_ctx): Extension<PermissionCtx>,
    Path(user_id): Path<i32>,
    Json(put_request): Json<PutUserRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let Subject::Registered(auth0_id) = subject else {
        return Err(ServerError::AccessDenied);
    };

    if permission_ctx.has(Permission::WriteAdmin) {
        db::patch_user_by_id(state.get_pool(), user_id, put_request).await?;
        return Ok(StatusCode::NO_CONTENT);
    }

    ensure_user_owns_data(state.get_pool(), user_id, auth0_id).await?;
    db::patch_user_by_id(state.get_pool(), user_id, put_request).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Extension(permission_ctx): Extension<PermissionCtx>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, ServerError> {
    let Subject::Registered(auth0_id) = subject else {
        return Err(ServerError::AccessDenied);
    };

    if permission_ctx.has(Permission::WriteAdmin) {
        db::delete_user_by_id(state.get_pool(), user_id).await?;
    }

    ensure_user_owns_data(state.get_pool(), user_id, auth0_id).await?;
    db::delete_user_by_id(state.get_pool(), user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn patch_user_activity(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Extension(_permission_ctx): Extension<PermissionCtx>,
    Path(user_id): Path<i32>,
) -> Result<(), ServerError> {
    if let Subject::Auth0 = subject {
        return Err(ServerError::AccessDenied);
    };

    db::update_user_activity(state.get_pool(), user_id).await?;
    Ok(())
}

pub async fn auth0_trigger_endpoint(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Json(auth0_user): Json<Auth0User>,
) -> Result<impl IntoResponse, ServerError> {
    let Subject::Auth0 = subject else {
        return Err(ServerError::AccessDenied);
    };

    info!("Auth0 post registration trigger was triggered");
    db::create_registered_user(state.get_pool(), &auth0_user).await?;

    Ok(())
}

pub async fn list_all_users(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Extension(permission_ctx): Extension<PermissionCtx>,
) -> Result<impl IntoResponse, ServerError> {
    let Subject::Registered(_) = subject else {
        return Err(ServerError::Api(
            StatusCode::FORBIDDEN,
            "Not allowed".into(),
        ));
    };

    if permission_ctx.has(Permission::ReadAdmin) {
        return Err(ServerError::Permission(Permission::ReadAdmin));
    }

    let users = db::list_all_users(state.get_pool()).await?;
    Ok((StatusCode::OK, Json(users)))
}

// Helper function
async fn ensure_user_owns_data(
    pool: &Pool<Postgres>,
    user_id: i32,
    auth0_id: String,
) -> Result<(), ServerError> {
    let target_user = db::get_user_by_id(pool, user_id)
        .await?
        .ok_or_else(|| ServerError::AccessDenied)?;

    if target_user.auth0_id != Some(auth0_id) {
        return Err(ServerError::AccessDenied);
    }

    Ok(())
}
