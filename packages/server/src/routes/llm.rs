use axum::{
    body::Body,
    extract::Json,
    http::{header, HeaderMap, HeaderValue, Method, StatusCode},
    response::Response,
};
use futures::TryStreamExt;

use crate::types::{ApiResponse, LlmProxyRequest};

pub async fn proxy_request(
    Json(req): Json<LlmProxyRequest>,
) -> Result<Response<Body>, (StatusCode, Json<ApiResponse<()>>)> {
    let client = reqwest::Client::new();

    let method = req
        .method
        .as_deref()
        .unwrap_or("POST")
        .parse::<Method>()
        .unwrap_or(Method::POST);

    let mut request_builder = client.request(method, &req.url);

    if let Some(headers) = &req.headers {
        for (key, value) in headers {
            request_builder = request_builder.header(key, value);
        }
    }

    if let Some(body) = &req.body {
        request_builder = request_builder.body(body.clone());
    }

    let response = request_builder.send().await.map_err(|e| {
        (
            StatusCode::BAD_GATEWAY,
            Json(ApiResponse::error(format!("Upstream request failed: {}", e))),
        )
    })?;

    let status = StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::OK);

    let mut response_headers = HeaderMap::new();
    for (key, value) in response.headers() {
        if let Ok(name) = header::HeaderName::try_from(key.as_str()) {
            if let Ok(val) = HeaderValue::from_bytes(value.as_bytes()) {
                response_headers.insert(name, val);
            }
        }
    }

    let stream = response.bytes_stream().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    });

    let body = Body::from_stream(stream);

    let mut builder = Response::builder().status(status);

    for (key, value) in response_headers {
        if let Some(k) = key {
            builder = builder.header(k, value);
        }
    }

    builder.body(body).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Failed to build response: {}", e))),
        )
    })
}
