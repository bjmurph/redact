use std::io::Result;

use crossterm::event::{
	Event::{Key, Resize},
	KeyCode::Char,
	KeyEvent, KeyModifiers,
};

use crate::{input::TerminalInput, output::TerminalOutput, Options};

pub struct Editor<I: TerminalInput, O: TerminalOutput> {
	file: Option<String>,
	input: I,
	output: O,
}

impl<I, O> Editor<I, O>
where
	I: TerminalInput,
	O: TerminalOutput,
{
	pub fn new(options: Options, input: I, output: O) -> Self {
		Self {
			file: options.file,
			input,
			output,
		}
	}

	pub fn run(&mut self) -> Result<()> {
		let mut last_event = None;

		loop {
			match self.input.read_event()? {
				Some(Key(event)) => {
					last_event = Some(format!("{event:?}"));

					if let KeyEvent {
						code: Char('q'),
						modifiers: KeyModifiers::CONTROL,
						..
					} = event
					{
						break;
					}
				}
				Some(Resize(cols, rows)) => {
					self.output.set_size(cols, rows);
				}
				_ => (),
			}

			self.output.update_screen(
				self.file.as_deref().unwrap_or("New File"),
				last_event.as_deref(),
			)?;
		}

		Ok(())
	}
}
