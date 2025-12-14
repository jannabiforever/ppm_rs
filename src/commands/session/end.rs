use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::services::Service;
use ppm_core::services::session::EndFocusSession;

use crate::commands::CommandHandler;

#[derive(Args, Debug, Default)]
pub struct EndCommand;

impl CommandHandler for EndCommand {
	fn build_service(self, context: PPMContext) -> Box<dyn Service> {
		Box::new(EndFocusSession {
			clock: context.clock.clone(),
			repository: context.session_repository.clone(),
			output_writer: context.output_writer.clone(),
		})
	}
}
