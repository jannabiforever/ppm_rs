pub mod in_memory_writer;

use std::sync::Mutex;
use std::{fmt, io};

pub use in_memory_writer::InMemoryWriter;

use crate::errors::{PPMError, PPMResult};

pub trait OutputWriter: Send + Sync {
	fn write(&self, message: &dyn fmt::Display) -> PPMResult<()>;
	fn write_line(&self, message: &dyn fmt::Display) -> PPMResult<()>;
}

// --------------------------------------------------------------------------------
// Concrete Implementations
// --------------------------------------------------------------------------------

impl<W: io::Write + Send> OutputWriter for Mutex<W> {
	fn write(&self, message: &dyn fmt::Display) -> PPMResult<()> {
		let mut writer = self.lock().map_err(|_| PPMError::LockError)?;
		write!(writer, "{}", message)?;
		Ok(())
	}

	fn write_line(&self, message: &dyn fmt::Display) -> PPMResult<()> {
		let mut writer = self.lock().map_err(|_| PPMError::LockError)?;
		writeln!(writer, "{}", message)?;
		Ok(())
	}
}

pub fn stdout_writer() -> Mutex<io::Stdout> {
	Mutex::new(io::stdout())
}
