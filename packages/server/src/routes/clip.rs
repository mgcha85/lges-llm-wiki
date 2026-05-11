use axum::{extract::Json, http::StatusCode};

use crate::services::clip as clip_service;
use crate::types::{ApiResponse, ClipRequest, ClipResponse, PendingClip};

pub async fn save_clip(
    Json(req): Json<ClipRequest>,
) -> Result<Json<ApiResponse<ClipResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let title = req.title;
    let url = req.url;
    let content = req.content;
    let project_name = req.project_name;

    tokio::task::spawn_blocking(move || {
        clip_service::save_clip(&title, &url, &content, project_name.as_deref())
    })
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Task join error: {}", e))),
        )
    })?
    .map(|path| Json(ApiResponse::success(ClipResponse { path })))
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(e))))
}

pub async fn get_pending_clips() -> Json<ApiResponse<Vec<PendingClip>>> {
    let clips = clip_service::get_pending_clips();
    Json(ApiResponse::success(clips))
}
