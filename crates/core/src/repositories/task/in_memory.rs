use std::sync::{Arc, Mutex};

use crate::errors::{PPMError, PPMResult};
use crate::models::{ProjectName, Task, TaskId, TaskStatus};
use crate::repositories::task::TaskRepository;

/// In-memory task repository for testing
pub struct InMemoryTaskRepository {
	tasks: Arc<Mutex<Vec<Task>>>,
}

impl InMemoryTaskRepository {
	pub fn new() -> Self {
		Self {
			tasks: Arc::new(Mutex::new(Vec::new())),
		}
	}

	pub fn get_all_tasks(&self) -> PPMResult<Vec<Task>> {
		Ok(self.tasks.lock()?.clone())
	}
}

impl Default for InMemoryTaskRepository {
	fn default() -> Self {
		Self::new()
	}
}

impl TaskRepository for InMemoryTaskRepository {
	fn create_task(&self, task: Task) -> PPMResult<()> {
		let mut tasks = self.tasks.lock()?;
		tasks.push(task);
		Ok(())
	}

	fn get_task(&self, task_id: &TaskId) -> PPMResult<Option<Task>> {
		let tasks = self.tasks.lock()?;
		Ok(tasks.iter().find(|t| &t.id == task_id).cloned())
	}

	fn update_task_status(&self, task_id: &TaskId, status: TaskStatus) -> PPMResult<()> {
		let mut tasks = self.tasks.lock()?;

		if let Some(task) = tasks.iter_mut().find(|t| &t.id == task_id) {
			task.status = status;
			Ok(())
		} else {
			Err(PPMError::NotFound(format!("Task {} not found", task_id)))
		}
	}

	fn list_tasks(&self) -> PPMResult<Vec<Task>> {
		self.get_all_tasks()
	}

	fn list_tasks_by_project(&self, project_name: &ProjectName) -> PPMResult<Vec<Task>> {
		let tasks = self.tasks.lock()?;
		Ok(tasks
			.iter()
			.filter(|&t| t.project_name.as_ref() == Some(project_name))
			.cloned()
			.collect())
	}

	fn delete_task(&self, task_id: &TaskId) -> PPMResult<()> {
		let mut tasks = self.tasks.lock()?;
		let initial_len = tasks.len();

		tasks.retain(|t| &t.id != task_id);

		if tasks.len() == initial_len {
			return Err(PPMError::NotFound(format!("Task {} not found", task_id)));
		}

		Ok(())
	}
}
