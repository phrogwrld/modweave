use crate::input::UserInput;
use crate::utils::sanitize_mod_id;
use anyhow::{Context, Result};
use std::{fs, path::Path};

pub fn create(project_dir: &Path, input: &UserInput) -> Result<()> {
	let package_path = input.maven_group.replace('.', "/");
	let main_class_path = project_dir
		.join("src")
		.join("main")
		.join("java")
		.join(&package_path);

	let class_name = &input.location.mod_name;
	let file_path = main_class_path.join(format!("{}.java", class_name));

	let template_content = super::read_template("fabric/java/MainClass.java.template")
		.context("Failed to read MainClass.java template")?;

	let content = template_content
		.replace("${PACKAGE}", &input.maven_group)
		.replace("${CLASS_NAME}", class_name)
		.replace("${MOD_ID}", &sanitize_mod_id(&input.location.mod_name));

	fs::write(&file_path, content)
		.with_context(|| format!("Failed to write main class file to {}", file_path.display()))?;

	Ok(())
}
