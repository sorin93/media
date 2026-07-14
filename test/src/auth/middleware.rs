use axum::{
    body::Body,
    extract::State,
    http::{HeaderValue, Request, header},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{auth::service, error::Error, state::AppState};

pub async fn inject_state(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    request.extensions_mut().insert(state);
    next.run(request).await
}

pub async fn guest_only(request: Request<Body>, next: Next) -> axum::response::Response {
    let is_auth = request
        .extensions()
        .get::<AppState>()
        .and_then(|state| {
            request
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer "))
                .map(|t| service::validate_access_token(state, t).is_ok())
        })
        .unwrap_or(false);

    if is_auth {
        return Error::BadRequest("Already authenticated".into()).into_response();
    }
    next.run(request).await
}

pub async fn auth_required(request: Request<Body>, next: Next) -> Result<Response, Error> {
    // TODO: Extract and validate token, add to request extensions
    Ok(next.run(request).await)
}
