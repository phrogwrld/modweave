#[derive(Debug)]
pub struct ModConfig {
	pub mod_id: String,
	pub name: String,
	pub description: String,
	pub version: String,
	pub authors: Vec<String>,
	pub license: String,
	pub minecraft_version: String,
	pub environment: ModEnvironment,
	pub entry_points: Vec<EntryPoint>,
}

#[derive(Debug)]
pub enum ModEnvironment {
	Client,
	Server,
	Universal,
}

#[derive(Debug)]
pub enum EntryPoint {
	Main,
	Client,
	Server,
}

fn sanitize_mod_id(name: &str) -> String {
	name.to_lowercase()
		.chars()
		.filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '_')
		.collect()
}
