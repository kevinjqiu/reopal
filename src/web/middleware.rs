// Placeholder for middleware functionality
// This will be expanded for authentication, logging, etc.

use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn auth_middleware(
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    // For now, just pass through all requests
    // TODO: Implement proper authentication
    Ok(next.run(request).await)
}
