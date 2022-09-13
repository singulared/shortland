use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{StatusCode, Uri},
    response::Redirect,
};

use crate::{errors::ServiceError, service, shortener::Shortner};

pub async fn create_shorten<S: Shortner>(
    State(state): State<Arc<service::State<S>>>,
    uri: String,
) -> std::result::Result<(StatusCode, String), ServiceError> {
    let validated_uri = uri.trim().parse::<Uri>()?;
    let id = state
        .backend
        .store(validated_uri.to_string().as_str())
        .await?;
    Ok((StatusCode::CREATED, state.shortner.encode(id).await?))
}

pub async fn expand_shorten<S: Shortner>(
    State(state): State<Arc<service::State<S>>>,
    Path(shorten): Path<String>,
) -> Result<Redirect, ServiceError> {
    let id = state.shortner.decode(&shorten).await?;
    let uri = state.backend.retrive(id).await?;
    let validated_uri = uri.trim().parse::<Uri>()?;
    Ok(Redirect::temporary(validated_uri.to_string().as_str()))
}

pub async fn get_stat_by_shorten<S: Shortner>(
    State(state): State<Arc<service::State<S>>>,
    Path(shorten): Path<String>,
) -> Result<String, ServiceError> {
    let id = state.shortner.decode(&shorten).await?;
    let stat = state.backend.stat(id, None).await?;
    Ok(stat.to_string())
}

pub async fn update_shorten<S: Shortner>(
    State(state): State<Arc<service::State<S>>>,
    Path(shorten): Path<String>,
    uri: String,
) -> Result<StatusCode, ServiceError> {
    let validated_uri = uri.trim().parse::<Uri>()?;
    let id = state.shortner.decode(&shorten).await?;
    state
        .backend
        .update(id, validated_uri.to_string().as_str())
        .await?;
    Ok(StatusCode::CREATED)
}

pub async fn delete_shorten<S: Shortner>(
    State(state): State<Arc<service::State<S>>>,
    Path(shorten): Path<String>,
) -> Result<StatusCode, ServiceError> {
    let id = state.shortner.decode(&shorten).await?;
    state.backend.delete(id).await?;
    Ok(StatusCode::GONE)
}
