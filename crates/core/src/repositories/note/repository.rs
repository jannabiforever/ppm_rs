use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Utc};

use crate::errors::{PPMError, PPMResult};
use crate::models::{Note, NoteId, ProjectName};

/// Data access abstraction for notes.
pub trait NoteRepository: Send + Sync {
	fn create_note(&self, note: Note) -> PPMResult<()>;
	fn get_note(&self, note_id: &NoteId) -> PPMResult<Option<Note>>;
	fn list_notes(&self) -> PPMResult<Vec<Note>>;
	fn list_notes_by_project(&self, project_name: &ProjectName) -> PPMResult<Vec<Note>>;
	fn delete_note(&self, note_id: &NoteId) -> PPMResult<()>;
}

// --------------------------------------------------------------------------------
// Concrete Implementations
// --------------------------------------------------------------------------------

/// Markdown-based note repository storing each note as a separate file.
///
/// Each note is stored as `{notes_dir}/{note_id}.md` with YAML front matter:
/// ```markdown
/// ---
/// id: note_1234567890
/// project: my-project
/// created_at: 2025-12-13T10:30:00Z
/// ---
///
/// # Note content in markdown
/// ```
pub struct LocalNoteRepository {
	notes_dir: PathBuf,
}

impl LocalNoteRepository {
	pub fn new(notes_dir: PathBuf) -> Self {
		Self {
			notes_dir,
		}
	}

	fn ensure_notes_dir(&self) -> PPMResult<()> {
		fs::create_dir_all(&self.notes_dir)?;
		Ok(())
	}

	fn note_file_path(&self, note_id: &NoteId) -> PathBuf {
		self.notes_dir.join(format!("{}.md", note_id.as_ref()))
	}

	fn parse_note_file(&self, content: &str) -> PPMResult<Note> {
		// Split front matter and body
		let parts: Vec<&str> = content.splitn(3, "---").collect();

		if parts.len() < 3 {
			return Err(PPMError::IoError(std::io::Error::other(
				"Invalid note format: missing front matter",
			)));
		}

		let front_matter = parts[1].trim();
		let body = parts[2].trim();

		// Parse YAML front matter manually (simple key: value format)
		let mut id: Option<NoteId> = None;
		let mut project: Option<ProjectName> = None;
		let mut created_at: Option<DateTime<Utc>> = None;

		for line in front_matter.lines() {
			let line = line.trim();
			if let Some((key, value)) = line.split_once(':') {
				let key = key.trim();
				let value = value.trim();

				match key {
					"id" => id = Some(NoteId::from(value)),
					"project" => {
						if !value.is_empty() {
							project = Some(ProjectName::from(value));
						}
					}
					"created_at" => {
						created_at = Some(
							DateTime::parse_from_rfc3339(value)
								.map_err(|e| {
									PPMError::IoError(std::io::Error::other(format!(
										"Failed to parse date: {}",
										e
									)))
								})?
								.with_timezone(&Utc),
						);
					}
					_ => {} // Ignore unknown fields
				}
			}
		}

		let id = id.ok_or_else(|| {
			PPMError::IoError(std::io::Error::other("Missing 'id' in front matter"))
		})?;
		let created_at = created_at.ok_or_else(|| {
			PPMError::IoError(std::io::Error::other("Missing 'created_at' in front matter"))
		})?;

		Ok(Note {
			id,
			project_name: project,
			content: body.to_string(),
			created_at,
		})
	}

	fn format_note_file(&self, note: &Note) -> String {
		let mut front_matter =
			format!("---\nid: {}\ncreated_at: {}\n", note.id, note.created_at.to_rfc3339());

		if let Some(ref project) = note.project_name {
			front_matter.push_str(&format!("project: {}\n", project));
		}

		front_matter.push_str("---\n\n");
		front_matter.push_str(&note.content);

		front_matter
	}
}

impl NoteRepository for LocalNoteRepository {
	fn create_note(&self, note: Note) -> PPMResult<()> {
		self.ensure_notes_dir()?;

		let file_path = self.note_file_path(&note.id);
		let content = self.format_note_file(&note);

		fs::write(&file_path, content)?;
		Ok(())
	}

	fn get_note(&self, note_id: &NoteId) -> PPMResult<Option<Note>> {
		let file_path = self.note_file_path(note_id);

		if !file_path.exists() {
			return Ok(None);
		}

		let content = fs::read_to_string(&file_path)?;
		let note = self.parse_note_file(&content)?;
		Ok(Some(note))
	}

	fn list_notes(&self) -> PPMResult<Vec<Note>> {
		if !self.notes_dir.exists() {
			return Ok(Vec::new());
		}

		let mut notes = Vec::new();

		for entry in fs::read_dir(&self.notes_dir)? {
			let entry = entry?;
			let path = entry.path();

			if path.extension().and_then(|s| s.to_str()) == Some("md") {
				let content = fs::read_to_string(&path)?;
				if let Ok(note) = self.parse_note_file(&content) {
					notes.push(note);
				}
			}
		}

		// Sort by created_at descending (newest first)
		notes.sort_by(|a, b| b.created_at.cmp(&a.created_at));

		Ok(notes)
	}

	fn list_notes_by_project(&self, project_name: &ProjectName) -> PPMResult<Vec<Note>> {
		let notes = self.list_notes()?;
		Ok(notes.into_iter().filter(|n| n.project_name.as_ref() == Some(project_name)).collect())
	}

	fn delete_note(&self, note_id: &NoteId) -> PPMResult<()> {
		let file_path = self.note_file_path(note_id);

		if !file_path.exists() {
			return Err(PPMError::NotFound(format!("Note {} not found", note_id)));
		}

		fs::remove_file(&file_path)?;
		Ok(())
	}
}
