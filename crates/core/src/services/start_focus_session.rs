use std::sync::Arc;

use chrono::Duration;

use crate::clock::Clock;
use crate::errors::{PPMError, PPMResult};
use crate::models::{FocusSession, generate_session_id};
use crate::output::OutputWriter;
use crate::repositories::SessionRepository;
use crate::services::Service;

pub trait StartFocusSessionService {
	fn ensure_no_active_focus_session(&self) -> PPMResult<()>;

	fn create_new_focus_session(&self) -> PPMResult<()>;
}

impl<S: StartFocusSessionService> Service for S {
	type Output = ();

	fn run(self) -> PPMResult<Self::Output> {
		self.ensure_no_active_focus_session()?;
		self.create_new_focus_session()?;
		Ok(())
	}
}

// --------------------------------------------------------------------------------
// Concrete Implementations
// --------------------------------------------------------------------------------

pub struct LocallyStartFocusSession {
	clock: Arc<dyn Clock>,
	repository: Arc<dyn SessionRepository>,
	output_writer: Arc<dyn OutputWriter>,
	duration_in_minutes: u32,
}

impl LocallyStartFocusSession {
	pub fn new(
		clock: Arc<dyn Clock>,
		repository: Arc<dyn SessionRepository>,
		output_writer: Arc<dyn OutputWriter>,
		duration_in_minutes: u32,
	) -> Self {
		Self {
			clock,
			repository,
			output_writer,
			duration_in_minutes,
		}
	}
}

impl StartFocusSessionService for LocallyStartFocusSession {
	fn ensure_no_active_focus_session(&self) -> PPMResult<()> {
		if self.repository.get_active_session(self.clock.now())?.is_some() {
			return Err(PPMError::SessionAlreadyActive);
		}
		Ok(())
	}

	fn create_new_focus_session(&self) -> PPMResult<()> {
		let duration_seconds = self.duration_in_minutes as i64 * 60;
		let now = self.clock.now();
		let session = FocusSession::new(
			generate_session_id(),
			now,
			now + Duration::seconds(duration_seconds),
		);

		self.repository.create_session(session)?;

		self.output_writer.write_line(&"[ppm] Focus session started")?;
		self.output_writer
			.write_line(&format!("Duration: {} minutes", self.duration_in_minutes))?;

		Ok(())
	}
}
