#[derive(Debug, thiserror::Error)]

pub enum PPMCliError {
	#[error("{0}")]
	Io(#[from] std::io::Error),

	#[error("{0}")]
	CommandParse(#[from] clap::Error),

	#[error("{0}")]
	Core(#[from] ppm_core::errors::PPMError),
}
