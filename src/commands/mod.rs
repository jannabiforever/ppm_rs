pub mod end;
pub mod start;

use ppm_core::context::PPMContext;
use ppm_core::services::Service;

pub trait CommandHandler {
	type Service: Service<Output = ()>;

	fn build_service(self, context: PPMContext) -> Self::Service;
}
