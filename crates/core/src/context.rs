use std::sync::Arc;

use crate::config::Config;
use crate::output::{OutputWriter, StdoutWriter};
use crate::repositories::{LocalSessionRepository, SessionRepository};

pub struct PPMContext {
	pub config: Config,
	pub session_repository: Arc<dyn SessionRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
}

impl PPMContext {
	pub fn new(config: Config) -> Self {
		Self {
			config,
			session_repository: Arc::new(LocalSessionRepository::new()),
			output_writer: Arc::new(StdoutWriter::new()),
		}
	}

	pub fn builder() -> PPMContextBuilder {
		PPMContextBuilder::default()
	}
}

// --------------------------------------------------------------------------------
// Builder Pattern for flexible DI
// --------------------------------------------------------------------------------

#[derive(Default)]
pub struct PPMContextBuilder {
	config: Option<Config>,
	session_repository: Option<Arc<dyn SessionRepository>>,
	output_writer: Option<Arc<dyn OutputWriter>>,
}

impl PPMContextBuilder {
	pub fn config(mut self, config: Config) -> Self {
		self.config = Some(config);
		self
	}

	pub fn session_repository(mut self, repository: Arc<dyn SessionRepository>) -> Self {
		self.session_repository = Some(repository);
		self
	}

	pub fn output_writer(mut self, writer: Arc<dyn OutputWriter>) -> Self {
		self.output_writer = Some(writer);
		self
	}

	pub fn build(self) -> PPMContext {
		PPMContext {
			config: self.config.unwrap_or_default(),
			session_repository: self
				.session_repository
				.unwrap_or_else(|| Arc::new(LocalSessionRepository::new())),
			output_writer: self.output_writer.unwrap_or_else(|| Arc::new(StdoutWriter::new())),
		}
	}
}
