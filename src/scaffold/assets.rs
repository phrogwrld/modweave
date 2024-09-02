use crate::input::UserInput;
use crate::utils::{create_directory, sanitize_mod_id};
use anyhow::{Context, Result};
use std::{fs, path::Path};

pub fn create(project_dir: &Path, input: &UserInput) -> Result<()> {
	let assets_dir = project_dir
		.join("src")
		.join("main")
		.join("resources")
		.join("assets");
	create_directory(&assets_dir)?;

	let mod_id = sanitize_mod_id(&input.location.mod_name);
	let mod_assets_dir = assets_dir.join(&mod_id);
	create_directory(&mod_assets_dir)?;

	let icon_src = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("templates")
		.join("common")
		.join("icon.png");
	let icon_dst = mod_assets_dir.join("icon.png");

	fs::copy(&icon_src, &icon_dst)
		.with_context(|| format!("Failed to copy icon.png to {}", icon_dst.display()))?;

	Ok(())
}
