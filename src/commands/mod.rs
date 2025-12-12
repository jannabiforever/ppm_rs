pub mod note;
pub mod session;
pub mod task;

use ppm_core::context::PPMContext;
use ppm_core::services::Service;

/// Command layer pattern - builds services from context.
///
/// Commands act as factories: they receive PPMContext and construct
/// the appropriate Service with dependencies. Execution happens in main.rs.
pub trait CommandHandler {
	type Service: Service<Output = ()>;

	fn build_service(self, context: PPMContext) -> Self::Service;
}
