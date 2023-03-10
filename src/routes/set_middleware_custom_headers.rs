use crate::routes::read_middleware_custom_headers::HeaderMessage;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;

pub async fn set_middleware_custom_headers<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let message = headers.get("message").ok_or(StatusCode::BAD_REQUEST)?;
    let message = message
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .to_string();
    let extension = request.extensions_mut();
    extension.insert(HeaderMessage(message));
    Ok(next.run(request).await)
}
