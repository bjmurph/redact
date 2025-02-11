use std::io::{self, Result, Write};

use crossterm::{
	cursor, execute, queue, style,
	terminal::{
		self, BeginSynchronizedUpdate, ClearType, EndSynchronizedUpdate, EnterAlternateScreen,
		LeaveAlternateScreen,
	},
};

pub trait TerminalOutput {
	fn set_size(&mut self, cols: u16, rows: u16);
	fn update_screen(&self, status: &str, keypress: Option<&str>) -> Result<()>;
}

pub struct Output {
	cols: u16,
	rows: u16,
}

impl Output {
	pub fn default() -> Result<Output> {
		execute!(io::stdout(), EnterAlternateScreen)?;
		let (cols, rows) = terminal::size()?;

		Ok(Output { cols, rows })
	}
}

impl TerminalOutput for Output {
	fn set_size(&mut self, cols: u16, rows: u16) {
		self.cols = cols;
		self.rows = rows;
	}

	fn update_screen(&self, status: &str, keypress: Option<&str>) -> Result<()> {
		execute!(io::stdout(), BeginSynchronizedUpdate)?;
		queue!(
			io::stdout(),
			style::ResetColor,
			terminal::Clear(ClearType::All),
			cursor::Hide,
			cursor::MoveTo(0, 0)
		)?;

		if let Some(event) = keypress {
			queue!(io::stdout(), style::Print(event))?;
		}

		let rows = self.rows;

		queue!(io::stdout(), cursor::MoveTo(1, rows - 2))?;
		queue!(io::stdout(), style::Print(status))?;

		io::stdout().flush()?;
		execute!(io::stdout(), EndSynchronizedUpdate)?;

		Ok(())
	}
}

impl Drop for Output {
	fn drop(&mut self) {
		execute!(io::stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
	}
}
