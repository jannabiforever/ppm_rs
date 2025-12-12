use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::models::ProjectName;
use ppm_core::services::task::CreateTask;

use crate::commands::CommandHandler;

#[derive(Args, Debug)]
pub struct AddCommand {
	/// Task description
	pub description: String,

	/// Project name
	#[arg(long, short)]
	pub project: ProjectName,
}

impl CommandHandler for AddCommand {
	type Service = CreateTask;

	fn build_service(self, context: PPMContext) -> Self::Service {
		CreateTask {
			clock: context.clock.clone(),
			task_repository: context.task_repository.clone(),
			output_writer: context.output_writer.clone(),
			project_name: self.project,
			description: self.description,
		}
	}
}
