use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::models::ProjectName;
use ppm_core::services::note::CreateNote;

use crate::commands::CommandHandler;

#[derive(Args, Debug)]
pub struct NewCommand {
	/// Note content
	pub content: String,

	/// Optional project name to associate with
	#[arg(long, short)]
	pub project: Option<ProjectName>,
}

impl CommandHandler for NewCommand {
	type Service = CreateNote;

	fn build_service(self, context: PPMContext) -> Self::Service {
		CreateNote {
			clock: context.clock.clone(),
			note_repository: context.note_repository.clone(),
			output_writer: context.output_writer.clone(),
			content: self.content,
			associated_project_name: self.project,
		}
	}
}
