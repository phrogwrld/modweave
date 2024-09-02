use crate::input::UserInput;
use crate::utils::create_directory;
use anyhow::Result;
use std::path::Path;

pub fn create(project_dir: &Path, input: &UserInput) -> Result<()> {
	let directories = [
		"src/main/java",
		"src/main/resources",
		"src/test/java",
		"src/test/resources",
	];

	for dir in &directories {
		create_directory(&project_dir.join(dir))?;
	}

	let package_path = input.maven_group.replace('.', "/");
	let main_class_path = project_dir
		.join("src")
		.join("main")
		.join("java")
		.join(&package_path);

	create_directory(&main_class_path)?;

	Ok(())
}
