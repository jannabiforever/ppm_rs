use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone)]
pub struct FocusSession {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl FocusSession {
    pub fn duration(&self) -> Duration {
        self.end - self.start
    }
}
