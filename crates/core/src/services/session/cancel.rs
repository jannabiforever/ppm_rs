use std::sync::Arc;

use crate::clock::Clock;
use crate::errors::PPMResult;
use crate::output::OutputWriter;
use crate::repositories::SessionRepository;
use crate::services::Service;

pub struct CancelFocusSession {
	pub clock: Arc<dyn Clock>,
	pub repository: Arc<dyn SessionRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
}

impl Service for CancelFocusSession {
	type Output = ();

	fn run(self) -> PPMResult<()> {
		let now = self.clock.now()?;
		let active_session = self.repository.get_active_session(now)?;

		match active_session {
			Some(session) => {
				self.repository.delete_session(&session.id)?;
				self.output_writer.write_line(&"Focus session cancelled")?;
				self.output_writer.write_line(&format!("Session ID: {}", session.id))?;
				Ok(())
			}
			None => Err(crate::errors::PPMError::NoActiveSession),
		}
	}
}
