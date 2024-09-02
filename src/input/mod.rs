pub mod gradle_dsl;
pub mod maven_group;
pub mod project_location;

use self::gradle_dsl::GradleDsl;
use self::project_location::ProjectLocation;
use crate::versions::VersionManager;

use anyhow::Result;
use crossterm::style::Stylize;
use inquire::{
	ui::{Color, RenderConfig, Styled},
	Confirm, Select, Text,
};

#[derive(Debug)]
pub struct UserInput {
	pub location: ProjectLocation,
	pub maven_group: String,
	pub gradle_dsl: GradleDsl,
	pub minecraft_version: String,
	pub yarn_version: String,
	pub fabric_loader_version: String,
	pub fabric_api_version: String,
	pub version: String,
	pub description: String,
	pub author: String,
	pub license: String,
	pub git: bool,
}

pub fn prompt() -> Result<UserInput> {
	let render_config = RenderConfig::default()
		.with_prompt_prefix(Styled::new("›").with_fg(Color::DarkMagenta))
		.with_answered_prompt_prefix(Styled::new("•").with_fg(Color::LightMagenta))
		.with_scroll_up_prefix(Styled::new("↑").with_fg(Color::DarkMagenta))
		.with_scroll_down_prefix(Styled::new("↓").with_fg(Color::DarkMagenta));

	let version_manager = VersionManager::new();
	let versions = version_manager.fetch_all_versions()?;

	println!(
		"\n{}",
		"🚀 Let's set up your Fabric mod project!".cyan().bold()
	);

	let location = project_location::prompt(&render_config)?;
	let maven_group = maven_group::prompt(&render_config)?;
	let gradle_dsl = gradle_dsl::prompt(&render_config)?;

	println!(
		"\n{}",
		"📦 Now, let's choose your mod's dependencies:"
			.green()
			.bold()
	);

	let minecraft_version = Select::new(
		&"Minecraft version:".bold().to_string(),
		versions
			.minecraft
			.into_iter()
			.filter(|(_, stable)| *stable)
			.map(|(v, _)| v)
			.collect(),
	)
	.with_render_config(render_config.clone())
	.prompt()?;

	let compatible_yarn_versions =
		version_manager.get_compatible_yarn_versions(&minecraft_version)?;
	let yarn_version = Select::new(
		&"Yarn mappings version:".bold().to_string(),
		compatible_yarn_versions,
	)
	.with_render_config(render_config.clone())
	.prompt()?;

	let fabric_loader_version = Select::new(
		&"Fabric Loader version:".bold().to_string(),
		versions
			.loader
			.into_iter()
			.filter(|(_, stable)| *stable)
			.map(|(v, _)| v)
			.collect(),
	)
	.with_render_config(render_config.clone())
	.prompt()?;

	let compatible_fabric_api_versions =
		version_manager.get_compatible_fabric_api_versions(&minecraft_version)?;
	let fabric_api_version = Select::new(
		&"Fabric API version:".bold().to_string(),
		compatible_fabric_api_versions,
	)
	.with_render_config(render_config.clone())
	.prompt()?;

	println!(
		"\n{}",
		"📝 Let's add some details about your mod:".yellow().bold()
	);

	let version = Text::new(&"Mod version:".bold().to_string())
		.with_default("0.1.0")
		.with_render_config(render_config.clone())
		.prompt()?;
	let description = Text::new(&"Mod description:".bold().to_string())
		.with_render_config(render_config.clone())
		.prompt()?;
	let author = Text::new(&"Mod author:".bold().to_string())
		.with_render_config(render_config.clone())
		.prompt()?;
	let license = Text::new(&"Mod license:".bold().to_string())
		.with_default("MIT")
		.with_render_config(render_config.clone())
		.prompt()?;

	let git = Confirm::new(&"Initialize Git repository?".bold().to_string())
		.with_default(false)
		.with_render_config(render_config)
		.prompt()?;

	println!(
		"\n{}",
		"✨ Great! Your mod project is ready to be created."
			.magenta()
			.bold()
	);

	Ok(UserInput {
		location,
		maven_group,
		gradle_dsl,
		minecraft_version,
		yarn_version,
		fabric_loader_version,
		fabric_api_version,
		version,
		description,
		author,
		license,
		git,
	})
}

pub fn warn_render_config() -> RenderConfig<'static> {
	RenderConfig::default()
		.with_prompt_prefix(Styled::new("⚠").with_fg(inquire::ui::Color::DarkYellow))
}
