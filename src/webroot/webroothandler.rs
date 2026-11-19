
use std::path::{Path, PathBuf};
use anyhow::{bail, Context, Result};

pub fn get_files(uri: &str) -> Result<Vec<u8>> {
    if uri.is_empty() {
        bail!("File path can't be empty");
    }

    let web_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("webroot")
        .join("static_files");

    let canonical_root = std::fs::canonicalize(&web_root)
        .context("Failed to resolve web root")?;

    let relative_path = uri.strip_prefix('/').unwrap_or(uri);

    let requested_path = Path::new(relative_path);

    if requested_path.components().any(|component| {
        matches!(
            component,
            std::path::Component::ParentDir
                | std::path::Component::RootDir
                | std::path::Component::Prefix(_)
        )
    }) {
        bail!("Invalid file path");
    }

    let file = canonical_root.join(requested_path);

    let canonical_file = std::fs::canonicalize(&file)
        .context("File not found or cannot be accessed")?;

    if !canonical_file.starts_with(&canonical_root) {
        bail!("Access denied");
    }

    let content = std::fs::read(&canonical_file)
        .context("Failed to read file")?;

    Ok(content)
}
