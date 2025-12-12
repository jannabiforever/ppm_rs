#[derive(Debug, thiserror::Error)]
pub enum PPMCliError {
	#[error("{0}")]
	Io(#[from] std::io::Error),

	#[error("{0}")]
	CoreError(#[from] ppm_core::errors::PPMError),
}
