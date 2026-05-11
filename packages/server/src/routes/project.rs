use axum::{extract::Json, http::StatusCode};

use crate::services::project as project_service;
use crate::types::{
    ApiResponse, CreateProjectRequest, OpenProjectRequest, ProjectListResponse, WikiProject,
};

pub async fn list_projects(
) -> Result<Json<ApiResponse<ProjectListResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    tokio::task::spawn_blocking(project_service::list_projects)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Task join error: {}", e))),
            )
        })?
        .map(|projects| Json(ApiResponse::success(ProjectListResponse { projects })))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(e))))
}

pub async fn create_project(
    Json(req): Json<CreateProjectRequest>,
) -> Result<Json<ApiResponse<WikiProject>>, (StatusCode, Json<ApiResponse<()>>)> {
    let name = req.name;

    tokio::task::spawn_blocking(move || project_service::create_project(&name))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Task join error: {}", e))),
            )
        })?
        .map(|project| Json(ApiResponse::success(project)))
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ApiResponse::error(e))))
}

pub async fn open_project(
    Json(req): Json<OpenProjectRequest>,
) -> Result<Json<ApiResponse<WikiProject>>, (StatusCode, Json<ApiResponse<()>>)> {
    let name = req.name;

    tokio::task::spawn_blocking(move || project_service::open_project(&name))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Task join error: {}", e))),
            )
        })?
        .map(|project| Json(ApiResponse::success(project)))
        .map_err(|e| (StatusCode::NOT_FOUND, Json(ApiResponse::error(e))))
}
