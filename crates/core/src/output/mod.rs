pub mod in_memory_writer;

use std::sync::Mutex;
use std::{fmt, io};

pub use in_memory_writer::InMemoryWriter;

use crate::errors::PPMResult;

/// Output abstraction for testability and flexibility.
///
/// Never use `println!` directly - always inject OutputWriter.
/// Uses interior mutability (Mutex) to allow &self methods on trait objects.
pub trait OutputWriter: Send + Sync {
	/// Write a message.
	///
	/// Note: no [ppm] header should be added to message. It is added automatically.
	fn write(&self, message: &dyn fmt::Display) -> PPMResult<()>;
	/// Write a message followed by a newline.
	///
	/// Note: no [ppm] header should be added to message. It is added automatically.
	fn write_line(&self, message: &dyn fmt::Display) -> PPMResult<()>;
}

// --------------------------------------------------------------------------------
// Concrete Implementations
// --------------------------------------------------------------------------------

impl<W: io::Write + Send> OutputWriter for Mutex<W> {
	fn write(&self, message: &dyn fmt::Display) -> PPMResult<()> {
		let mut writer = self.lock()?;
		write!(writer, "[ppm] {}", message)?;
		Ok(())
	}

	fn write_line(&self, message: &dyn fmt::Display) -> PPMResult<()> {
		let mut writer = self.lock()?;
		writeln!(writer, "[ppm] {}", message)?;
		Ok(())
	}
}

pub fn stdout_writer() -> Mutex<io::Stdout> {
	Mutex::new(io::stdout())
}
