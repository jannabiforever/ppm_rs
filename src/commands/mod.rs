pub mod start;

use ppm_core::context::PPMContext;

use crate::errors::PPMCliError;

pub trait CommandHandler {
	fn execute(self, context: PPMContext) -> Result<(), PPMCliError>;
}
