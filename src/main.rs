#![warn(clippy::all, clippy::pedantic)]

mod editor;
use std::env;

use editor::Editor;

fn main() {
	if env::args().any(|a| a == "--help" || a == "-h") {
		show_help();
		return;
	}

	if env::args().any(|a| a == "--version") {
		show_version();
		return;
	}

	let args: Vec<String> = env::args().collect();

	Editor::new(args).run();
}

fn show_help() {
	let binary_name = env!("CARGO_BIN_NAME");

	print!(
		"Usage: {binary_name} [arguments]

Arguments:
  --version        Print version information and exit
  -h or --help     Print this help message and exit
"
	);
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
