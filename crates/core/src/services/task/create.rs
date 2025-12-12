use std::sync::Arc;

use crate::clock::Clock;
use crate::errors::PPMResult;
use crate::models::{ProjectName, Task, TaskId, TaskStatus};
use crate::output::OutputWriter;
use crate::repositories::task::TaskRepository;
use crate::services::Service;

pub struct CreateTask {
	pub clock: Arc<dyn Clock>,
	pub task_repository: Arc<dyn TaskRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
	pub project_name: ProjectName,
	pub description: String,
}

impl Service for CreateTask {
	type Output = ();

	fn run(self) -> PPMResult<()> {
		let task = Task {
			id: TaskId::new(),
			project_name: self.project_name.clone(),
			description: self.description.clone(),
			status: TaskStatus::Pending,
			created_at: self.clock.now(),
		};

		self.task_repository.create_task(task)?;
		self.output_writer
			.write_line(&format!("Task created for project '{}'", self.project_name))?;

		Ok(())
	}
}
