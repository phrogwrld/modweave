use anyhow::Result;
use crossterm::style::Stylize;
use inquire::{ui::RenderConfig, validator::Validation, Text};
use std::error::Error;

pub fn prompt(render_config: &RenderConfig) -> Result<String> {
	Text::new(
		&"What is your Maven group (e.g: com.example)?"
			.bold()
			.to_string(),
	)
	.with_validator(validate_maven_group)
	.with_render_config(render_config.clone())
	.prompt()
	.map_err(Into::into)
}

pub fn validate_maven_group(maven_group: &str) -> Result<Validation, Box<dyn Error + Send + Sync>> {
	if maven_group.is_empty() {
		return Ok(Validation::Invalid("Maven group cannot be empty".bold().into()));
	}

	if !maven_group
		.chars()
		.all(|c| c.is_ascii_alphanumeric() || c == '.')
	{
		return Ok(Validation::Invalid(
			"Maven group can only contain alphanumeric characters and dots".into(),
		));
	}

	if maven_group.starts_with('.') || maven_group.ends_with('.') {
		return Ok(Validation::Invalid(
			"Maven group cannot start or end with a dot".into(),
		));
	}

	Ok(Validation::Valid)
}
