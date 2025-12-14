pub mod note;
pub mod project;
pub mod session;
pub mod task;

use crate::errors::PPMResult;

/// Core abstraction for all business logic operations.
///
/// Services are created by CommandHandlers and executed in main.rs.
/// They encapsulate business logic and orchestrate dependencies.
pub trait Service {
	fn run(&self) -> PPMResult<()>;
}
