use std::{
	io::{self, Write},
	time::Duration,
};

use crossterm::event::Event::Resize;
use crossterm::event::KeyCode::Char;
use crossterm::{
	cursor,
	event::Event::Key,
	queue, style,
	terminal::{self, BeginSynchronizedUpdate, ClearType, EndSynchronizedUpdate},
};
use crossterm::{
	event::{poll, read, KeyEvent, KeyModifiers},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

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
		execute!(io::stdout(), EnterAlternateScreen).unwrap();

		enable_raw_mode().unwrap();

		let (mut _cols, mut rows) = terminal::size().unwrap();
		let mut last_event = None;

		loop {
			if is_event_available().unwrap() {
				match read() {
					Ok(Key(event)) => {
						last_event = Some(format!("{event:?}"));

						if let KeyEvent {
							code: Char('q'),
							modifiers: KeyModifiers::CONTROL,
							..
						} = event
						{
							execute!(io::stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
							disable_raw_mode().unwrap();
							break;
						}
					}
					Ok(Resize(c, r)) => {
						(_cols, rows) = (c, r);
					}
					Err(err) => println!("Error: {err}"),
					_ => (),
				}
			}

			execute!(io::stdout(), BeginSynchronizedUpdate).unwrap();

			queue!(
				io::stdout(),
				style::ResetColor,
				terminal::Clear(ClearType::All),
				cursor::Hide,
				cursor::MoveTo(0, 0)
			)
			.unwrap();

			if let Some(event) = &last_event {
				queue!(io::stdout(), style::Print(event)).unwrap();
			}

			queue!(io::stdout(), cursor::MoveTo(1, rows - 1)).unwrap();

			match &self.file {
				Some(file) => queue!(io::stdout(), style::Print(file)).unwrap(),
				None => queue!(io::stdout(), style::Print("New File")).unwrap(),
			}

			io::stdout().flush().unwrap();

			execute!(io::stdout(), EndSynchronizedUpdate).unwrap();
		}
	}
}

fn is_event_available() -> io::Result<bool> {
	poll(Duration::from_millis(20))
}
