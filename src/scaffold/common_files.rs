use anyhow::{Context, Result};
use std::{fs, path::Path};

pub fn copy(project_dir: &Path) -> Result<()> {
	let common_files = [
		(".gitignore.template", ".gitignore"),
		(".gitattributes.template", ".gitattributes"),
	];

	for (src, dst) in &common_files {
		fs::copy(
			Path::new(env!("CARGO_MANIFEST_DIR"))
				.join("templates")
				.join("common")
				.join(src),
			project_dir.join(dst),
		)
		.with_context(|| format!("Failed to copy {} to {}", src, dst))?;
	}

	Ok(())
}
