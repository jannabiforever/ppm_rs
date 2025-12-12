use std::sync::Arc;

use crate::errors::PPMResult;
use crate::models::{Task, TaskStatus};
use crate::output::OutputWriter;
use crate::repositories::task::TaskRepository;
use crate::services::Service;

pub struct ListTasks {
	pub task_repository: Arc<dyn TaskRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
	pub filter: Option<TaskFilter>,
}

#[derive(Debug, Clone)]
pub enum TaskFilter {
	All,
	Pending,
	Done,
	Canceled,
}

impl Service for ListTasks {
	type Output = ();

	fn run(self) -> PPMResult<()> {
		let mut tasks = self.task_repository.list_tasks()?;

		// Sort by created_at descending (newest first)
		tasks.sort_by(|a, b| b.created_at.cmp(&a.created_at));

		// Apply filter
		let filtered_tasks: Vec<Task> = match self.filter {
			Some(TaskFilter::Pending) => {
				tasks.into_iter().filter(|t| matches!(t.status, TaskStatus::Pending)).collect()
			}
			Some(TaskFilter::Done) => {
				tasks.into_iter().filter(|t| matches!(t.status, TaskStatus::Done(_))).collect()
			}
			Some(TaskFilter::Canceled) => {
				tasks.into_iter().filter(|t| matches!(t.status, TaskStatus::Canceled(_))).collect()
			}
			Some(TaskFilter::All) | None => tasks,
		};

		if filtered_tasks.is_empty() {
			self.output_writer.write_line(&"No tasks found")?;
			return Ok(());
		}

		self.output_writer.write_line(&format!("{} task(s) found:", filtered_tasks.len()))?;

		for task in filtered_tasks {
			let status_display = match &task.status {
				TaskStatus::Pending => "[ ]",
				TaskStatus::Done(_) => "[✓]",
				TaskStatus::Canceled(_) => "[✗]",
			};

			self.output_writer.write_line(&format!(
				"  {} {} - {} ({})",
				status_display, task.id, task.description, task.project_name
			))?;
		}

		Ok(())
	}
}
