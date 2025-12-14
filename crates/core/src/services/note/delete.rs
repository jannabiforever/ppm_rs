use std::sync::Arc;

use crate::errors::PPMResult;
use crate::models::NoteId;
use crate::output::OutputWriter;
use crate::repositories::note::NoteRepository;
use crate::services::Service;

pub struct DeleteNote {
	pub note_repository: Arc<dyn NoteRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
	pub note_id: NoteId,
}

impl Service for DeleteNote {
	fn run(&self) -> PPMResult<()> {
		self.note_repository.delete_note(&self.note_id)?;
		self.output_writer.write_line(&format!("Note {} deleted", self.note_id))?;

		Ok(())
	}
}
