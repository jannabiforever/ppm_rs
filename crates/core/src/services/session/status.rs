use std::sync::Arc;

use chrono::{DateTime, Utc};

use crate::errors::PPMResult;
use crate::output::OutputWriter;
use crate::repositories::SessionRepository;
use crate::services::Service;

pub struct GetSessionStatus {
	pub clock: Arc<dyn crate::clock::Clock>,
	pub repository: Arc<dyn SessionRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
}

impl Service for GetSessionStatus {
	type Output = ();

	fn run(self) -> PPMResult<()> {
		let now = self.clock.now();
		let active_session = self.repository.get_active_session(now)?;

		match active_session {
			Some(session) => {
				let remaining = session.end - now;
				let remaining_minutes = remaining.num_minutes();

				self.output_writer.write_line(&format!(
					"Focus session active ({} minutes remaining)",
					remaining_minutes
				))?;
				self.output_writer
					.write_line(&format!("Started: {}", format_datetime(&session.start)))?;
				self.output_writer
					.write_line(&format!("Ends: {}", format_datetime(&session.end)))?;
			}
			None => {
				self.output_writer.write_line(&"No active focus session")?;
			}
		}

		Ok(())
	}
}

fn format_datetime(dt: &DateTime<Utc>) -> String {
	dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}
