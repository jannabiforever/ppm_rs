use std::fs::{self, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use crate::errors::{PPMError, PPMResult};
use crate::models::{Project, ProjectName, ProjectStatus};

/// Data access abstraction for projects.
pub trait ProjectRepository: Send + Sync {
	fn create_project(&self, project: Project) -> PPMResult<()>;
	fn get_project(&self, project_name: &ProjectName) -> PPMResult<Option<Project>>;
	fn update_project_status(
		&self,
		project_name: &ProjectName,
		status: ProjectStatus,
	) -> PPMResult<()>;
	fn list_projects(&self) -> PPMResult<Vec<Project>>;
	fn delete_project(&self, project_name: &ProjectName) -> PPMResult<()>;
}

// --------------------------------------------------------------------------------
// Concrete Implementations
// --------------------------------------------------------------------------------

/// File-based project repository storing data in JSON format.
pub struct LocalProjectRepository {
	storage_path: PathBuf,
}

impl LocalProjectRepository {
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

	fn load_projects(&self) -> PPMResult<Vec<Project>> {
		if !self.storage_path.exists() {
			return Ok(Vec::new());
		}

		let file = fs::File::open(&self.storage_path)?;
		let reader = BufReader::new(file);
		let projects: Vec<Project> = serde_json::from_reader(reader)
			.map_err(|e| std::io::Error::other(format!("Failed to parse projects: {}", e)))?;

		Ok(projects)
	}

	fn save_projects(&self, projects: &[Project]) -> PPMResult<()> {
		self.ensure_storage_dir()?;

		let file =
			OpenOptions::new().write(true).create(true).truncate(true).open(&self.storage_path)?;

		let mut writer = BufWriter::new(file);
		serde_json::to_writer_pretty(&mut writer, projects)
			.map_err(|e| std::io::Error::other(format!("Failed to write projects: {}", e)))?;

		writer.flush()?;

		Ok(())
	}
}

impl ProjectRepository for LocalProjectRepository {
	fn create_project(&self, project: Project) -> PPMResult<()> {
		let mut projects = self.load_projects()?;

		// Check if project with same name already exists
		if projects.iter().any(|p| p.name == project.name) {
			return Err(PPMError::AlreadyExists(format!(
				"Project '{}' already exists",
				project.name
			)));
		}

		projects.push(project);
		self.save_projects(&projects)?;
		Ok(())
	}

	fn get_project(&self, project_name: &ProjectName) -> PPMResult<Option<Project>> {
		let projects = self.load_projects()?;
		Ok(projects.into_iter().find(|p| &p.name == project_name))
	}

	fn update_project_status(
		&self,
		project_name: &ProjectName,
		status: ProjectStatus,
	) -> PPMResult<()> {
		let mut projects = self.load_projects()?;

		if let Some(project) = projects.iter_mut().find(|p| &p.name == project_name) {
			project.status = status;
			self.save_projects(&projects)?;
			Ok(())
		} else {
			Err(PPMError::NotFound(format!("Project '{}' not found", project_name)))
		}
	}

	fn list_projects(&self) -> PPMResult<Vec<Project>> {
		self.load_projects()
	}

	fn delete_project(&self, project_name: &ProjectName) -> PPMResult<()> {
		let mut projects = self.load_projects()?;
		let initial_len = projects.len();

		projects.retain(|p| &p.name != project_name);

		if projects.len() == initial_len {
			return Err(PPMError::NotFound(format!("Project '{}' not found", project_name)));
		}

		self.save_projects(&projects)?;
		Ok(())
	}
}
