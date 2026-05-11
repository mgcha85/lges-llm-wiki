use std::path::PathBuf;
use std::sync::OnceLock;

static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Initialize the DATA_DIR from environment variable.
/// Must be called once at server startup.
pub fn init_data_dir() -> Result<(), String> {
    let dir = std::env::var("DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./data"));

    if !dir.exists() {
        std::fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create DATA_DIR '{}': {}", dir.display(), e))?;
    }

    if !dir.is_dir() {
        return Err(format!("DATA_DIR '{}' is not a directory", dir.display()));
    }

    DATA_DIR
        .set(dir.canonicalize().map_err(|e| e.to_string())?)
        .map_err(|_| "DATA_DIR already initialized".to_string())
}

/// Get the DATA_DIR path. Panics if not initialized.
pub fn data_dir() -> &'static PathBuf {
    DATA_DIR.get().expect("DATA_DIR not initialized - call init_data_dir() first")
}

/// Validate that a path is within DATA_DIR (prevent path traversal).
pub fn validate_path(path: &PathBuf) -> Result<PathBuf, String> {
    let canonical = path.canonicalize().map_err(|e| {
        format!("Invalid path '{}': {}", path.display(), e)
    })?;

    if !canonical.starts_with(data_dir()) {
        return Err(format!(
            "Access denied: path '{}' is outside DATA_DIR",
            path.display()
        ));
    }

    Ok(canonical)
}

/// Resolve a relative path within DATA_DIR.
pub fn resolve_path(relative: &str) -> PathBuf {
    data_dir().join(relative)
}

/// Get server port from environment or default.
pub fn server_port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3001)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_resolve_path() {
    }
}
