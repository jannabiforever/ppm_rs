use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::models::NoteId;
use ppm_core::services::note::DeleteNote;

use crate::commands::CommandHandler;

#[derive(Args, Debug)]
pub struct DeleteCommand {
	/// Note ID to delete
	pub note_id: NoteId,
}

impl CommandHandler for DeleteCommand {
	type Service = DeleteNote;

	fn build_service(self, context: PPMContext) -> Self::Service {
		DeleteNote {
			note_repository: context.note_repository.clone(),
			output_writer: context.output_writer.clone(),
			note_id: self.note_id,
		}
	}
}
