use axum::{
    body::Body,
    extract::State,
    http::{HeaderValue, Method, Request, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use tower_http::cors::{AllowOrigin, CorsLayer};

use std::time::Duration;

use crate::{error::Error, services, state::AppState};

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
                .map(|t| services::auth::validate_access_token(state, t).is_ok())
        })
        .unwrap_or(false);

    if is_auth {
        return Error::BadRequest("Already authenticated".into()).into_response();
    }
    next.run(request).await
}

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_credentials(true)
        .allow_headers([
            header::ACCEPT,
            header::ACCEPT_ENCODING,
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
        ])
        .allow_methods([
            Method::DELETE,
            Method::GET,
            Method::OPTIONS,
            Method::PATCH,
            Method::POST,
            Method::PUT,
        ])
        .allow_origin(AllowOrigin::mirror_request())
        .max_age(Duration::from_secs(86400))
}

pub async fn security_headers(req: Request<Body>, next: Next) -> Response {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    // Content Security Policy: Default to blocking everything, allow only self.
    // Adjust "..." to your actual policy needs.
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static("default-src 'self'; frame-ancestors 'none';"),
    );

    // Referrer Policy: Don't send referrer information
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("no-referrer"),
    );

    // HSTS: Enforce HTTPS for 1 year, include subdomains, preload
    headers.insert(
        header::STRICT_TRANSPORT_SECURITY,
        HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
    );

    // No Sniff: Prevent browser from guessing content type
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );

    // X-Frame-Options: Prevent clickjacking (replaced by CSP frame-ancestors in modern browsers, but good backup)
    headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));

    // XSS Protection: Block rendering if XSS detected (Legacy, but still used)
    headers.insert(
        header::X_XSS_PROTECTION,
        HeaderValue::from_static("1; mode=block"),
    );

    response
}
