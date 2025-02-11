use std::{io::Result, time::Duration};

use crossterm::{
	event::{
		poll, read,
		Event::{self},
	},
	terminal::{disable_raw_mode, enable_raw_mode},
};

pub trait TerminalInput {
	fn read_event(&self) -> Result<Option<Event>>;
}

pub struct Input;

impl Input {
	pub fn default() -> Result<Input> {
		enable_raw_mode()?;

		Ok(Input)
	}
}

impl TerminalInput for Input {
	fn read_event(&self) -> Result<Option<Event>> {
		if poll(Duration::from_millis(20))? {
			Ok(Some(read()?))
		} else {
			Ok(None)
		}
	}
}

impl Drop for Input {
	fn drop(&mut self) {
		disable_raw_mode().unwrap();
	}
}
