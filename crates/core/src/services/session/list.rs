use std::sync::Arc;

use chrono::{DateTime, Utc};

use crate::clock::Clock;
use crate::errors::PPMResult;
use crate::output::OutputWriter;
use crate::repositories::SessionRepository;
use crate::services::Service;

pub struct ListSessions {
	pub clock: Arc<dyn Clock>,
	pub repository: Arc<dyn SessionRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
	pub limit: Option<usize>,
}

impl Service for ListSessions {
	fn run(&self) -> PPMResult<()> {
		let now = self.clock.now()?;
		let mut sessions = self.repository.list_sessions()?;

		if sessions.is_empty() {
			self.output_writer.write_line(&"No focus sessions found")?;
			return Ok(());
		}

		// Sort by start time (most recent first)
		sessions.sort_by(|a, b| b.start.cmp(&a.start));

		// Apply limit if specified
		if let Some(limit) = self.limit {
			sessions.truncate(limit);
		}

		self.output_writer.write_line(&format!("Focus sessions ({})", sessions.len()))?;
		self.output_writer.write_line(&"")?;

		for session in sessions {
			let status = if session.is_active(now) {
				"Active"
			} else {
				"Completed"
			};

			let duration_minutes = session.duration().num_minutes();

			self.output_writer.write_line(&format!(
				"[{}] {} - {} minutes",
				status,
				format_datetime(&session.start),
				duration_minutes
			))?;
			self.output_writer.write_line(&format!("  ID: {}", session.id))?;
		}

		Ok(())
	}
}

fn format_datetime(dt: &DateTime<Utc>) -> String {
	dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}
