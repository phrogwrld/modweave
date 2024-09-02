mod args;
mod input;
mod scaffold;
mod utils;
mod versions;

use anyhow::Result;

fn main() -> Result<()> {

	let input = input::prompt()?;

	scaffold::create(input)?;

	success_message();

	Ok(())
}

fn success_message() {
	println!("Project created successfully!");
	println!("To build your mod, run `./gradlew build` in the project directory.");
	println!("For more information, visit https://fabricmc.net/wiki/tutorial:setup");
}
