use anyhow::{anyhow, Result};
use crossterm::style::Stylize;
use dunce::canonicalize;
use inquire::{ui::RenderConfig, validator::Validation, Confirm, Text};
use std::{fs, path::PathBuf};

use super::warn_render_config;

const DEFAULT_NAME: &str = "MyMod";
const VALID_CHARS: &[char] = &[
	'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
	't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
	'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
	'5', '6', '7', '8', '9',
];

#[derive(Debug)]
pub struct ProjectLocation {
	pub mod_name: String,
	pub path: PathBuf,
}

impl ProjectLocation {
	pub fn new(mod_name: &str) -> Result<Self> {
		let sanitized_name = sanitize_mod_name(mod_name);
		let path = PathBuf::from(&sanitized_name);

		if path.exists() && path.is_file() {
			return Err(anyhow!(
				"'{}' is an existing file, please provide a valid mod name",
				mod_name
			));
		}

		let canonical_path = if path.exists() {
			canonicalize(&path)?
		} else {
			fs::create_dir_all(&path)?;
			let res = canonicalize(&path);
			fs::remove_dir(&path)?;
			res?
		};

		let location = Self {
			mod_name: sanitized_name,
			path: canonical_path,
		};
		location.validate()?;

		Ok(location)
	}

	fn validate(&self) -> Result<()> {
		if self.mod_name.chars().all(|c| VALID_CHARS.contains(&c)) {
			Ok(())
		} else {
			Err(anyhow!(
				"Mod name '{}' must only contain alphanumeric characters",
				self.mod_name
			))
		}
	}
}

pub fn prompt(render_config: &RenderConfig) -> Result<ProjectLocation> {
	let location = loop {
		let input = Text::new(&format!("{}", "What's the name of your Fabric mod?".bold()))
			.with_default(DEFAULT_NAME)
			.with_render_config(*render_config)
			.with_validator(|input: &str| match ProjectLocation::new(input) {
				Ok(_) => Ok(Validation::Valid),
				Err(e) => Ok(Validation::Invalid(e.to_string().into())),
			})
			.prompt()?;

		let location = ProjectLocation::new(&input)?;

		if location.path.exists() {
			let should_continue = Confirm::new(&format!(
				"{}",
				format!(
					"'{}' already exists. Do you want to use it anyway?",
					location.path.display()
				)
				.bold()
			))
			.with_default(false)
			.with_render_config(warn_render_config())
			.prompt()?;

			if should_continue {
				break location;
			}
		} else {
			break location;
		}
	};

	Ok(location)
}

fn sanitize_mod_name(name: &str) -> String {
	name.chars().filter(|c| c.is_alphanumeric()).collect()
}
