use clap::Args;
use ppm_core::models::ProjectName;
use ppm_core::services::project::CreateProject;

use crate::commands::CommandHandler;

#[derive(Args, Debug)]
pub struct NewCommand {
	/// Project name
	pub name: String,

	/// Project description
	#[arg(short, long, default_value = "")]
	pub description: String,
}

impl NewCommand {
	pub fn new(name: String, description: String) -> Self {
		Self {
			name,
			description,
		}
	}
}

impl CommandHandler for NewCommand {
	fn build_service(
		self,
		context: ppm_core::context::PPMContext,
	) -> Box<dyn ppm_core::services::Service> {
		Box::new(CreateProject {
			clock: context.clock.clone(),
			project_repository: context.project_repository.clone(),
			output_writer: context.output_writer.clone(),
			name: ProjectName::from(self.name),
			description: self.description,
		})
	}
}
