use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::models::ProjectName;
use ppm_core::services::Service;
use ppm_core::services::task::CreateTask;

use crate::commands::CommandHandler;

#[derive(Args, Debug)]
pub struct NewCommand {
	/// Task description
	pub description: String,

	/// Project name
	#[arg(long, short)]
	pub project: Option<ProjectName>,
}

impl CommandHandler for NewCommand {
	fn build_service(self, context: PPMContext) -> Box<dyn Service> {
		Box::new(CreateTask {
			clock: context.clock.clone(),
			task_repository: context.task_repository.clone(),
			output_writer: context.output_writer.clone(),
			project_name: self.project,
			description: self.description,
		})
	}
}
