#[derive(Default)]
pub(crate) struct Editor {
	args: Vec<String>,
}

impl Editor {
	pub fn new(args: Vec<String>) -> Self {
		Self { args }
	}

	pub fn run(&mut self) {
		println!("Hello, {}!", self.args[0]);
	}
}
