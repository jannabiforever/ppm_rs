use serde::{Deserialize, Serialize};

use crate::errors::{PPMError, PPMResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	pub default_focus_duration_in_minutes: u32,
	pub session_storage_path: String,
	pub task_storage_path: String,
	pub note_storage_path: String,
}

impl Config {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn load() -> PPMResult<Self> {
		Ok(Self::default())
	}

	pub fn validate(&self) -> PPMResult<()> {
		if self.default_focus_duration_in_minutes == 0 {
			return Err(PPMError::ConfigError(
				"default_focus_duration must be greater than 0".to_string(),
			));
		}
		Ok(())
	}
}

impl Default for Config {
	fn default() -> Self {
		let home = std::env::var("HOME").unwrap_or_else(|_| String::from("."));
		let session_storage_path = format!("{}/.config/ppm/sessions.json", home);
		let task_storage_path = format!("{}/.config/ppm/tasks.json", home);
		let note_storage_path = format!("{}/.config/ppm/notes.json", home);

		Self {
			default_focus_duration_in_minutes: 60,
			session_storage_path,
			task_storage_path,
			note_storage_path,
		}
	}
}
