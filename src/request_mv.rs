use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
};
use tracing::info;

pub async fn request_mv(req: Request<Body>, next: Next) -> Response<Body> {
    let method = req.method().to_string();
    let uri = req.uri().to_string();
    info!("Incomming request: method={} uri={}", method, uri);

    let res = next.run(req).await;

    info!("Outgoing request: method={} uri={}", method, uri);
    return res;
}
