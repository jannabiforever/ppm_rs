use std::fs::{self, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use crate::errors::{PPMError, PPMResult};
use crate::models::{Note, NoteId, ProjectName};

/// Data access abstraction for notes.
pub trait NoteRepository: Send + Sync {
	fn create_note(&self, note: Note) -> PPMResult<()>;
	fn get_note(&self, note_id: &NoteId) -> PPMResult<Option<Note>>;
	fn update_note_content(&self, note_id: &NoteId, content: String) -> PPMResult<()>;
	fn list_notes(&self) -> PPMResult<Vec<Note>>;
	fn list_notes_by_project(&self, project_name: &ProjectName) -> PPMResult<Vec<Note>>;
	fn delete_note(&self, note_id: &NoteId) -> PPMResult<()>;
}

// --------------------------------------------------------------------------------
// Concrete Implementations
// --------------------------------------------------------------------------------

/// File-based note repository storing data in JSON format.
pub struct LocalNoteRepository {
	storage_path: PathBuf,
}

impl LocalNoteRepository {
	pub fn new(storage_path: PathBuf) -> Self {
		Self {
			storage_path,
		}
	}

	fn ensure_storage_dir(&self) -> PPMResult<()> {
		if let Some(parent) = self.storage_path.parent() {
			fs::create_dir_all(parent)?;
		}
		Ok(())
	}

	fn load_notes(&self) -> PPMResult<Vec<Note>> {
		if !self.storage_path.exists() {
			return Ok(Vec::new());
		}

		let file = fs::File::open(&self.storage_path)?;
		let reader = BufReader::new(file);
		let notes: Vec<Note> = serde_json::from_reader(reader)
			.map_err(|e| std::io::Error::other(format!("Failed to parse notes: {}", e)))?;

		Ok(notes)
	}

	fn save_notes(&self, notes: &[Note]) -> PPMResult<()> {
		self.ensure_storage_dir()?;

		let file =
			OpenOptions::new().write(true).create(true).truncate(true).open(&self.storage_path)?;

		let mut writer = BufWriter::new(file);
		serde_json::to_writer_pretty(&mut writer, notes)
			.map_err(|e| std::io::Error::other(format!("Failed to write notes: {}", e)))?;

		writer.flush()?;

		Ok(())
	}
}

impl NoteRepository for LocalNoteRepository {
	fn create_note(&self, note: Note) -> PPMResult<()> {
		let mut notes = self.load_notes()?;
		notes.push(note);
		self.save_notes(&notes)?;
		Ok(())
	}

	fn get_note(&self, note_id: &NoteId) -> PPMResult<Option<Note>> {
		let notes = self.load_notes()?;
		Ok(notes.into_iter().find(|n| &n.id == note_id))
	}

	fn update_note_content(&self, note_id: &NoteId, content: String) -> PPMResult<()> {
		let mut notes = self.load_notes()?;

		if let Some(note) = notes.iter_mut().find(|n| &n.id == note_id) {
			note.content = content;
			self.save_notes(&notes)?;
			Ok(())
		} else {
			Err(PPMError::NotFound(format!("Note {} not found", note_id)))
		}
	}

	fn list_notes(&self) -> PPMResult<Vec<Note>> {
		self.load_notes()
	}

	fn list_notes_by_project(&self, project_name: &ProjectName) -> PPMResult<Vec<Note>> {
		let notes = self.load_notes()?;
		Ok(notes
			.into_iter()
			.filter(|n| n.associated_project_name.as_ref() == Some(project_name))
			.collect())
	}

	fn delete_note(&self, note_id: &NoteId) -> PPMResult<()> {
		let mut notes = self.load_notes()?;
		let initial_len = notes.len();

		notes.retain(|n| &n.id != note_id);

		if notes.len() == initial_len {
			return Err(PPMError::NotFound(format!("Note {} not found", note_id)));
		}

		self.save_notes(&notes)?;
		Ok(())
	}
}
