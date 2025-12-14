use std::sync::PoisonError;

#[derive(Debug, thiserror::Error)]
pub enum PPMError {
	#[error("A focus session is already active")]
	SessionAlreadyActive,

	#[error("No active focus session found")]
	NoActiveSession,

	#[error("Failed to create focus session: {0}")]
	SessionCreationFailed(String),

	#[error("Failed to end focus session: {0}")]
	SessionEndFailed(String),

	#[error("{0}")]
	NotFound(String),

	#[error("{0}")]
	AlreadyExists(String),

	#[error("Configuration error: {0}")]
	ConfigError(String),

	#[error("{0}")]
	IoError(#[from] std::io::Error),

	#[error("Failed to acquire output writer lock")]
	LockError,

	#[error("Editor error: {0}")]
	EditorError(String),

	#[error("{0}")]
	SerdeJson(#[from] serde_json::Error),
}

impl<T> From<PoisonError<T>> for PPMError {
	fn from(_: PoisonError<T>) -> Self {
		PPMError::LockError
	}
}

impl PartialEq for PPMError {
	fn eq(&self, other: &Self) -> bool {
		// errors should be identified by their display strings.
		self.to_string() == other.to_string()
	}
}

pub type PPMResult<T> = Result<T, PPMError>;
