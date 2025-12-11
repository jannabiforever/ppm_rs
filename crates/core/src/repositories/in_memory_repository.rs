use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};

use crate::errors::{PPMError, PPMResult};
use crate::models::FocusSession;
use crate::repositories::SessionRepository;

/// In-memory session repository for testing
pub struct InMemorySessionRepository {
	sessions: Arc<Mutex<Vec<FocusSession>>>,
}

impl InMemorySessionRepository {
	pub fn new() -> Self {
		Self {
			sessions: Arc::new(Mutex::new(Vec::new())),
		}
	}

	pub fn get_all_sessions(&self) -> Vec<FocusSession> {
		self.sessions.lock().unwrap().clone()
	}
}

impl Default for InMemorySessionRepository {
	fn default() -> Self {
		Self::new()
	}
}

impl SessionRepository for InMemorySessionRepository {
	fn get_active_session(&self, current_time: DateTime<Utc>) -> PPMResult<Option<FocusSession>> {
		let sessions = self.sessions.lock().unwrap();
		Ok(sessions.iter().find(|s| s.is_active(current_time)).cloned())
	}

	fn create_session(&self, session: FocusSession) -> PPMResult<()> {
		let mut sessions = self.sessions.lock().unwrap();
		sessions.push(session);
		Ok(())
	}

	fn end_session(&self, session_id: &str, current_time: DateTime<Utc>) -> PPMResult<()> {
		let mut sessions = self.sessions.lock().unwrap();

		if let Some(session) = sessions.iter_mut().find(|s| s.id == session_id) {
			session.end = current_time;
			Ok(())
		} else {
			Err(PPMError::NoActiveSession)
		}
	}

	fn list_sessions(&self) -> PPMResult<Vec<FocusSession>> {
		Ok(self.sessions.lock().unwrap().clone())
	}
}
