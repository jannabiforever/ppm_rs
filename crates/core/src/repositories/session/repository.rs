use std::fs::{self, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use chrono::{DateTime, Utc};

use crate::errors::{PPMError, PPMResult};
use crate::models::{FocusSession, FocusSessionId};

/// Data access abstraction for focus sessions.
///
/// Note: Methods take `current_time` as parameter (dependency inversion).
/// This avoids Repository depending on Clock - time comes from the service layer.
pub trait SessionRepository: Send + Sync {
	fn get_active_session(&self, current_time: DateTime<Utc>) -> PPMResult<Option<FocusSession>>;
	fn create_session(&self, session: FocusSession) -> PPMResult<()>;
	fn end_session(
		&self,
		session_id: &FocusSessionId,
		current_time: DateTime<Utc>,
	) -> PPMResult<()>;
	fn delete_session(&self, session_id: &FocusSessionId) -> PPMResult<()>;
	fn list_sessions(&self) -> PPMResult<Vec<FocusSession>>;
}

// --------------------------------------------------------------------------------
// Concrete Implementations
// --------------------------------------------------------------------------------

/// File-based session repository storing data in JSON format.
pub struct LocalSessionRepository {
	storage_path: PathBuf,
}

impl LocalSessionRepository {
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

	fn load_sessions(&self) -> PPMResult<Vec<FocusSession>> {
		if !self.storage_path.exists() {
			return Ok(Vec::new());
		}

		let file = fs::File::open(&self.storage_path)?;
		let reader = BufReader::new(file);
		let sessions: Vec<FocusSession> = serde_json::from_reader(reader)
			.map_err(|e| std::io::Error::other(format!("Failed to parse sessions: {}", e)))?;

		Ok(sessions)
	}

	fn save_sessions(&self, sessions: &[FocusSession]) -> PPMResult<()> {
		self.ensure_storage_dir()?;

		let file =
			OpenOptions::new().write(true).create(true).truncate(true).open(&self.storage_path)?;

		let mut writer = BufWriter::new(file);
		serde_json::to_writer_pretty(&mut writer, sessions)
			.map_err(|e| std::io::Error::other(format!("Failed to write sessions: {}", e)))?;

		writer.flush()?;

		Ok(())
	}
}

impl SessionRepository for LocalSessionRepository {
	fn get_active_session(&self, current_time: DateTime<Utc>) -> PPMResult<Option<FocusSession>> {
		let sessions = self.load_sessions()?;
		Ok(sessions.into_iter().find(|s| s.is_active(current_time)))
	}

	fn create_session(&self, session: FocusSession) -> PPMResult<()> {
		let mut sessions = self.load_sessions()?;
		sessions.push(session);
		self.save_sessions(&sessions)?;
		Ok(())
	}

	fn end_session(
		&self,
		session_id: &FocusSessionId,
		current_time: DateTime<Utc>,
	) -> PPMResult<()> {
		let mut sessions = self.load_sessions()?;

		if let Some(session) = sessions.iter_mut().find(|s| &s.id == session_id) {
			// Update end time to now
			session.end = current_time;
			self.save_sessions(&sessions)?;
			Ok(())
		} else {
			Err(PPMError::NoActiveSession)
		}
	}

	fn delete_session(&self, session_id: &FocusSessionId) -> PPMResult<()> {
		let mut sessions = self.load_sessions()?;
		let initial_len = sessions.len();

		sessions.retain(|s| &s.id != session_id);

		if sessions.len() == initial_len {
			return Err(PPMError::NoActiveSession);
		}

		self.save_sessions(&sessions)?;
		Ok(())
	}

	fn list_sessions(&self) -> PPMResult<Vec<FocusSession>> {
		self.load_sessions()
	}
}
