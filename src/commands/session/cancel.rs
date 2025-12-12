use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::services::session::CancelFocusSession;

use crate::commands::CommandHandler;

#[derive(Args, Debug, Default)]
pub struct CancelCommand {}

impl CancelCommand {
	pub fn new() -> Self {
		Default::default()
	}
}

impl CommandHandler for CancelCommand {
	type Service = CancelFocusSession;

	fn build_service(self, context: PPMContext) -> Self::Service {
		CancelFocusSession {
			clock: context.clock.clone(),
			repository: context.session_repository.clone(),
			output_writer: context.output_writer.clone(),
		}
	}
}
