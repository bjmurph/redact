use crate::Options;

#[derive(Default)]
pub struct Editor {
	file: Option<String>,
}

impl Editor {
	pub fn new(options: Options) -> Self {
		Self { file: options.file }
	}

	pub fn run(&mut self) {
		match &self.file {
			Some(file) => println!("Hello, {file}!"),
			None => println!("Hello, world!"),
		}
	}
}
