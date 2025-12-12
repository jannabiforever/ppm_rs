use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::services::start_focus_session::StartFocusSession;

use crate::commands::CommandHandler;

#[derive(Args, Debug)]
pub struct StartCommand {
	pub duration: Option<u32>,
}

impl CommandHandler for StartCommand {
	type Service = StartFocusSession;

	fn build_service(self, context: PPMContext) -> Self::Service {
		StartFocusSession {
			clock: context.clock.clone(),
			repository: context.session_repository.clone(),
			output_writer: context.output_writer.clone(),
			duration_in_minutes: self
				.duration
				.unwrap_or(context.config.default_focus_duration_in_minutes),
		}
	}
}
