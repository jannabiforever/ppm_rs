use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::models::ProjectName;
use ppm_core::services::note::CreateNote;

use crate::commands::CommandHandler;

#[derive(Args, Debug)]
pub struct NewCommand {
	/// Optional project name to associate with (if not provided, uses active session's project)
	#[arg(long, short)]
	pub project: Option<ProjectName>,
}

impl CommandHandler for NewCommand {
	type Service = CreateNote;

	fn build_service(self, context: PPMContext) -> Self::Service {
		CreateNote {
			clock: context.clock.clone(),
			note_repository: context.note_repository.clone(),
			session_repository: context.session_repository.clone(),
			output_writer: context.output_writer.clone(),
			editor: context.editor.clone(),
			associated_project_name: self.project,
		}
	}
}
