use std::sync::Arc;

use crate::errors::PPMResult;
use crate::models::{Project, ProjectStatus};
use crate::output::OutputWriter;
use crate::repositories::project::ProjectRepository;
use crate::services::Service;

pub struct ListProjects {
	pub project_repository: Arc<dyn ProjectRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
	pub filter: Option<ProjectFilter>,
}

#[derive(Debug, Clone)]
pub enum ProjectFilter {
	All,
	Active,
	Inactive,
}

impl Service for ListProjects {
	fn run(&self) -> PPMResult<()> {
		let mut projects = self.project_repository.list_projects()?;

		// Sort by created_at descending (newest first)
		projects.sort_by(|a, b| b.created_at.cmp(&a.created_at));

		// Apply filter
		let filtered_projects: Vec<Project> = match self.filter {
			Some(ProjectFilter::Active) => {
				projects.into_iter().filter(|p| matches!(p.status, ProjectStatus::Active)).collect()
			}
			Some(ProjectFilter::Inactive) => projects
				.into_iter()
				.filter(|p| matches!(p.status, ProjectStatus::Inactive))
				.collect(),
			Some(ProjectFilter::All) | None => projects,
		};

		if filtered_projects.is_empty() {
			self.output_writer.write_line(&"No projects found")?;
			return Ok(());
		}

		self.output_writer.write_line(&format!("{} project(s) found:", filtered_projects.len()))?;

		for project in filtered_projects {
			let status_display = match &project.status {
				ProjectStatus::Active => "[Active]",
				ProjectStatus::Inactive => "[Inactive]",
			};

			self.output_writer.write_line(&format!(
				"  {} {} - {}",
				status_display, project.name, project.description
			))?;
		}

		Ok(())
	}
}
