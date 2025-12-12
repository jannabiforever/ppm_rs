use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::services::session::GetSessionStats;

use crate::commands::CommandHandler;

#[derive(Args, Debug, Default)]
pub struct StatsCommand {}

impl StatsCommand {
	pub fn new() -> Self {
		Self {}
	}
}

impl CommandHandler for StatsCommand {
	type Service = GetSessionStats;

	fn build_service(self, context: PPMContext) -> Self::Service {
		GetSessionStats {
			clock: context.clock.clone(),
			repository: context.session_repository.clone(),
			output_writer: context.output_writer.clone(),
		}
	}
}
