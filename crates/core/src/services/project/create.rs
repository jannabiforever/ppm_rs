use std::sync::Arc;

use crate::clock::Clock;
use crate::errors::PPMResult;
use crate::models::{Project, ProjectName, ProjectStatus};
use crate::output::OutputWriter;
use crate::repositories::project::ProjectRepository;
use crate::services::Service;

pub struct CreateProject {
	pub clock: Arc<dyn Clock>,
	pub project_repository: Arc<dyn ProjectRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
	pub name: ProjectName,
	pub description: String,
}

impl Service for CreateProject {
	fn run(&self) -> PPMResult<()> {
		let project = Project {
			name: self.name.clone(),
			description: self.description.clone(),
			created_at: self.clock.now()?,
			status: ProjectStatus::Active,
		};

		self.project_repository.create_project(project)?;
		self.output_writer.write_line(&format!("Project '{}' created", self.name))?;

		Ok(())
	}
}
