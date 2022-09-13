use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    Json,
};

use crate::{backend::Backend, service, shortener::Shortner};

pub async fn create_shorten<S: Shortner, B: Backend>(
    State(state): State<Arc<service::State<S, B>>>,
    Json(url): Json<String>,
) -> (StatusCode, String) {
    let id = state.backend.store(&url).await.unwrap();
    (StatusCode::CREATED, state.shortner.encode(id).await)
}

pub async fn expand_shorten<S: Shortner, B: Backend>(
    State(state): State<Arc<service::State<S, B>>>,
    Path(shorten): Path<String>,
) -> (StatusCode, HeaderMap) {
    let id = state.shortner.decode(&shorten).await;
    let url = state.backend.retrive(id).await.unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(header::LOCATION, url.parse().unwrap());
    (StatusCode::TEMPORARY_REDIRECT, headers)
}

pub async fn get_stat_by_shorten<S: Shortner, B: Backend>(
    State(state): State<Arc<service::State<S, B>>>,
    Path(shorten): Path<String>,
) -> String {
    let id = state.shortner.decode(&shorten).await;
    let stat = state.backend.stat(id, None).await.unwrap();
    stat.to_string()
}

pub async fn update_shorten<S: Shortner, B: Backend>(
    State(state): State<Arc<service::State<S, B>>>,
    Path(shorten): Path<String>,
    Json(url): Json<String>
    ) -> StatusCode {
    let id = state.shortner.decode(&shorten).await;
    state.backend.update(id, &url).await.unwrap();
    StatusCode::CREATED
}

pub async fn delete_shorten<S: Shortner, B: Backend>(
    State(state): State<Arc<service::State<S, B>>>,
    Path(shorten): Path<String>,
    ) -> StatusCode {
    let id = state.shortner.decode(&shorten).await;
    state.backend.delete(id).await.unwrap();
    StatusCode::GONE
}
