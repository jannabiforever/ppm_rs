pub mod end_focus_session;
pub mod start_focus_session;

use crate::errors::PPMResult;

/// Core abstraction for all business logic operations.
///
/// Services are created by CommandHandlers and executed in main.rs.
/// They encapsulate business logic and orchestrate dependencies.
pub trait Service {
	type Output;

	fn run(self) -> PPMResult<Self::Output>;
}
