use std::borrow::BorrowMut;

use anyhow::Result;
use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use shortland::{service::application, settings::{Config, Backend}};
use tower::ServiceExt;

fn test_config() -> Config {
    let mut config = Config::default();
    let Backend::Redis(backend) = config.backend.borrow_mut();
    backend.connection = "redis://localhost:6379/1".to_owned();
    config
}

#[tokio::test]
async fn test_create_shorten() -> Result<()> {
    let config = test_config();
    let app = application(&config).await?;
    let response = app.oneshot(
        Request::builder()
            .uri("/urls")
            .method(Method::POST)
            .body(Body::from("http://example.com"))?,
    ).await?;
    assert_eq!(response.status(), StatusCode::CREATED);
    Ok(())
}

#[tokio::test]
async fn test_invalid_uri_create_shorten() -> Result<()> {
    let config = test_config();
    let app = application(&config).await?;
    let response = app.oneshot(
        Request::builder()
            .uri("/urls")
            .method(Method::POST)
            .body(Body::from("\\"))?,
    ).await?;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    Ok(())
}
