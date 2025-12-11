use crate::errors::PPMResult;
use crate::models::FocusSession;

pub trait SessionRepository: Send + Sync {
	fn get_active_session(&self) -> PPMResult<Option<FocusSession>>;
	fn create_session(&self, session: FocusSession) -> PPMResult<()>;
	fn end_session(&self, session_id: &str) -> PPMResult<()>;
}

// --------------------------------------------------------------------------------
// Concrete Implementations
// --------------------------------------------------------------------------------

pub struct LocalSessionRepository {
	// TODO: 실제로는 파일 시스템이나 SQLite를 사용
	// 지금은 메모리 내 저장소로 시뮬레이션
}

impl LocalSessionRepository {
	pub fn new() -> Self {
		Self {}
	}
}

impl Default for LocalSessionRepository {
	fn default() -> Self {
		Self::new()
	}
}

impl SessionRepository for LocalSessionRepository {
	fn get_active_session(&self) -> PPMResult<Option<FocusSession>> {
		// TODO: 실제 구현에서는 파일이나 DB에서 읽기
		Ok(None)
	}

	fn create_session(&self, _session: FocusSession) -> PPMResult<()> {
		// TODO: 실제 구현에서는 파일이나 DB에 저장
		Ok(())
	}

	fn end_session(&self, _session_id: &str) -> PPMResult<()> {
		// TODO: 실제 구현에서는 세션 종료 처리
		Ok(())
	}
}
