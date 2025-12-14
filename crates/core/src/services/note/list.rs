use std::sync::Arc;

use crate::errors::PPMResult;
use crate::output::OutputWriter;
use crate::repositories::note::NoteRepository;
use crate::services::Service;

pub struct ListNotes {
	pub note_repository: Arc<dyn NoteRepository>,
	pub output_writer: Arc<dyn OutputWriter>,
	pub limit: Option<usize>,
}

impl Service for ListNotes {
	fn run(&self) -> PPMResult<()> {
		let mut notes = self.note_repository.list_notes()?;

		// Sort by created_at descending (newest first)
		notes.sort_by(|a, b| b.created_at.cmp(&a.created_at));

		// Apply limit if specified
		if let Some(limit) = self.limit {
			notes.truncate(limit);
		}

		if notes.is_empty() {
			self.output_writer.write_line(&"No notes found")?;
			return Ok(());
		}

		self.output_writer.write_line(&format!("{} note(s) found:", notes.len()))?;

		for note in notes {
			let project_display = match note.project_name {
				Some(ref project) => format!(" ({})", project),
				None => String::new(),
			};

			// Truncate content for list view (first 50 chars)
			let content_preview = if note.content.len() > 50 {
				format!("{}...", &note.content[..50])
			} else {
				note.content.clone()
			};

			self.output_writer
				.write_line(&format!("  {} - {}{}", note.id, content_preview, project_display))?;
		}

		Ok(())
	}
}
