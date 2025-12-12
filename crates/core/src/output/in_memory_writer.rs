use std::fmt;
use std::sync::{Arc, Mutex};

use crate::errors::PPMResult;
use crate::output::OutputWriter;

/// In-memory output writer for testing
#[derive(Clone)]
pub struct InMemoryWriter {
	lines: Arc<Mutex<Vec<String>>>,
}

impl InMemoryWriter {
	pub fn new() -> Self {
		Self {
			lines: Arc::new(Mutex::new(Vec::new())),
		}
	}

	pub fn get_output(&self) -> Vec<String> {
		self.lines.lock().unwrap().clone()
	}

	pub fn clear(&self) {
		self.lines.lock().unwrap().clear();
	}
}

impl Default for InMemoryWriter {
	fn default() -> Self {
		Self::new()
	}
}

impl OutputWriter for InMemoryWriter {
	fn write(&self, message: &dyn fmt::Display) -> PPMResult<()> {
		let mut lines = self.lines.lock().unwrap();
		let formatted = format!("[ppm] {}", message);
		if let Some(last) = lines.last_mut() {
			last.push_str(&formatted);
		} else {
			lines.push(formatted);
		}
		Ok(())
	}

	fn write_line(&self, message: &dyn fmt::Display) -> PPMResult<()> {
		let mut lines = self.lines.lock().unwrap();
		lines.push(format!("[ppm] {}", message));
		Ok(())
	}
}
