use crate::dev::state::ProxyState;
use axum::{
    body::to_bytes, body::Body, debug_handler, extract::Request, response::Response, Extension,
};
use hyper::header::{CONTENT_LENGTH, TRANSFER_ENCODING};

#[debug_handler]
pub async fn fallback_handler(
    Extension(state): Extension<ProxyState>,
    mut req: Request<Body>,
) -> Response<Body> {
    let method = req.method_mut().clone();
    let path = req.uri_mut().clone();
    let path = path.path();
    let headers = req.headers_mut().clone();
    let body = req.into_body();

    tracing::info!("{method} -> {path}");

    let res = state
        .request(
            method,
            path,
            None,
            Some(to_bytes(body, usize::MAX).await.unwrap()),
            Some(headers),
        )
        .await
        .unwrap();

    let mut builder = Response::builder().status(res.status());

    for key in res.headers().keys() {
        if *key == CONTENT_LENGTH || *key == TRANSFER_ENCODING {
            continue;
        }

        builder = builder.header(key, res.headers().get(key).unwrap());
    }

    builder
        .body(Body::from(res.bytes().await.unwrap()))
        .unwrap()
}
