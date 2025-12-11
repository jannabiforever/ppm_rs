#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum PPMError {
	#[error("A focus session is already active")]
	SessionAlreadyActive,

	#[error("No active focus session found")]
	NoActiveSession,

	#[error("Failed to create focus session: {0}")]
	SessionCreationFailed(String),

	#[error("Failed to end focus session: {0}")]
	SessionEndFailed(String),

	#[error("Configuration error: {0}")]
	ConfigError(String),

	#[error("IO error: {0}")]
	IoError(String),
}

pub type PPMResult<T> = Result<T, PPMError>;
