use std::sync::{Arc, Mutex};

use crate::errors::{PPMError, PPMResult};
use crate::models::{Note, NoteId, ProjectName};
use crate::repositories::note::NoteRepository;

/// In-memory note repository for testing
pub struct InMemoryNoteRepository {
	notes: Arc<Mutex<Vec<Note>>>,
}

impl InMemoryNoteRepository {
	pub fn new() -> Self {
		Self {
			notes: Arc::new(Mutex::new(Vec::new())),
		}
	}

	pub fn get_all_notes(&self) -> Vec<Note> {
		self.notes.lock().unwrap().clone()
	}
}

impl Default for InMemoryNoteRepository {
	fn default() -> Self {
		Self::new()
	}
}

impl NoteRepository for InMemoryNoteRepository {
	fn create_note(&self, note: Note) -> PPMResult<()> {
		let mut notes = self.notes.lock().unwrap();
		notes.push(note);
		Ok(())
	}

	fn get_note(&self, note_id: &NoteId) -> PPMResult<Option<Note>> {
		let notes = self.notes.lock().unwrap();
		Ok(notes.iter().find(|n| &n.id == note_id).cloned())
	}

	fn update_note_content(&self, note_id: &NoteId, content: String) -> PPMResult<()> {
		let mut notes = self.notes.lock().unwrap();

		if let Some(note) = notes.iter_mut().find(|n| &n.id == note_id) {
			note.content = content;
			Ok(())
		} else {
			Err(PPMError::NotFound(format!("Note {} not found", note_id)))
		}
	}

	fn list_notes(&self) -> PPMResult<Vec<Note>> {
		Ok(self.notes.lock().unwrap().clone())
	}

	fn list_notes_by_project(&self, project_name: &ProjectName) -> PPMResult<Vec<Note>> {
		let notes = self.notes.lock().unwrap();
		Ok(notes
			.iter()
			.filter(|n| n.associated_project_name.as_ref() == Some(project_name))
			.cloned()
			.collect())
	}

	fn delete_note(&self, note_id: &NoteId) -> PPMResult<()> {
		let mut notes = self.notes.lock().unwrap();
		let initial_len = notes.len();

		notes.retain(|n| &n.id != note_id);

		if notes.len() == initial_len {
			return Err(PPMError::NotFound(format!("Note {} not found", note_id)));
		}

		Ok(())
	}
}
