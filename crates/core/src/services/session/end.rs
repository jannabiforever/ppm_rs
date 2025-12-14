use std::sync::Arc;

use crate::clock::Clock;
use crate::errors::{PPMError, PPMResult};
use crate::models::FocusSessionId;
use crate::output::OutputWriter;
use crate::repositories::SessionRepository;
use crate::services::Service;

/// Service to end the currently active focus session.
///
/// Updates the session's end time to now and outputs confirmation.
pub struct EndFocusSession {
	pub clock: Arc<dyn Clock>,
	pub repository: Arc<dyn SessionRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
}

impl EndFocusSession {
	fn get_active_session_id(&self) -> PPMResult<FocusSessionId> {
		let now = self.clock.now()?;
		let session = self.repository.get_active_session(now)?.ok_or(PPMError::NoActiveSession)?;

		Ok(session.id)
	}

	fn end_session(&self, session_id: FocusSessionId) -> PPMResult<()> {
		let now = self.clock.now()?;
		self.repository.end_session(&session_id, now)?;

		self.output_writer.write_line(&"Focus session ended")?;
		self.output_writer.write_line(&format!("Session ID: {}", session_id))?;

		Ok(())
	}
}

impl Service for EndFocusSession {
	fn run(&self) -> PPMResult<()> {
		let session_id = self.get_active_session_id()?;
		self.end_session(session_id)?;
		Ok(())
	}
}
