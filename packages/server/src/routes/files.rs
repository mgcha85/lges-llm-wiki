use axum::{
    body::Body,
    extract::Path,
    http::{header, StatusCode},
    response::Response,
};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::config::{data_dir, validate_path};

pub async fn serve_file(Path(file_path): Path<String>) -> Result<Response<Body>, StatusCode> {
    let decoded = urlencoding::decode(&file_path)
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_owned();

    let path = if std::path::Path::new(&decoded).is_absolute() {
        std::path::PathBuf::from(&decoded)
    } else {
        data_dir().join(&decoded)
    };

    let canonical = validate_path(&path).map_err(|_| StatusCode::FORBIDDEN)?;

    if !canonical.is_file() {
        return Err(StatusCode::NOT_FOUND);
    }

    let file = File::open(&canonical)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mime = mime_guess::from_path(&canonical)
        .first_or_octet_stream()
        .to_string();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime)
        .header(header::CACHE_CONTROL, "public, max-age=3600")
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
