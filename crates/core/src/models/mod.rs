use std::fmt;

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FocusSessionId(pub String);

impl FocusSessionId {
	pub fn new() -> Self {
		use std::time::{SystemTime, UNIX_EPOCH};
		let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
		FocusSessionId(format!("session_{}", timestamp))
	}
}

impl Default for FocusSessionId {
	fn default() -> Self {
		Self::new()
	}
}

impl fmt::Display for FocusSessionId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl From<String> for FocusSessionId {
	fn from(value: String) -> Self {
		Self(value)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectName(pub String);

impl fmt::Display for ProjectName {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl From<String> for ProjectName {
	fn from(value: String) -> Self {
		Self(value)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
	pub name: ProjectName,
	pub description: String,
	pub is_active: bool,
	pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
	pub id: String,
	pub project_name: ProjectName,
	pub description: String,
	pub status: TaskStatus,
	pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
	Pending,
	/// Done with a timestamp
	Done(DateTime<Utc>),
	/// Canceled with a timestamp
	Canceled(DateTime<Utc>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
	pub id: String,
	pub associated_project_name: Option<ProjectName>,
	pub content: String,
	pub created_at: DateTime<Utc>,
}
