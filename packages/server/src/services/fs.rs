use std::fs;
use std::path::Path;

use crate::config::{data_dir, validate_path};
use crate::types::FileNode;

pub fn read_file(path: &Path) -> Result<String, String> {
    let canonical = validate_path(&path.to_path_buf())?;
    fs::read_to_string(&canonical)
        .map_err(|e| format!("Failed to read file '{}': {}", path.display(), e))
}

pub fn write_file(path: &Path, contents: &str) -> Result<(), String> {
    let full_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        data_dir().join(path)
    };
    
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create parent dirs: {}", e))?;
    }
    
    if let Some(parent) = full_path.parent() {
        let parent_canonical = parent.canonicalize().map_err(|e| {
            format!("Invalid parent path '{}': {}", parent.display(), e)
        })?;
        if !parent_canonical.starts_with(data_dir()) {
            return Err(format!(
                "Access denied: path '{}' is outside DATA_DIR",
                full_path.display()
            ));
        }
    }
    
    fs::write(&full_path, contents)
        .map_err(|e| format!("Failed to write file '{}': {}", full_path.display(), e))
}

pub fn delete_file(path: &Path) -> Result<(), String> {
    let canonical = validate_path(&path.to_path_buf())?;
    
    if canonical.is_dir() {
        fs::remove_dir_all(&canonical)
            .map_err(|e| format!("Failed to delete directory '{}': {}", path.display(), e))
    } else {
        fs::remove_file(&canonical)
            .map_err(|e| format!("Failed to delete file '{}': {}", path.display(), e))
    }
}

pub fn create_directory(path: &Path) -> Result<(), String> {
    let full_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        data_dir().join(path)
    };
    
    if full_path.exists() {
        validate_path(&full_path)?;
    } else if let Some(parent) = full_path.parent() {
        if parent.exists() {
            validate_path(&parent.to_path_buf())?;
        }
    }
    
    fs::create_dir_all(&full_path)
        .map_err(|e| format!("Failed to create directory '{}': {}", full_path.display(), e))
}

pub fn list_directory(path: &Path) -> Result<Vec<FileNode>, String> {
    let canonical = validate_path(&path.to_path_buf())?;
    
    if !canonical.is_dir() {
        return Err(format!("Path is not a directory: '{}'", path.display()));
    }
    
    build_tree(&canonical, 0, 30)
}

fn build_tree(dir: &Path, depth: usize, max_depth: usize) -> Result<Vec<FileNode>, String> {
    if depth >= max_depth {
        return Ok(vec![]);
    }

    let mut entries: Vec<_> = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory '{}': {}", dir.display(), e))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_str()
                .map(|n| !n.starts_with('.'))
                .unwrap_or(false)
        })
        .collect();

    entries.sort_by(|a, b| {
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    let mut nodes = Vec::new();
    for entry in entries {
        let entry_path = entry.path();
        let name = entry
            .file_name()
            .to_str()
            .unwrap_or("")
            .to_string();
        let path_str = entry_path.to_string_lossy().replace('\\', "/");
        let is_dir = entry_path.is_dir();

        let children = if is_dir {
            let kids = build_tree(&entry_path, depth + 1, max_depth)?;
            if kids.is_empty() { None } else { Some(kids) }
        } else {
            None
        };

        nodes.push(FileNode {
            name,
            path: path_str,
            is_dir,
            children,
        });
    }

    Ok(nodes)
}

pub fn file_exists(path: &Path) -> bool {
    if let Ok(canonical) = validate_path(&path.to_path_buf()) {
        canonical.exists()
    } else {
        false
    }
}

pub fn copy_file(source: &Path, destination: &Path) -> Result<(), String> {
    let src_canonical = validate_path(&source.to_path_buf())?;
    
    let dest_path = if destination.is_absolute() {
        destination.to_path_buf()
    } else {
        data_dir().join(destination)
    };
    
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create parent dirs: {}", e))?;
    }
    
    fs::copy(&src_canonical, &dest_path)
        .map_err(|e| format!("Failed to copy '{}' to '{}': {}", source.display(), destination.display(), e))?;
    
    Ok(())
}

pub fn get_relative_path(full_path: &Path, base: &Path) -> String {
    full_path
        .strip_prefix(base)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| full_path.to_string_lossy().replace('\\', "/"))
}
