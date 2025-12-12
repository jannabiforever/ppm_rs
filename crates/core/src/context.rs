use std::path::PathBuf;
use std::sync::Arc;

use crate::clock::{Clock, SystemClock};
use crate::config::Config;
use crate::output::{OutputWriter, stdout_writer};
use crate::repositories::note::{LocalNoteRepository, NoteRepository};
use crate::repositories::session::{LocalSessionRepository, SessionRepository};
use crate::repositories::task::{LocalTaskRepository, TaskRepository};

/// Dependency injection container.
///
/// Assembled in main.rs and passed to CommandHandlers.
/// Clone is cheap (Arc clones only increment reference counts).
#[derive(Clone)]
pub struct PPMContext {
	pub config: Config,
	pub clock: Arc<dyn Clock>,
	pub session_repository: Arc<dyn SessionRepository>,
	pub task_repository: Arc<dyn TaskRepository>,
	pub note_repository: Arc<dyn NoteRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
}

impl PPMContext {
	pub fn new(config: Config) -> Self {
		let clock = Arc::new(SystemClock::new());
		let session_storage_path = PathBuf::from(&config.session_storage_path);
		let task_storage_path = PathBuf::from(&config.task_storage_path);
		let note_storage_path = PathBuf::from(&config.note_storage_path);

		Self {
			config,
			clock: clock.clone(),
			session_repository: Arc::new(LocalSessionRepository::new(session_storage_path)),
			task_repository: Arc::new(LocalTaskRepository::new(task_storage_path)),
			note_repository: Arc::new(LocalNoteRepository::new(note_storage_path)),
			output_writer: Arc::new(stdout_writer()),
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
	clock: Option<Arc<dyn Clock>>,
	session_repository: Option<Arc<dyn SessionRepository>>,
	task_repository: Option<Arc<dyn TaskRepository>>,
	note_repository: Option<Arc<dyn NoteRepository>>,
	output_writer: Option<Arc<dyn OutputWriter>>,
}

impl PPMContextBuilder {
	pub fn config(mut self, config: Config) -> Self {
		self.config = Some(config);
		self
	}

	pub fn clock(mut self, clock: Arc<dyn Clock>) -> Self {
		self.clock = Some(clock);
		self
	}

	pub fn session_repository(mut self, repository: Arc<dyn SessionRepository>) -> Self {
		self.session_repository = Some(repository);
		self
	}

	pub fn task_repository(mut self, repository: Arc<dyn TaskRepository>) -> Self {
		self.task_repository = Some(repository);
		self
	}

	pub fn note_repository(mut self, repository: Arc<dyn NoteRepository>) -> Self {
		self.note_repository = Some(repository);
		self
	}

	pub fn output_writer(mut self, writer: Arc<dyn OutputWriter>) -> Self {
		self.output_writer = Some(writer);
		self
	}

	pub fn build(self) -> PPMContext {
		let config = self.config.unwrap_or_default();
		let clock = self.clock.unwrap_or_else(|| Arc::new(SystemClock::new()));
		let session_storage_path = PathBuf::from(&config.session_storage_path);
		let task_storage_path = PathBuf::from(&config.task_storage_path);
		let note_storage_path = PathBuf::from(&config.note_storage_path);

		PPMContext {
			config,
			clock: clock.clone(),
			session_repository: self
				.session_repository
				.unwrap_or_else(|| Arc::new(LocalSessionRepository::new(session_storage_path))),
			task_repository: self
				.task_repository
				.unwrap_or_else(|| Arc::new(LocalTaskRepository::new(task_storage_path))),
			note_repository: self
				.note_repository
				.unwrap_or_else(|| Arc::new(LocalNoteRepository::new(note_storage_path))),
			output_writer: self.output_writer.unwrap_or_else(|| Arc::new(stdout_writer())),
		}
	}
}
