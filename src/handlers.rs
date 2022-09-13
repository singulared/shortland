use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    Json,
};

use crate::{backend::Backend, errors::ServiceError, service, shortener::Shortner};

pub async fn create_shorten<S: Shortner, B: Backend>(
    State(state): State<Arc<service::State<S, B>>>,
    Json(url): Json<String>,
) -> std::result::Result<(StatusCode, String), ServiceError> {
    let id = state.backend.store(&url).await?;
    Ok((StatusCode::CREATED, state.shortner.encode(id).await?))
}

pub async fn expand_shorten<S: Shortner, B: Backend>(
    State(state): State<Arc<service::State<S, B>>>,
    Path(shorten): Path<String>,
) -> Result<Redirect, ServiceError> {
    let id = state.shortner.decode(&shorten).await?;
    let url = state.backend.retrive(id).await?;
    Ok(Redirect::temporary(url.as_str()))
}

pub async fn get_stat_by_shorten<S: Shortner, B: Backend>(
    State(state): State<Arc<service::State<S, B>>>,
    Path(shorten): Path<String>,
) -> Result<String, ServiceError> {
    let id = state.shortner.decode(&shorten).await?;
    let stat = state.backend.stat(id, None).await?;
    Ok(stat.to_string())
}

pub async fn update_shorten<S: Shortner, B: Backend>(
    State(state): State<Arc<service::State<S, B>>>,
    Path(shorten): Path<String>,
    Json(url): Json<String>,
) -> Result<StatusCode, ServiceError> {
    let id = state.shortner.decode(&shorten).await?;
    state.backend.update(id, &url).await?;
    Ok(StatusCode::CREATED)
}

pub async fn delete_shorten<S: Shortner, B: Backend>(
    State(state): State<Arc<service::State<S, B>>>,
    Path(shorten): Path<String>,
) -> Result<StatusCode, ServiceError> {
    let id = state.shortner.decode(&shorten).await?;
    state.backend.delete(id).await?;
    Ok(StatusCode::GONE)
}
