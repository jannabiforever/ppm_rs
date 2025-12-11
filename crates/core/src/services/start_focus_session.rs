use crate::{errors::PPMResult, services::Service};

pub trait StartFocusSessionService {
    fn ensure_no_active_focus_session(&self) -> PPMResult<()>;
    fn create_new_focus_session(&self) -> PPMResult<()>;
}

impl<S: StartFocusSessionService> Service for S {
    type Output = ();

    fn run(self) -> PPMResult<Self::Output> {
        self.ensure_no_active_focus_session()
            .and_then(|_| self.create_new_focus_session())
    }
}

// --------------------------------------------------------------------------------
// Concrete Implementations
// --------------------------------------------------------------------------------

pub struct LocallyStartFocusSession {}

impl StartFocusSessionService for LocallyStartFocusSession {
    fn ensure_no_active_focus_session(&self) -> PPMResult<()> {
        Ok(())
    }

    fn create_new_focus_session(&self) -> PPMResult<()> {
        Ok(())
    }
}
