use std::sync::Arc;

use chrono::{DateTime, Datelike, Duration, Utc};

use crate::clock::Clock;
use crate::errors::PPMResult;
use crate::output::OutputWriter;
use crate::repositories::SessionRepository;
use crate::services::Service;

pub struct GetSessionStats {
	pub clock: Arc<dyn Clock>,
	pub repository: Arc<dyn SessionRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
}

impl Service for GetSessionStats {
	fn run(&self) -> PPMResult<()> {
		let now = self.clock.now()?;
		let sessions = self.repository.list_sessions()?;

		if sessions.is_empty() {
			self.output_writer.write_line(&"No focus sessions found")?;
			return Ok(());
		}

		// Filter sessions by time periods
		let today_sessions: Vec<_> =
			sessions.iter().filter(|s| is_same_day(&s.start, &now)).collect();

		let week_sessions: Vec<_> =
			sessions.iter().filter(|s| is_same_week(&s.start, &now)).collect();

		// Calculate total durations
		let today_duration: Duration = today_sessions.iter().map(|s| s.duration()).sum();
		let week_duration: Duration = week_sessions.iter().map(|s| s.duration()).sum();
		let total_duration: Duration = sessions.iter().map(|s| s.duration()).sum();

		// Calculate average
		let avg_duration = if !sessions.is_empty() {
			total_duration / sessions.len() as i32
		} else {
			Duration::zero()
		};

		// Display stats
		self.output_writer.write_line(&"Focus Session Statistics")?;
		self.output_writer.write_line(&"")?;

		self.output_writer.write_line(&format!(
			"Today: {} ({} sessions)",
			format_duration(&today_duration),
			today_sessions.len()
		))?;

		self.output_writer.write_line(&format!(
			"This week: {} ({} sessions)",
			format_duration(&week_duration),
			week_sessions.len()
		))?;

		self.output_writer.write_line(&format!(
			"All time: {} ({} sessions)",
			format_duration(&total_duration),
			sessions.len()
		))?;

		self.output_writer.write_line(&"")?;

		self.output_writer
			.write_line(&format!("Average session: {}", format_duration(&avg_duration)))?;

		Ok(())
	}
}

fn is_same_day(dt1: &DateTime<Utc>, dt2: &DateTime<Utc>) -> bool {
	dt1.year() == dt2.year() && dt1.ordinal() == dt2.ordinal()
}

fn is_same_week(dt1: &DateTime<Utc>, dt2: &DateTime<Utc>) -> bool {
	let diff = (*dt2 - *dt1).num_days();
	(0..7).contains(&diff) && dt1.year() == dt2.year()
}

fn format_duration(duration: &Duration) -> String {
	let total_minutes = duration.num_minutes();
	let hours = total_minutes / 60;
	let minutes = total_minutes % 60;

	if hours > 0 {
		format!("{}h {}m", hours, minutes)
	} else {
		format!("{}m", minutes)
	}
}
