use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::services::note::ListNotes;

use crate::commands::CommandHandler;

#[derive(Args, Debug)]
pub struct ListCommand {
	/// Limit the number of notes to display
	#[arg(short, long)]
	pub limit: Option<usize>,
}

impl CommandHandler for ListCommand {
	type Service = ListNotes;

	fn build_service(self, context: PPMContext) -> Self::Service {
		ListNotes {
			note_repository: context.note_repository.clone(),
			output_writer: context.output_writer.clone(),
			limit: self.limit,
		}
	}
}
