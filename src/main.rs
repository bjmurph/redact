#![warn(clippy::all, clippy::pedantic)]

mod editor;
mod input;
mod options;
mod output;
use std::{env, process::ExitCode};

use editor::Editor;
use input::Input;
use options::{options, Options};
use output::Output;

fn main() -> ExitCode {
	let options = options().run();

	if options.version {
		show_version();
		return ExitCode::SUCCESS;
	}

	match Editor::new(
		options,
		Input::default().unwrap(),
		Output::default().unwrap(),
	)
	.run()
	{
		Ok(()) => ExitCode::SUCCESS,
		Err(_) => ExitCode::FAILURE,
	}
}

fn show_version() {
	let binary_name = env!("CARGO_BIN_NAME");
	let version = env!("CARGO_PKG_VERSION");

	let git_sha = env!("VERGEN_GIT_SHA");
	let commit_date = env!("VERGEN_GIT_COMMIT_DATE");

	let dirty_flag = match env!("VERGEN_GIT_DIRTY") {
		"false" => "",
		&_ => "+",
	};

	println!("{binary_name} {version} ({git_sha}{dirty_flag} {commit_date})");
}
