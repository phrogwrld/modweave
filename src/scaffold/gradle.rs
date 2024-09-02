use crate::input::{gradle_dsl::GradleDsl, UserInput};
use crate::utils::{create_directory, get_java_version};
use anyhow::{Context, Result};
use std::{fs, path::Path};

pub fn create(project_dir: &Path, input: &UserInput) -> Result<()> {
	let dsl_subdir = match input.gradle_dsl {
		GradleDsl::Groovy => "groovy",
		GradleDsl::Kotlin => "kotlin",
	};

	create_build_gradle(project_dir, input, dsl_subdir)?;
	create_gradle_properties(project_dir, input)?;
	copy_wrapper_files(project_dir)?;
	copy_settings_file(project_dir, dsl_subdir)?;

	Ok(())
}

fn create_build_gradle(project_dir: &Path, input: &UserInput, dsl_subdir: &str) -> Result<()> {
	let build_template_path = format!(
		"gradle/dsl/{}/build.gradle{}",
		dsl_subdir,
		if matches!(input.gradle_dsl, GradleDsl::Kotlin) {
			".kts"
		} else {
			""
		}
	);

	let build_content = super::read_template(&build_template_path)
		.context("Failed to read build.gradle template")?;

	let java_version = get_java_version(&input.minecraft_version)?;
	let processed_build_content = build_content.replace("${{ java_version }}", java_version);

	let build_file_name = match input.gradle_dsl {
		GradleDsl::Groovy => "build.gradle",
		GradleDsl::Kotlin => "build.gradle.kts",
	};

	fs::write(project_dir.join(build_file_name), processed_build_content)
		.context("Failed to write build.gradle file")?;

	Ok(())
}

fn create_gradle_properties(project_dir: &Path, input: &UserInput) -> Result<()> {
	let gradle_properties_content = super::read_template("gradle/gradle.properties")
		.context("Failed to read gradle.properties template")?;

	let processed_gradle_properties = gradle_properties_content
		.replace("${{ base_name }}", &input.location.mod_name.to_lowercase())
		.replace("${{ maven_group }}", &input.maven_group)
		.replace("${{ mod_name }}", &input.location.mod_name)
		.replace("${{ mod_version }}", &input.version)
		.replace("${{ minecraft_version }}", &input.minecraft_version)
		.replace("${{ yarn_mappings }}", &input.yarn_version)
		.replace("${{ fabric_api_version }}", &input.fabric_api_version)
		.replace("${{ fabric_loader_version }}", &input.fabric_loader_version);

	fs::write(
		project_dir.join("gradle.properties"),
		processed_gradle_properties,
	)
	.context("Failed to write gradle.properties file")?;

	Ok(())
}

fn copy_wrapper_files(project_dir: &Path) -> Result<()> {
	let wrapper_dir = project_dir.join("gradle").join("wrapper");
	create_directory(&wrapper_dir)?;

	let wrapper_files = [
		(
			"gradle/wrapper/gradle-wrapper.jar",
			"gradle/wrapper/gradle-wrapper.jar",
		),
		(
			"gradle/wrapper/gradle-wrapper.properties",
			"gradle/wrapper/gradle-wrapper.properties",
		),
		("gradle/gradlew", "gradlew"),
		("gradle/gradlew.bat", "gradlew.bat"),
	];

	for (src, dst) in &wrapper_files {
		fs::copy(
			Path::new(env!("CARGO_MANIFEST_DIR"))
				.join("templates")
				.join(src),
			project_dir.join(dst),
		)
		.with_context(|| format!("Failed to copy {} to {}", src, dst))?;
	}

	Ok(())
}

fn copy_settings_file(project_dir: &Path, dsl_subdir: &str) -> Result<()> {
	let settings_file = match dsl_subdir {
		"groovy" => "settings.gradle",
		"kotlin" => "settings.gradle.kts",
		_ => unreachable!(),
	};

	fs::copy(
		Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("templates")
			.join("gradle")
			.join("dsl")
			.join(dsl_subdir)
			.join(settings_file),
		project_dir.join(settings_file),
	)
	.with_context(|| format!("Failed to copy {}", settings_file))?;

	Ok(())
}
