use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::models::ProjectName;
use ppm_core::services::Service;
use ppm_core::services::note::CreateNote;

use crate::commands::CommandHandler;

#[derive(Args, Debug)]
pub struct NewCommand {
	/// Optional project name to associate with (if not provided, uses active session's project)
	#[arg(long, short)]
	pub project_name: Option<ProjectName>,
}

impl CommandHandler for NewCommand {
	fn build_service(self, context: PPMContext) -> Box<dyn Service> {
		Box::new(CreateNote {
			clock: context.clock.clone(),
			note_repository: context.note_repository.clone(),
			session_repository: context.session_repository.clone(),
			output_writer: context.output_writer.clone(),
			editor: context.editor.clone(),
			project_name: self.project_name,
		})
	}
}
