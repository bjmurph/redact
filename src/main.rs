#![warn(clippy::all, clippy::pedantic)]

mod editor;
mod options;
use std::env;

use editor::Editor;
use options::{options, Options};

fn main() {
	let options = options().run();

	if options.version {
		show_version();
		return;
	}

	Editor::new(options).run();
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
