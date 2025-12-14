use std::sync::Arc;

use crate::clock::Clock;
use crate::editor::Editor;
use crate::errors::PPMResult;
use crate::models::{Note, NoteId, ProjectName};
use crate::output::OutputWriter;
use crate::repositories::note::NoteRepository;
use crate::repositories::session::SessionRepository;
use crate::services::Service;

pub struct CreateNote {
	pub clock: Arc<dyn Clock>,
	pub note_repository: Arc<dyn NoteRepository>,
	pub session_repository: Arc<dyn SessionRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
	pub editor: Arc<dyn Editor>,
	pub project_name: Option<ProjectName>,
}

impl CreateNote {
	/// Extracts the note body from full content (removes front matter)
	fn extract_note_body(&self, full_content: &str) -> String {
		// Split by --- delimiters
		let parts: Vec<&str> = full_content.splitn(3, "---").collect();

		if parts.len() >= 3 {
			// Front matter exists, return body part
			parts[2].trim().to_string()
		} else {
			// No front matter found, return as-is
			full_content.trim().to_string()
		}
	}
}

impl Service for CreateNote {
	type Output = ();

	fn run(self) -> PPMResult<()> {
		let current_time = self.clock.now()?;
		let note_id = NoteId::new();

		// Determine project name: use provided, or fetch from active session
		let project_name = if let Some(ref name) = self.project_name {
			Some(name.clone())
		} else {
			// Try to get project from active session
			self.session_repository
				.get_active_session(current_time)?
				.and_then(|session| session.associated_project_name)
		};

		// Prepare initial content with front matter template
		let mut initial_content =
			format!("---\nid: {}\ncreated_at: {}\n", note_id, current_time.to_rfc3339());

		if let Some(ref project) = project_name {
			initial_content.push_str(&format!("project: {}\n", project));
		}

		initial_content.push_str("---\n\n# Write your note here\n\n");

		// Open editor with front matter template
		let full_content = self.editor.open(Some(&initial_content))?;

		// If user didn't write anything, abort
		let full_content = match full_content {
			Some(c) => c,
			None => {
				self.output_writer.write_line(&"Note creation cancelled (no content provided)")?;
				return Ok(());
			}
		};

		// Parse the edited content to extract body (skip front matter)
		let content = self.extract_note_body(&full_content);

		let note = Note {
			id: note_id,
			project_name: project_name.clone(),
			content,
			created_at: current_time,
		};

		self.note_repository.create_note(note)?;

		let message = match project_name {
			Some(ref project) => format!("Note created for project '{}'", project),
			None => "Note created".to_string(),
		};

		self.output_writer.write_line(&message)?;

		Ok(())
	}
}
