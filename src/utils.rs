use anyhow::{Context, Result};
use semver::Version;
use std::{fs, path::Path};

pub fn sanitize_mod_id(name: &str) -> String {
	name.to_lowercase()
		.chars()
		.filter(|&c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
		.collect()
}

pub fn create_directory(path: &Path) -> Result<()> {
	fs::create_dir_all(path)
		.with_context(|| format!("Failed to create directory: {}", path.display()))
}

pub fn get_java_version(minecraft_version: &str) -> Result<&'static str> {
	let version = Version::parse(minecraft_version).unwrap();

	Ok(if version >= Version::parse("1.20.5").unwrap() {
		"21"
	} else if version >= Version::parse("1.17.0").unwrap() {
		"17"
	} else {
		"8"
	})
}
