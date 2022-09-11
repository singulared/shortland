use std::sync::Arc;

use axum::{extract::{State, Path}, response::IntoResponse, Json};

use crate::{
    backend::Backend, service, shortener::Shortner,
};

pub async fn create_shorten<S: Shortner, B: Backend>(
    State(state): State<Arc<service::State<S, B>>>,
    Json(url): Json<String>,
) -> impl IntoResponse {
    dbg!(&state.config);
    let id = state.backend.store(&url).await.unwrap();
    state.shortner.encode(id).await
}

pub async fn expand_shorten<S: Shortner, B: Backend>(
    State(state): State<Arc<service::State<S, B>>>,
    Path(shorten): Path<String>,
) -> impl IntoResponse {
    let id = state.shortner.decode(&shorten).await;
    state.backend.retrive(id).await.unwrap()
}
