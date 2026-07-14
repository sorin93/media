// Basic tests for follow endpoints

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use uuid::Uuid;

mod test_utils;
use test_utils::*;

#[tokio::test]
async fn test_follow_requires_auth() {
    let mut app = create_test_app().await;
    let user_id = Uuid::new_v4();

    let request = Request::builder()
        .uri(format!("/followers/{}", user_id))
        .method("POST")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Should require authentication
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_unfollow_requires_auth() {
    let mut app = create_test_app().await;
    let user_id = Uuid::new_v4();

    let request = Request::builder()
        .uri(format!("/followers/{}", user_id))
        .method("DELETE")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_followers_requires_auth() {
    let mut app = create_test_app().await;

    let request = Request::builder()
        .uri("/followers")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
