use std::process::Command;
use std::{env, fs};

use crate::errors::{PPMError, PPMResult};

/// Abstraction for opening external text editors.
///
/// Enables testing by allowing mock implementations that return predetermined content.
pub trait Editor: Send + Sync {
	/// Opens an editor with optional initial content.
	/// Returns the edited content, or None if the user cancels/saves empty file.
	fn open(&self, initial_content: Option<&str>) -> PPMResult<Option<String>>;
}

// --------------------------------------------------------------------------------
// Concrete Implementations
// --------------------------------------------------------------------------------

/// System editor that spawns the user's preferred text editor.
///
/// Tries in order:
/// 1. VISUAL environment variable
/// 2. EDITOR environment variable
/// 3. vim (default for CLI tools)
pub struct SystemEditor;

impl SystemEditor {
	pub fn new() -> Self {
		Self
	}

	fn get_editor_command(&self) -> String {
		env::var("VISUAL").or_else(|_| env::var("EDITOR")).unwrap_or_else(|_| String::from("vim"))
	}
}

impl Editor for SystemEditor {
	fn open(&self, initial_content: Option<&str>) -> PPMResult<Option<String>> {
		// Create temporary file
		let temp_dir = env::temp_dir();
		let timestamp =
			std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros();
		let temp_file = temp_dir.join(format!("ppm_note_{}.txt", timestamp));

		// Write initial content if provided
		if let Some(content) = initial_content {
			fs::write(&temp_file, content)?;
		}

		// Spawn editor
		let editor = self.get_editor_command();
		let status = Command::new(&editor).arg(&temp_file).status().map_err(|e| {
			PPMError::EditorError(format!("Failed to open editor '{}': {}", editor, e))
		})?;

		if !status.success() {
			fs::remove_file(&temp_file)?;
			return Err(PPMError::EditorError(format!(
				"Editor '{}' exited with non-zero status",
				editor
			)));
		}

		// Read edited content
		let content = fs::read_to_string(&temp_file)?;

		// Clean up
		fs::remove_file(&temp_file)?;

		// Return None if content is empty (user didn't write anything)
		if content.trim().is_empty() {
			Ok(None)
		} else {
			Ok(Some(content.trim().to_string()))
		}
	}
}

impl Default for SystemEditor {
	fn default() -> Self {
		Self::new()
	}
}
