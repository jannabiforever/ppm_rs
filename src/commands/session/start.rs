use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::models::ProjectName;
use ppm_core::services::Service;
use ppm_core::services::session::StartFocusSession;

use crate::commands::CommandHandler;

#[derive(Args, Debug)]
pub struct StartCommand {
	#[arg(long, short)]
	pub duration: Option<u32>,

	#[arg(long, short)]
	pub associated_project_name: Option<ProjectName>,
}

impl CommandHandler for StartCommand {
	fn build_service(self, context: PPMContext) -> Box<dyn Service> {
		Box::new(StartFocusSession {
			clock: context.clock.clone(),
			repository: context.session_repository.clone(),
			output_writer: context.output_writer.clone(),
			duration_in_minutes: self
				.duration
				.unwrap_or(context.config.default_focus_duration_in_minutes),
			associated_project_name: self.associated_project_name,
		})
	}
}
