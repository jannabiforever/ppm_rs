use std::sync::Arc;

use chrono::{Duration, Utc};

use crate::errors::{PPMError, PPMResult};
use crate::models::FocusSession;
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
	repository: Arc<dyn SessionRepository>,
	output_writer: Arc<dyn OutputWriter>,
	duration_in_minutes: u32,
}

impl LocallyStartFocusSession {
	pub fn new(
		repository: Arc<dyn SessionRepository>,
		output_writer: Arc<dyn OutputWriter>,
		duration_in_minutes: u32,
	) -> Self {
		Self {
			repository,
			output_writer,
			duration_in_minutes,
		}
	}
}

impl StartFocusSessionService for LocallyStartFocusSession {
	fn ensure_no_active_focus_session(&self) -> PPMResult<()> {
		if self.repository.get_active_session()?.is_some() {
			return Err(PPMError::SessionAlreadyActive);
		}
		Ok(())
	}

	fn create_new_focus_session(&self) -> PPMResult<()> {
		let duration_seconds = self.duration_in_minutes as i64 * 60;
		let session = FocusSession {
			start: Utc::now(),
			end: Utc::now() + Duration::seconds(duration_seconds),
		};

		self.repository.create_session(session)?;

		self.output_writer.write_line(&"[ppm] Focus session started")?;
		self.output_writer
			.write_line(&format!("Duration: {} minutes", self.duration_in_minutes))?;

		Ok(())
	}
}
