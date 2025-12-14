use std::fs::{self, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use crate::errors::{PPMError, PPMResult};
use crate::models::{ProjectName, Task, TaskId, TaskStatus};

/// Data access abstraction for tasks.
pub trait TaskRepository: Send + Sync {
	fn create_task(&self, task: Task) -> PPMResult<()>;
	fn get_task(&self, task_id: &TaskId) -> PPMResult<Option<Task>>;
	fn update_task_status(&self, task_id: &TaskId, status: TaskStatus) -> PPMResult<()>;
	fn list_tasks(&self) -> PPMResult<Vec<Task>>;
	fn list_tasks_by_project(&self, project_name: &ProjectName) -> PPMResult<Vec<Task>>;
	fn delete_task(&self, task_id: &TaskId) -> PPMResult<()>;
}

// --------------------------------------------------------------------------------
// Concrete Implementations
// --------------------------------------------------------------------------------

/// File-based task repository storing data in JSON format.
pub struct LocalTaskRepository {
	storage_path: PathBuf,
}

impl LocalTaskRepository {
	pub fn new(storage_path: PathBuf) -> Self {
		Self {
			storage_path,
		}
	}

	fn ensure_storage_dir(&self) -> PPMResult<()> {
		if let Some(parent) = self.storage_path.parent() {
			fs::create_dir_all(parent)?;
		}
		Ok(())
	}

	fn load_tasks(&self) -> PPMResult<Vec<Task>> {
		if !self.storage_path.exists() {
			return Ok(Vec::new());
		}

		let file = fs::File::open(&self.storage_path)?;
		let reader = BufReader::new(file);
		let tasks: Vec<Task> = serde_json::from_reader(reader)
			.map_err(|e| std::io::Error::other(format!("Failed to parse tasks: {}", e)))?;

		Ok(tasks)
	}

	fn save_tasks(&self, tasks: &[Task]) -> PPMResult<()> {
		self.ensure_storage_dir()?;

		let file =
			OpenOptions::new().write(true).create(true).truncate(true).open(&self.storage_path)?;

		let mut writer = BufWriter::new(file);
		serde_json::to_writer_pretty(&mut writer, tasks)
			.map_err(|e| std::io::Error::other(format!("Failed to write tasks: {}", e)))?;

		writer.flush()?;

		Ok(())
	}
}

impl TaskRepository for LocalTaskRepository {
	fn create_task(&self, task: Task) -> PPMResult<()> {
		let mut tasks = self.load_tasks()?;
		tasks.push(task);
		self.save_tasks(&tasks)?;
		Ok(())
	}

	fn get_task(&self, task_id: &TaskId) -> PPMResult<Option<Task>> {
		let tasks = self.load_tasks()?;
		Ok(tasks.into_iter().find(|t| &t.id == task_id))
	}

	fn update_task_status(&self, task_id: &TaskId, status: TaskStatus) -> PPMResult<()> {
		let mut tasks = self.load_tasks()?;

		if let Some(task) = tasks.iter_mut().find(|t| &t.id == task_id) {
			task.status = status;
			self.save_tasks(&tasks)?;
			Ok(())
		} else {
			Err(PPMError::NotFound(format!("Task {} not found", task_id)))
		}
	}

	fn list_tasks(&self) -> PPMResult<Vec<Task>> {
		self.load_tasks()
	}

	fn list_tasks_by_project(&self, project_name: &ProjectName) -> PPMResult<Vec<Task>> {
		let tasks = self.load_tasks()?;
		Ok(tasks.into_iter().filter(|t| t.project_name.as_ref() == Some(project_name)).collect())
	}

	fn delete_task(&self, task_id: &TaskId) -> PPMResult<()> {
		let mut tasks = self.load_tasks()?;
		let initial_len = tasks.len();

		tasks.retain(|t| &t.id != task_id);

		if tasks.len() == initial_len {
			return Err(PPMError::NotFound(format!("Task {} not found", task_id)));
		}

		self.save_tasks(&tasks)?;
		Ok(())
	}
}
