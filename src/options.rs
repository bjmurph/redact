use bpaf::{Bpaf, ShellComp};

#[derive(Bpaf)]
#[bpaf(options)]
pub struct Options {
	pub version: bool,
	/// The file to edit
	#[bpaf(positional("FILE"), complete_shell(ShellComp::File{mask: None}))]
	pub file: Option<String>,
}
