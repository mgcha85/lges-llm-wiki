use std::fs;
use std::sync::Mutex;

use chrono::Local;

use crate::config::data_dir;
use crate::services::project::list_projects;
use crate::types::PendingClip;

static PENDING_CLIPS: Mutex<Vec<PendingClip>> = Mutex::new(Vec::new());

pub fn save_clip(
    title: &str,
    url: &str,
    content: &str,
    project_name: Option<&str>,
) -> Result<String, String> {
    let project_path = match project_name {
        Some(name) => data_dir().join(name),
        None => {
            let projects = list_projects()?;
            if projects.is_empty() {
                return Err("No projects available. Create a project first.".to_string());
            }
            std::path::PathBuf::from(&projects[0].path)
        }
    };

    if !project_path.exists() {
        return Err(format!("Project path does not exist: {}", project_path.display()));
    }

    let date = Local::now().format("%Y-%m-%d").to_string();
    let date_compact = Local::now().format("%Y%m%d").to_string();

    let slug_raw: String = title
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == ' ' || c == '-' {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
        .to_lowercase();
    let slug: String = slug_raw.chars().take(50).collect();

    let base_name = format!("{}-{}", slug, date_compact);
    let sources_dir = project_path.join("raw").join("sources");

    fs::create_dir_all(&sources_dir)
        .map_err(|e| format!("Failed to create directory: {}", e))?;

    let mut file_path = sources_dir.join(format!("{}.md", base_name));
    let mut counter = 2u32;
    while file_path.exists() {
        file_path = sources_dir.join(format!("{}-{}.md", base_name, counter));
        counter += 1;
    }

    let markdown = format!(
        r#"---
type: clip
title: "{}"
url: "{}"
clipped: {}
origin: web-clip
sources: []
tags: [web-clip]
---

# {}

Source: {}

{}
"#,
        title.replace('"', r#"\""#),
        url.replace('"', r#"\""#),
        date,
        title,
        url,
        content,
    );

    fs::write(&file_path, &markdown)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    let relative_path = file_path
        .strip_prefix(&project_path)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| file_path.to_string_lossy().replace('\\', "/"));

    if let Ok(mut pending) = PENDING_CLIPS.lock() {
        pending.push(PendingClip {
            project_path: project_path.to_string_lossy().replace('\\', "/"),
            file_path: file_path.to_string_lossy().replace('\\', "/"),
        });
    }

    Ok(relative_path)
}

pub fn get_pending_clips() -> Vec<PendingClip> {
    let mut pending = PENDING_CLIPS.lock().unwrap_or_else(|e| e.into_inner());
    std::mem::take(&mut *pending)
}
