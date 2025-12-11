use std::fmt::Display;

pub trait OutputWriter: Send + Sync {
	fn write(&self, message: &dyn Display);
	fn write_line(&self, message: &dyn Display);
	fn write_error(&self, message: &dyn Display);
}

// --------------------------------------------------------------------------------
// Concrete Implementations
// --------------------------------------------------------------------------------

pub struct StdoutWriter;

impl StdoutWriter {
	pub fn new() -> Self {
		Self
	}
}

impl Default for StdoutWriter {
	fn default() -> Self {
		Self::new()
	}
}

impl OutputWriter for StdoutWriter {
	fn write(&self, message: &dyn Display) {
		print!("{}", message);
	}

	fn write_line(&self, message: &dyn Display) {
		println!("{}", message);
	}

	fn write_error(&self, message: &dyn Display) {
		eprintln!("{}", message);
	}
}
