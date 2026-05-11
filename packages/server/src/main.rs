mod config;
mod routes;
mod services;
mod types;

use axum::{
    body::Body,
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Router,
};
use rust_embed::Embed;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Embed)]
#[folder = "../web/dist"]
struct Assets;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "llm_wiki_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    config::init_data_dir().expect("Failed to initialize DATA_DIR");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/fs/read", post(routes::fs::read_file))
        .route("/api/fs/write", post(routes::fs::write_file))
        .route("/api/fs/list", post(routes::fs::list_directory))
        .route("/api/fs/delete", delete(routes::fs::delete_file))
        .route("/api/fs/mkdir", post(routes::fs::create_directory))
        .route("/api/project/list", get(routes::project::list_projects))
        .route("/api/project/create", post(routes::project::create_project))
        .route("/api/project/open", post(routes::project::open_project))
        .route("/api/upload", post(routes::upload::upload_files))
        .route("/api/clip", post(routes::clip::save_clip))
        .route("/api/clip/pending", get(routes::clip::get_pending_clips))
        .route("/api/llm/proxy", post(routes::llm::proxy_request))
        .route("/files/*file_path", get(routes::files::serve_file))
        .route("/health", get(health_check))
        .fallback(static_handler)
        .layer(cors);

    let port = config::server_port();
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("Starting server on {}", addr);
    tracing::info!("DATA_DIR: {}", config::data_dir().display());

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

async fn health_check() -> &'static str {
    "ok"
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    
    if let Some(content) = Assets::get(path) {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, mime.as_ref())
            .body(Body::from(content.data.into_owned()))
            .unwrap()
    } else if let Some(content) = Assets::get("index.html") {
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(Body::from(content.data.into_owned()))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap()
    }
}
