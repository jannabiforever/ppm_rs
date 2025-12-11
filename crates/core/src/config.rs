use serde::{Deserialize, Serialize};

use crate::errors::{PPMError, PPMResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	pub default_focus_duration_in_minutes: u32,
	pub session_storage_path: String,
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
		Self {
			default_focus_duration_in_minutes: 60,
			session_storage_path: String::from("~/.config/ppm/sessions"),
		}
	}
}
