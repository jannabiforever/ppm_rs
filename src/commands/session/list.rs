use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::services::Service;
use ppm_core::services::session::ListSessions;

use crate::commands::CommandHandler;

#[derive(Args, Debug)]
pub struct ListCommand {
	/// Limit the number of sessions to display
	#[arg(short, long)]
	pub limit: Option<usize>,
}

impl ListCommand {
	pub fn new(limit: Option<usize>) -> Self {
		Self {
			limit,
		}
	}
}

impl CommandHandler for ListCommand {
	fn build_service(self, context: PPMContext) -> Box<dyn Service> {
		Box::new(ListSessions {
			clock: context.clock.clone(),
			repository: context.session_repository.clone(),
			output_writer: context.output_writer.clone(),
			limit: self.limit,
		})
	}
}
