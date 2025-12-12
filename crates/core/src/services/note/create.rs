use std::sync::Arc;

use crate::clock::Clock;
use crate::errors::PPMResult;
use crate::models::{Note, NoteId, ProjectName};
use crate::output::OutputWriter;
use crate::repositories::note::NoteRepository;
use crate::services::Service;

pub struct CreateNote {
	pub clock: Arc<dyn Clock>,
	pub note_repository: Arc<dyn NoteRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
	pub content: String,
	pub associated_project_name: Option<ProjectName>,
}

impl Service for CreateNote {
	type Output = ();

	fn run(self) -> PPMResult<()> {
		let note = Note {
			id: NoteId::new(),
			associated_project_name: self.associated_project_name.clone(),
			content: self.content.clone(),
			created_at: self.clock.now(),
		};

		self.note_repository.create_note(note)?;

		let message = match self.associated_project_name {
			Some(ref project) => format!("Note created for project '{}'", project),
			None => "Note created".to_string(),
		};

		self.output_writer.write_line(&message)?;

		Ok(())
	}
}
