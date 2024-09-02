use anyhow::{anyhow, Context, Result};
use std::process::{Command, Stdio};

use crate::input::UserInput;

#[derive(Debug)]
pub enum GitStep {
	Init,
	Add,
	Commit,
}

impl GitStep {
	fn args(&self) -> Vec<&str> {
		match self {
			GitStep::Init => vec!["init"],
			GitStep::Add => vec!["add", "."],
			GitStep::Commit => vec!["commit", "-m", "init"],
		}
	}
}

pub fn create_repo(input: &UserInput) -> Result<()> {
	if !input.git {
		return Ok(());
	}

	for step in &[GitStep::Init, GitStep::Add, GitStep::Commit] {
		run_git_commmand(input, step)
			.with_context(|| format!("Failed to run git command: {:?}", step))?;
	}

	Ok(())
}

fn run_git_commmand(input: &UserInput, step: &GitStep) -> Result<()> {
	let status = Command::new("git")
		.args(step.args())
		.current_dir(&input.location.path)
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.status()?;

	if !status.success() {
		return Err(anyhow!("Failed to run git command: {:?}", step));
	}

	Ok(())
}
