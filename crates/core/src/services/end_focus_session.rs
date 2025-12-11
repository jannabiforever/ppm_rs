use std::sync::Arc;

use crate::clock::Clock;
use crate::errors::{PPMError, PPMResult};
use crate::output::OutputWriter;
use crate::repositories::SessionRepository;
use crate::services::Service;

pub struct EndFocusSession {
	pub clock: Arc<dyn Clock>,
	pub repository: Arc<dyn SessionRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
}

impl EndFocusSession {
	fn get_active_session_id(&self) -> PPMResult<String> {
		let now = self.clock.now();
		let session = self.repository.get_active_session(now)?.ok_or(PPMError::NoActiveSession)?;

		Ok(session.id)
	}

	fn end_session(&self, session_id: String) -> PPMResult<()> {
		let now = self.clock.now();
		self.repository.end_session(&session_id, now)?;

		self.output_writer.write_line(&"[ppm] Focus session ended")?;
		self.output_writer.write_line(&format!("Session ID: {}", session_id))?;

		Ok(())
	}
}

impl Service for EndFocusSession {
	type Output = ();

	fn run(self) -> PPMResult<Self::Output> {
		let session_id = self.get_active_session_id()?;
		self.end_session(session_id)?;
		Ok(())
	}
}
