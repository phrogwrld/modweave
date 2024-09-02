use crate::input::UserInput;
use crate::utils::{get_java_version, sanitize_mod_id};
use anyhow::{Context, Result};
use std::{fs, path::Path};

pub fn create(project_dir: &Path, input: &UserInput) -> Result<()> {
	let template_content = super::read_template("fabric/fabric.mod.json")
		.context("Failed to read fabric.mod.json template")?;

	let mod_id = sanitize_mod_id(&input.location.mod_name);
	let replaced_content = template_content
		.replace("${MAIN_CLASS}", &input.location.mod_name)
		.replace("${MOD_ID}", &mod_id)
		.replace("${PACKAGE}", &input.maven_group)
		.replace("${VERSION}", &input.version)
		.replace("${MOD_NAME}", &input.location.mod_name)
		.replace("${DESCRIPTION}", &input.description)
		.replace("${AUTHOR}", &input.author)
		.replace("${LICENSE}", &input.license)
		.replace("${MINECRAFT_VERSION}", &input.minecraft_version)
		.replace(
			"${JAVA_VERSION}",
			&get_java_version(&input.minecraft_version)?,
		);

	let target_path = project_dir
		.join("src")
		.join("main")
		.join("resources")
		.join("fabric.mod.json");

	fs::write(&target_path, replaced_content)
		.with_context(|| format!("Failed to write mod file to {}", target_path.display()))?;

	Ok(())
}
