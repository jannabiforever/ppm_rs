use std::sync::Arc;

use chrono::Duration;

use crate::clock::Clock;
use crate::errors::{PPMError, PPMResult};
use crate::models::FocusSession;
use crate::output::OutputWriter;
use crate::repositories::SessionRepository;
use crate::services::Service;

/// Service to start a new focus session.
///
/// Validates no active session exists, creates a new session,
/// and outputs confirmation to the user.
pub struct StartFocusSession {
	pub clock: Arc<dyn Clock>,
	pub repository: Arc<dyn SessionRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
	pub duration_in_minutes: u32,
}

impl StartFocusSession {
	fn ensure_no_active_focus_session(&self) -> PPMResult<()> {
		if self.repository.get_active_session(self.clock.now())?.is_some() {
			return Err(PPMError::SessionAlreadyActive);
		}
		Ok(())
	}

	fn create_new_focus_session(&self) -> PPMResult<()> {
		let duration_seconds = self.duration_in_minutes as i64 * 60;
		let now = self.clock.now();
		let session = FocusSession {
			id: FocusSession::generate_id(),
			start: now,
			end: now + Duration::seconds(duration_seconds),
		};

		self.repository.create_session(session)?;

		self.output_writer.write_line(&"Focus session started")?;
		self.output_writer
			.write_line(&format!("Duration: {} minutes", self.duration_in_minutes))?;

		Ok(())
	}
}

impl Service for StartFocusSession {
	type Output = ();

	fn run(self) -> PPMResult<Self::Output> {
		self.ensure_no_active_focus_session()?;
		self.create_new_focus_session()?;
		Ok(())
	}
}
