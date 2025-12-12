use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::services::end_focus_session::EndFocusSession;

use crate::commands::CommandHandler;

#[derive(Args, Debug, Default)]
pub struct EndCommand;

impl CommandHandler for EndCommand {
	type Service = EndFocusSession;

	fn build_service(self, context: PPMContext) -> Self::Service {
		EndFocusSession {
			clock: context.clock.clone(),
			repository: context.session_repository.clone(),
			output_writer: context.output_writer.clone(),
		}
	}
}
