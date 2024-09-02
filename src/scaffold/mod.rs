mod assets;
mod common_files;
pub mod git;
mod gradle;
mod main_class;
mod mod_file;
mod project_structure;

use crate::input::UserInput;
use anyhow::{Context, Result};
use std::{fs, path::Path};

pub fn create(input: UserInput) -> Result<()> {
	let project_dir = &input.location.path;

	if project_dir.exists() {
		fs::remove_dir_all(project_dir).context("Failed to remove existing project directory")?;
	}

	let result = (|| {
		project_structure::create(project_dir, &input)?;
		main_class::create(project_dir, &input)?;
		mod_file::create(project_dir, &input)?;
		gradle::create(project_dir, &input)?;
		assets::create(project_dir, &input)?;
		common_files::copy(project_dir)?;

		if input.git {
			git::create_repo(&input).context("Failed to initialize git repository")?;
		}

		Ok(())
	})();

	if result.is_err() {
		fs::remove_dir_all(project_dir)
			.context("Failed to remove project directory after error")?;
	}

	result
}

pub(crate) fn read_template(template_path: &str) -> Result<String> {
	let full_path = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("templates")
		.join(template_path);

	fs::read_to_string(&full_path)
		.with_context(|| format!("Failed to read template file at {}", full_path.display()))
}
