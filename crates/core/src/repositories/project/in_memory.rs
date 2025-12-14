use std::sync::{Arc, Mutex};

use crate::errors::{PPMError, PPMResult};
use crate::models::{Project, ProjectName, ProjectStatus};
use crate::repositories::project::ProjectRepository;

/// In-memory project repository for testing
pub struct InMemoryProjectRepository {
	projects: Arc<Mutex<Vec<Project>>>,
}

impl InMemoryProjectRepository {
	pub fn new() -> Self {
		Self {
			projects: Arc::new(Mutex::new(Vec::new())),
		}
	}

	pub fn get_all_projects(&self) -> PPMResult<Vec<Project>> {
		Ok(self.projects.lock()?.clone())
	}
}

impl Default for InMemoryProjectRepository {
	fn default() -> Self {
		Self::new()
	}
}

impl ProjectRepository for InMemoryProjectRepository {
	fn create_project(&self, project: Project) -> PPMResult<()> {
		let mut projects = self.projects.lock()?;

		// Check if project with same name already exists
		if projects.iter().any(|p| p.name == project.name) {
			return Err(PPMError::AlreadyExists(format!(
				"Project '{}' already exists",
				project.name
			)));
		}

		projects.push(project);
		Ok(())
	}

	fn get_project(&self, project_name: &ProjectName) -> PPMResult<Option<Project>> {
		let projects = self.projects.lock()?;
		Ok(projects.iter().find(|p| &p.name == project_name).cloned())
	}

	fn update_project_status(
		&self,
		project_name: &ProjectName,
		status: ProjectStatus,
	) -> PPMResult<()> {
		let mut projects = self.projects.lock()?;

		if let Some(project) = projects.iter_mut().find(|p| &p.name == project_name) {
			project.status = status;
			Ok(())
		} else {
			Err(PPMError::NotFound(format!("Project '{}' not found", project_name)))
		}
	}

	fn list_projects(&self) -> PPMResult<Vec<Project>> {
		self.get_all_projects()
	}

	fn delete_project(&self, project_name: &ProjectName) -> PPMResult<()> {
		let mut projects = self.projects.lock()?;
		let initial_len = projects.len();

		projects.retain(|p| &p.name != project_name);

		if projects.len() == initial_len {
			return Err(PPMError::NotFound(format!("Project '{}' not found", project_name)));
		}

		Ok(())
	}
}
