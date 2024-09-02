use anyhow::Result;
use crossterm::style::{style, Stylize};
use inquire::{ui::RenderConfig, Select};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum GradleDsl {
	Groovy,
	Kotlin,
}

impl GradleDsl {
	pub const fn as_str(self) -> &'static str {
		match self {
			GradleDsl::Groovy => "groovy",
			GradleDsl::Kotlin => "kotlin",
		}
	}
}

impl fmt::Display for GradleDsl {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

pub fn prompt(render_config: &RenderConfig) -> Result<GradleDsl> {
	println!("{}", style("Select Gradle DSL:").bold());

	let options = [GradleDsl::Groovy, GradleDsl::Kotlin];
	Select::new("", options.to_vec())
		.with_render_config(*render_config)
		.prompt()
		.map_err(Into::into)
}
