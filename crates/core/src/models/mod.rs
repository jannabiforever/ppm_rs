use chrono::{DateTime, Duration, Utc};
use model_macros::{model, model_id, model_name};

pub fn gen_id() -> String {
	use std::time::{SystemTime, UNIX_EPOCH};
	let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
	format!("{}", timestamp)
}

#[model_id(prefix = "session_", gen = crate::models::gen_id)]
pub struct FocusSessionId(pub String);

#[model_name]
pub struct ProjectName(pub String);

#[model]
pub struct Project {
	pub name: ProjectName,
	pub description: String,
	pub created_at: DateTime<Utc>,
	pub status: ProjectStatus,
}

#[model]
pub enum ProjectStatus {
	Active,
	Inactive,
}

#[model]
pub struct FocusSession {
	pub id: FocusSessionId,
	pub associated_project_name: Option<ProjectName>,
	pub start: DateTime<Utc>,
	pub end: DateTime<Utc>,
}

impl FocusSession {
	pub fn duration(&self) -> Duration {
		self.end - self.start
	}

	pub fn is_active(&self, now: DateTime<Utc>) -> bool {
		now >= self.start && now <= self.end
	}
}

#[model_id(prefix = "task_", gen = crate::models::gen_id)]
pub struct TaskId(pub String);

#[model]
pub struct Task {
	pub id: TaskId,
	pub project_name: Option<ProjectName>,
	pub description: String,
	pub status: TaskStatus,
	pub created_at: DateTime<Utc>,
}

#[model]
pub enum TaskStatus {
	Pending,
	Done(DateTime<Utc>),
	Canceled(DateTime<Utc>),
}

#[model_id(prefix = "note_", gen = crate::models::gen_id)]
pub struct NoteId(pub String);

#[model]
pub struct Note {
	pub id: NoteId,
	pub project_name: Option<ProjectName>,
	pub content: String,
	pub created_at: DateTime<Utc>,
}
