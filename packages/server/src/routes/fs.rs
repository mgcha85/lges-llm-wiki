use std::path::PathBuf;

use axum::{extract::Json, http::StatusCode};

use crate::config::data_dir;
use crate::services::fs as fs_service;
use crate::types::{
    ApiResponse, CreateDirectoryRequest, DeleteFileRequest, FileNode, ListDirectoryRequest,
    ReadFileRequest, ReadFileResponse, WriteFileRequest,
};

pub async fn read_file(
    Json(req): Json<ReadFileRequest>,
) -> Result<Json<ApiResponse<ReadFileResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let path = resolve_request_path(&req.path);

    tokio::task::spawn_blocking(move || fs_service::read_file(&path))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Task join error: {}", e))),
            )
        })?
        .map(|content| Json(ApiResponse::success(ReadFileResponse { content })))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(e))))
}

pub async fn write_file(
    Json(req): Json<WriteFileRequest>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    let path = resolve_request_path(&req.path);
    let contents = req.contents;

    tokio::task::spawn_blocking(move || fs_service::write_file(&path, &contents))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Task join error: {}", e))),
            )
        })?
        .map(|_| {
            Json(ApiResponse {
                ok: true,
                data: None,
                error: None,
            })
        })
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(e))))
}

pub async fn list_directory(
    Json(req): Json<ListDirectoryRequest>,
) -> Result<Json<ApiResponse<Vec<FileNode>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let path = resolve_request_path(&req.path);

    tokio::task::spawn_blocking(move || fs_service::list_directory(&path))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Task join error: {}", e))),
            )
        })?
        .map(|nodes| Json(ApiResponse::success(nodes)))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(e))))
}

pub async fn delete_file(
    Json(req): Json<DeleteFileRequest>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    let path = resolve_request_path(&req.path);

    tokio::task::spawn_blocking(move || fs_service::delete_file(&path))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Task join error: {}", e))),
            )
        })?
        .map(|_| {
            Json(ApiResponse {
                ok: true,
                data: None,
                error: None,
            })
        })
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(e))))
}

pub async fn create_directory(
    Json(req): Json<CreateDirectoryRequest>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    let path = resolve_request_path(&req.path);

    tokio::task::spawn_blocking(move || fs_service::create_directory(&path))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Task join error: {}", e))),
            )
        })?
        .map(|_| {
            Json(ApiResponse {
                ok: true,
                data: None,
                error: None,
            })
        })
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(e))))
}

fn resolve_request_path(path: &str) -> PathBuf {
    let p = PathBuf::from(path);
    if p.is_absolute() {
        p
    } else {
        data_dir().join(path)
    }
}
