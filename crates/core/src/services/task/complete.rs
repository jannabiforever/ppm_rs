use std::sync::Arc;

use crate::clock::Clock;
use crate::errors::{PPMError, PPMResult};
use crate::models::{TaskId, TaskStatus};
use crate::output::OutputWriter;
use crate::repositories::task::TaskRepository;
use crate::services::Service;

pub struct CompleteTask {
	pub clock: Arc<dyn Clock>,
	pub task_repository: Arc<dyn TaskRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
	pub task_id: TaskId,
}

impl Service for CompleteTask {
	type Output = ();

	fn run(self) -> PPMResult<()> {
		// Validate task exists and is pending
		let task = self
			.task_repository
			.get_task(&self.task_id)?
			.ok_or_else(|| PPMError::NotFound(format!("Task {} not found", self.task_id)))?;

		match task.status {
			TaskStatus::Pending => {
				// Update status to Done with current timestamp
				let completed_at = self.clock.now();
				self.task_repository
					.update_task_status(&self.task_id, TaskStatus::Done(completed_at))?;

				self.output_writer.write_line(&format!("Task {} completed", self.task_id))?;
				Ok(())
			}
			TaskStatus::Done(_) => {
				Err(PPMError::NotFound(format!("Task {} is already completed", self.task_id)))
			}
			TaskStatus::Canceled(_) => {
				Err(PPMError::NotFound(format!("Task {} is canceled", self.task_id)))
			}
		}
	}
}
