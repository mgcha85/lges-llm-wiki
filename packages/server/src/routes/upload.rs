use std::path::PathBuf;

use axum::{
    extract::{Multipart, Query},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::config::data_dir;
use crate::types::ApiResponse;

#[derive(Debug, Deserialize)]
pub struct UploadQuery {
    pub project: String,
    #[serde(default)]
    pub subdir: Option<String>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadResponse {
    pub files: Vec<UploadedFile>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadedFile {
    pub name: String,
    pub path: String,
    pub size: u64,
}

pub async fn upload_files(
    Query(query): Query<UploadQuery>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<UploadResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let project_dir = data_dir().join(&query.project);
    
    if !project_dir.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error(format!("Project '{}' not found", query.project))),
        ));
    }

    let target_dir = match &query.subdir {
        Some(subdir) => project_dir.join(subdir),
        None => project_dir.join("raw/sources"),
    };

    fs::create_dir_all(&target_dir).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Failed to create upload dir: {}", e))),
        )
    })?;

    let mut uploaded_files = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(format!("Failed to read multipart field: {}", e))),
        )
    })? {
        let file_name = match field.file_name() {
            Some(name) => sanitize_filename(name),
            None => continue,
        };

        if file_name.is_empty() {
            continue;
        }

        let file_path = find_unique_path(&target_dir, &file_name).await;
        
        let data = field.bytes().await.map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error(format!("Failed to read file data: {}", e))),
            )
        })?;

        let size = data.len() as u64;

        let mut file = fs::File::create(&file_path).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to create file: {}", e))),
            )
        })?;

        file.write_all(&data).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to write file: {}", e))),
            )
        })?;

        let relative_path = file_path
            .strip_prefix(data_dir())
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| file_path.to_string_lossy().replace('\\', "/"));

        uploaded_files.push(UploadedFile {
            name: file_name,
            path: relative_path,
            size,
        });
    }

    Ok(Json(ApiResponse::success(UploadResponse {
        files: uploaded_files,
    })))
}

fn sanitize_filename(name: &str) -> String {
    let name = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' || c == ' ' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>();

    name.trim().to_string()
}

async fn find_unique_path(dir: &PathBuf, name: &str) -> PathBuf {
    let path = dir.join(name);
    if !path.exists() {
        return path;
    }

    let stem = std::path::Path::new(name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(name);
    let ext = std::path::Path::new(name)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| format!(".{}", s))
        .unwrap_or_default();

    let mut counter = 2u32;
    loop {
        let new_name = format!("{}-{}{}", stem, counter, ext);
        let new_path = dir.join(&new_name);
        if !new_path.exists() {
            return new_path;
        }
        counter += 1;
    }
}
