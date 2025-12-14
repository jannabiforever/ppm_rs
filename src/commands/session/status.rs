use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::services::Service;
use ppm_core::services::session::GetSessionStatus;

use crate::commands::CommandHandler;

#[derive(Args, Debug, Default)]
pub struct StatusCommand {}

impl StatusCommand {
	pub fn new() -> Self {
		Self {}
	}
}

impl CommandHandler for StatusCommand {
	fn build_service(self, context: PPMContext) -> Box<dyn Service> {
		Box::new(GetSessionStatus {
			clock: context.clock.clone(),
			repository: context.session_repository.clone(),
			output_writer: context.output_writer.clone(),
		})
	}
}
